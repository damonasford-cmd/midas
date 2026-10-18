
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const MAX_GOAL_BYTES: usize = 64 * 1024;
const MAX_PLAN_BYTES: usize = 4 * 1024 * 1024;
const MAX_RESULT_BYTES: usize = 4 * 1024 * 1024;
const MAX_ERROR_BYTES: usize = 16 * 1024;
const MAX_ACTOR_BYTES: usize = 256;
const MAX_APPROVAL_NOTE_BYTES: usize = 16 * 1024;
const MAX_TASK_LIST_LIMIT: usize = 1_000;
const MAX_EVENT_LIST_LIMIT: usize = 10_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Planned,
    Running,
    AwaitingApproval,
    Succeeded,
    Failed,
    Cancelled,
}

impl TaskStatus {
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeeded | Self::Cancelled)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub requested_at_ms: u64,
    pub requested_by: String,
    pub details: Value,
    pub resolved_at_ms: Option<u64>,
    pub resolved_by: Option<String>,
    pub approved: Option<bool>,
    pub resolution_note: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskRecord {
    pub id: String,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub goal: String,
    pub status: TaskStatus,
    pub plan: Option<String>,
    pub result: Option<Value>,
    pub last_error: Option<String>,
    pub attempts: u32,
    pub approval: Option<ApprovalRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskEvent {
    pub event_id: String,
    pub timestamp_ms: u64,
    pub task_id: String,
    pub event_type: String,
    pub actor: Option<String>,
    pub details: Value,
}

struct StoreInner {
    root: PathBuf,
    tasks_dir: PathBuf,
    events_path: PathBuf,
    mutation_lock: Mutex<()>,
    journal: Mutex<File>,
}

/// Persistent task state for MIDAS.
///
/// Task snapshots are atomically replaced JSON files. The event journal is
/// append-only JSONL and is synchronized after each event.
///
/// The snapshot and the event journal cannot be committed as one filesystem
/// transaction. If journal writing fails after a snapshot is saved, the
/// returned error explains that the snapshot was already committed. Callers
/// must reread the task before retrying.
///
/// This initial implementation coordinates threads sharing one TaskStore.
/// Multiple processes writing to the same directory require an additional
/// cross-process locking mechanism.
#[derive(Clone)]
pub struct TaskStore {
    inner: Arc<StoreInner>,
}

impl TaskStore {
    /// Opens or initializes the persistent task store at `root`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let tasks_dir = root.join("tasks");
        let events_path = root.join("events.jsonl");

        fs::create_dir_all(&tasks_dir)
            .with_context(|| {
                format!("Impossible de créer {}", tasks_dir.display())
            })?;

        let journal = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&events_path)
            .with_context(|| {
                format!("Impossible d'ouvrir {}", events_path.display())
            })?;

        Ok(Self {
            inner: Arc::new(StoreInner {
                root,
                tasks_dir,
                events_path,
                mutation_lock: Mutex::new(()),
                journal: Mutex::new(journal),
            }),
        })
    }

    /// Returns the configured store directory.
    pub fn root(&self) -> &Path {
        &self.inner.root
    }

    /// Creates a new task in the pending state.
    pub fn create_task(&self, goal: &str) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;
        validate_text(goal, "objectif", MAX_GOAL_BYTES)?;

        let now = now_ms()?;

        let record = TaskRecord {
            id: Uuid::new_v4().to_string(),
            created_at_ms: now,
            updated_at_ms: now,
            goal: goal.trim().to_string(),
            status: TaskStatus::Pending,
            plan: None,
            result: None,
            last_error: None,
            attempts: 0,
            approval: None,
        };

        self.commit_record(
            &record,
            "task_created",
            Some("midas".to_string()),
            json!({
                "goal_bytes": record.goal.len()
            }),
        )?;

        Ok(record)
    }

    /// Reads a task snapshot by UUID.
    pub fn get_task(&self, id: &str) -> Result<TaskRecord> {
        let path = self.task_path(id)?;

        let bytes = fs::read(&path)
            .with_context(|| {
                format!("Impossible de lire la tâche {}", id)
            })?;

        let record: TaskRecord = serde_json::from_slice(&bytes)
            .with_context(|| {
                format!("La tâche {} contient un JSON invalide", id)
            })?;

        if record.id != canonical_uuid(id)? {
            bail!(
                "L'identifiant dans le fichier ne correspond pas \
                 à la tâche demandée"
            );
        }

        Ok(record)
    }

    /// Lists tasks, most recently updated first.
    /// The limit must be between 1 and 1000.
    pub fn list_tasks(&self, limit: usize) -> Result<Vec<TaskRecord>> {
        if limit == 0 || limit > MAX_TASK_LIST_LIMIT {
            bail!(
                "La limite doit être comprise entre 1 et {}",
                MAX_TASK_LIST_LIMIT
            );
        }

        let mut records = Vec::new();

        let entries = fs::read_dir(&self.inner.tasks_dir)
            .with_context(|| {
                format!(
                    "Impossible de lister {}",
                    self.inner.tasks_dir.display()
                )
            })?;

        for entry in entries {
            let entry =
                entry.context("Impossible de lire une entrée de tâches")?;

            let file_type = entry
                .file_type()
                .context("Impossible de déterminer le type du fichier")?;

            if !file_type.is_file()
                || entry.path().extension().and_then(|ext| ext.to_str())
                    != Some("json")
            {
                continue;
            }

            let bytes = fs::read(entry.path())
                .with_context(|| {
                    format!(
                        "Impossible de lire {}",
                        entry.path().display()
                    )
                })?;

            let record: TaskRecord =
                serde_json::from_slice(&bytes).with_context(|| {
                    format!(
                        "JSON de tâche invalide dans {}",
                        entry.path().display()
                    )
                })?;

            records.push(record);
        }

        records.sort_by(|a, b| {
            b.updated_at_ms
                .cmp(&a.updated_at_ms)
                .then_with(|| b.created_at_ms.cmp(&a.created_at_ms))
        });

        records.truncate(limit);
        Ok(records)
    }

    /// Stores a plan and moves a pending task to the planned state.
    /// A plan can be revised while a task remains pending or planned.
    pub fn save_plan(&self, id: &str, plan: &str) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;
        validate_text(plan, "plan", MAX_PLAN_BYTES)?;

        let mut record = self.get_task(id)?;

        if !matches!(
            &record.status,
            TaskStatus::Pending | TaskStatus::Planned
        ) {
            bail!(
                "Un plan ne peut être modifié que pour une tâche \
                 en attente ou planifiée"
            );
        }

        record.plan = Some(plan.trim().to_string());
        record.status = TaskStatus::Planned;
        record.last_error = None;
        record.updated_at_ms = now_ms()?;

        self.commit_record(
            &record,
            "task_plan_saved",
            Some("midas".to_string()),
            json!({
                "plan_bytes": record.plan.as_ref().map_or(0, String::len)
            }),
        )?;

        Ok(record)
    }

    /// Starts a planned task. Starting increments the attempt count.
    pub fn start_task(&self, id: &str) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;
        let mut record = self.get_task(id)?;

        if record.status != TaskStatus::Planned {
            bail!("Seule une tâche planifiée peut démarrer");
        }

        if record.plan.as_deref().unwrap_or_default().trim().is_empty() {
            bail!("Impossible de démarrer une tâche sans plan");
        }

        record.attempts = record
            .attempts
            .checked_add(1)
            .context("Le compteur de tentatives a dépassé sa limite")?;

        record.status = TaskStatus::Running;
        record.updated_at_ms = now_ms()?;

        self.commit_record(
            &record,
            "task_started",
            Some("midas".to_string()),
            json!({
                "attempt": record.attempts
            }),
        )?;

        Ok(record)
    }

    /// Pauses a task for an explicit approval decision.
    ///
    /// Do not include passwords, API keys or other secrets in `details`.
    pub fn request_approval(
        &self,
        id: &str,
        requested_by: &str,
        details: Value,
    ) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;

        validate_text(
            requested_by,
            "demandeur de validation",
            MAX_ACTOR_BYTES,
        )?;

        validate_json_size(
            &details,
            MAX_RESULT_BYTES,
            "détails de validation",
        )?;

        let mut record = self.get_task(id)?;

        if !matches!(
            &record.status,
            TaskStatus::Pending | TaskStatus::Planned | TaskStatus::Running
        ) {
            bail!(
                "Cette tâche ne peut pas passer en attente de validation \
                 depuis son état actuel"
            );
        }

        let now = now_ms()?;

        record.status = TaskStatus::AwaitingApproval;
        record.updated_at_ms = now;

        record.approval = Some(ApprovalRecord {
            requested_at_ms: now,
            requested_by: requested_by.trim().to_string(),
            details,
            resolved_at_ms: None,
            resolved_by: None,
            approved: None,
            resolution_note: None,
        });

        let details_bytes = serde_json::to_vec(
            &record
                .approval
                .as_ref()
                .context("Demande de validation manquante")?
                .details,
        )?
        .len();

        self.commit_record(
            &record,
            "approval_requested",
            Some(requested_by.trim().to_string()),
            json!({
                "details_bytes": details_bytes
            }),
        )?;

        Ok(record)
    }

    /// Records an approval decision.
    ///
    /// Approval returns the task to planned; rejection cancels it.
    /// The caller must authenticate `resolved_by` and authorize this decision
    /// before calling this storage method. The name is audit metadata,
    /// not proof of identity or authorization.
    pub fn resolve_approval(
        &self,
        id: &str,
        resolved_by: &str,
        approved: bool,
        note: Option<&str>,
    ) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;

        validate_text(resolved_by, "validateur", MAX_ACTOR_BYTES)?;

        if let Some(note) = note {
            validate_optional_text(
                note,
                "note de validation",
                MAX_APPROVAL_NOTE_BYTES,
            )?;
        }

        let mut record = self.get_task(id)?;

        if record.status != TaskStatus::AwaitingApproval {
            bail!("La tâche n'est pas en attente de validation");
        }

        let now = now_ms()?;

        let approval = record
            .approval
            .as_mut()
            .context(
                "État invalide : la demande de validation est absente",
            )?;

        if approval.resolved_at_ms.is_some() || approval.approved.is_some() {
            bail!("Cette demande de validation a déjà été traitée");
        }

        approval.resolved_at_ms = Some(now);
        approval.resolved_by = Some(resolved_by.trim().to_string());
        approval.approved = Some(approved);
        approval.resolution_note =
            note.map(|value| value.trim().to_string());

        record.status = if approved {
            TaskStatus::Planned
        } else {
            TaskStatus::Cancelled
        };

        record.updated_at_ms = now;

        self.commit_record(
            &record,
            "approval_resolved",
            Some(resolved_by.trim().to_string()),
            json!({
                "approved": approved,
                "note_present": note.is_some()
            }),
        )?;

        Ok(record)
    }

    /// Marks a running task as successful after its caller verifies the result.
    pub fn complete_task(
        &self,
        id: &str,
        result: Value,
    ) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;

        validate_json_size(&result, MAX_RESULT_BYTES, "résultat")?;

        let mut record = self.get_task(id)?;

        if record.status != TaskStatus::Running {
            bail!(
                "Seule une tâche en cours peut être terminée avec succès"
            );
        }

        let result_bytes = serde_json::to_vec(&result)?.len();

        record.result = Some(result);
        record.last_error = None;
        record.status = TaskStatus::Succeeded;
        record.updated_at_ms = now_ms()?;

        self.commit_record(
            &record,
            "task_succeeded",
            Some("midas".to_string()),
            json!({
                "result_bytes": result_bytes
            }),
        )?;

        Ok(record)
    }

    /// Marks a non-terminal task as failed and saves a diagnostic.
    pub fn fail_task(&self, id: &str, error: &str) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;
        validate_text(error, "erreur", MAX_ERROR_BYTES)?;

        let mut record = self.get_task(id)?;

        if record.status.is_terminal() {
            bail!(
                "Une tâche terminée ou annulée ne peut pas être marquée en échec"
            );
        }

        record.last_error = Some(error.trim().to_string());
        record.status = TaskStatus::Failed;
        record.updated_at_ms = now_ms()?;

        self.commit_record(
            &record,
            "task_failed",
            Some("midas".to_string()),
            json!({
                "error_bytes": record.last_error.as_ref().map_or(0, String::len)
            }),
        )?;

        Ok(record)
    }

    /// Requeues a failed task for another attempt.
    /// The caller must determine whether retrying is safe.
    pub fn requeue_failed_task(&self, id: &str) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;
        let mut record = self.get_task(id)?;

        if record.status != TaskStatus::Failed {
            bail!("Seule une tâche en échec peut être reprogrammée");
        }

        if record.plan.as_deref().unwrap_or_default().trim().is_empty() {
            bail!("Impossible de reprogrammer une tâche sans plan");
        }

        record.status = TaskStatus::Planned;
        record.last_error = None;
        record.updated_at_ms = now_ms()?;

        self.commit_record(
            &record,
            "task_requeued",
            Some("midas".to_string()),
            json!({
                "next_attempt": record.attempts.saturating_add(1)
            }),
        )?;

        Ok(record)
    }

    /// Cancels a non-terminal task and records the cancellation.
    /// The caller must check whether `cancelled_by` is authorized.
    pub fn cancel_task(
        &self,
        id: &str,
        cancelled_by: &str,
        reason: &str,
    ) -> Result<TaskRecord> {
        let _guard = self.lock_mutations()?;

        validate_text(
            cancelled_by,
            "auteur de l'annulation",
            MAX_ACTOR_BYTES,
        )?;

        validate_text(reason, "motif d'annulation", MAX_ERROR_BYTES)?;

        let mut record = self.get_task(id)?;

        if record.status.is_terminal() {
            bail!("La tâche est déjà terminée ou annulée");
        }

        let now = now_ms()?;

        if let Some(approval) = record.approval.as_mut() {
            if approval.resolved_at_ms.is_none() {
                approval.resolved_at_ms = Some(now);
                approval.resolved_by = Some(cancelled_by.trim().to_string());
                approval.approved = Some(false);
                approval.resolution_note = Some(reason.trim().to_string());
            }
        }

        record.status = TaskStatus::Cancelled;
        record.last_error = None;
        record.updated_at_ms = now;

        self.commit_record(
            &record,
            "task_cancelled",
            Some(cancelled_by.trim().to_string()),
            json!({
                "reason": reason.trim()
            }),
        )?;

        Ok(record)
    }

    /// Returns the most recent events from the append-only journal.
    pub fn recent_events(&self, limit: usize) -> Result<Vec<TaskEvent>> {
        if limit == 0 || limit > MAX_EVENT_LIST_LIMIT {
            bail!(
                "La limite doit être comprise entre 1 et {}",
                MAX_EVENT_LIST_LIMIT
            );
        }

        // Prevent reading while this process is appending an event.
        let _journal_guard = self.lock_journal()?;

        let file = File::open(&self.inner.events_path)
            .with_context(|| {
                format!(
                    "Impossible de lire {}",
                    self.inner.events_path.display()
                )
            })?;

        let reader = BufReader::new(file);
        let mut recent = VecDeque::with_capacity(limit);

        for line in reader.lines() {
            let line = line.context("Impossible de lire une ligne du journal")?;

            if line.trim().is_empty() {
                bail!("Le journal contient une ligne vide inattendue");
            }

            let event: TaskEvent = serde_json::from_str(&line)
                .context("Le journal contient un événement JSON invalide")?;

            if recent.len() == limit {
                recent.pop_front();
            }

            recent.push_back(event);
        }

        Ok(recent.into_iter().collect())
    }

    fn task_path(&self, id: &str) -> Result<PathBuf> {
        let canonical_id = canonical_uuid(id)?;
        Ok(self.inner.tasks_dir.join(format!("{canonical_id}.json")))
    }

    fn commit_record(
        &self,
        record: &TaskRecord,
        event_type: &str,
        actor: Option<String>,
        details: Value,
    ) -> Result<()> {
        // Calculate this before saving the snapshot so a clock error does
        // not leave a saved task without even attempting its audit event.
        let event_timestamp = now_ms()?;
        let path = self.task_path(&record.id)?;

        atomic_write_json(&path, record)
            .with_context(|| {
                format!("Impossible d'enregistrer la tâche {}", record.id)
            })?;

        let event = TaskEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp_ms: event_timestamp,
            task_id: record.id.clone(),
            event_type: event_type.to_string(),
            actor,
            details,
        };

        self.append_event(&event).with_context(|| {
            format!(
                "L'état de la tâche {} est enregistré, mais l'ajout au \
                 journal d'audit a échoué. Relire la tâche avant toute \
                 nouvelle tentative.",
                record.id
            )
        })
    }

    fn append_event(&self, event: &TaskEvent) -> Result<()> {
        let mut journal = self.lock_journal()?;

        serde_json::to_writer(&mut *journal, event)
            .context("Impossible de sérialiser l'événement")?;

        journal
            .write_all(b"\n")
            .context("Impossible de terminer la ligne du journal")?;

        journal
            .sync_data()
            .context("Impossible de synchroniser le journal sur le disque")?;

        Ok(())
    }

    fn lock_mutations(&self) -> Result<MutexGuard<'_, ()>> {
        self.inner
            .mutation_lock
            .lock()
            .map_err(|_| anyhow::anyhow!("Le verrou des tâches est empoisonné"))
    }

    fn lock_journal(&self) -> Result<MutexGuard<'_, File>> {
        self.inner
            .journal
            .lock()
            .map_err(|_| anyhow::anyhow!("Le verrou du journal est empoisonné"))
    }
}

fn canonical_uuid(id: &str) -> Result<String> {
    Uuid::parse_str(id)
        .map(|uuid| uuid.to_string())
        .context("Identifiant de tâche invalide : un UUID est requis")
}

fn validate_text(value: &str, label: &str, max_bytes: usize) -> Result<()> {
    if value.trim().is_empty() {
        bail!("Le champ {label} ne peut pas être vide");
    }

    if value.len() > max_bytes {
        bail!("Le champ {label} dépasse la limite de {max_bytes} octets");
    }

    Ok(())
}

fn validate_optional_text(
    value: &str,
    label: &str,
    max_bytes: usize,
) -> Result<()> {
    if value.len() > max_bytes {
        bail!("Le champ {label} dépasse la limite de {max_bytes} octets");
    }

    Ok(())
}

fn validate_json_size(
    value: &Value,
    max_bytes: usize,
    label: &str,
) -> Result<()> {
    let size = serde_json::to_vec(value)
        .with_context(|| format!("Impossible de mesurer le champ {label}"))?
        .len();

    if size > max_bytes {
        bail!("Le champ {label} dépasse la limite de {max_bytes} octets");
    }

    Ok(())
}

fn now_ms() -> Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("L'horloge système est antérieure à l'époque Unix")?;

    u64::try_from(duration.as_millis())
        .context("L'horodatage dépasse la limite prise en charge")
}

fn atomic_write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path
        .parent()
        .context("Le chemin du fichier ne contient pas de répertoire parent")?;

    fs::create_dir_all(parent)
        .with_context(|| format!("Impossible de créer {}", parent.display()))?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Nom de fichier invalide")?;

    let temp_path =
        parent.join(format!(".{file_name}.{}.tmp", Uuid::new_v4()));

    let write_result = (|| -> Result<()> {
        let mut temp_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .with_context(|| {
                format!("Impossible de créer {}", temp_path.display())
            })?;

        serde_json::to_writer(&mut temp_file, value)
            .with_context(|| {
                format!("Impossible d'écrire le JSON dans {}", temp_path.display())
            })?;

        temp_file
            .write_all(b"\n")
            .with_context(|| {
                format!("Impossible de terminer {}", temp_path.display())
            })?;

        temp_file
            .sync_all()
            .with_context(|| {
                format!("Impossible de synchroniser {}", temp_path.display())
            })?;

        drop(temp_file);

        fs::rename(&temp_path, path)
            .with_context(|| {
                format!("Impossible de remplacer {}", path.display())
            })?;

        // Directory syncing is best-effort because platform support differs.
        if let Ok(directory) = File::open(parent) {
            let _ = directory.sync_all();
        }

        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    write_result
}

#[cfg(test)]
mod tests {
    use super::{TaskStatus, TaskStore};
    use serde_json::json;
    use std::fs;
    use std::thread;
    use uuid::Uuid;

    fn test_root() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "midas-task-store-test-{}",
            Uuid::new_v4()
        ))
    }

    #[test]
    fn persists_complete_task_lifecycle_and_events() {
        let root = test_root();
        let store = TaskStore::open(&root).expect("store opens");

        let created = store
            .create_task("Créer un prototype")
            .expect("task created");

        assert_eq!(created.status, TaskStatus::Pending);

        let planned = store
            .save_plan(&created.id, "Concevoir, coder et tester")
            .expect("plan saved");

        assert_eq!(planned.status, TaskStatus::Planned);

        let running = store
            .start_task(&created.id)
            .expect("task starts");

        assert_eq!(running.status, TaskStatus::Running);
        assert_eq!(running.attempts, 1);

        let completed = store
            .complete_task(&created.id, json!({ "tests_passed": true }))
            .expect("task completes");

        assert_eq!(completed.status, TaskStatus::Succeeded);

        let reloaded = store.get_task(&created.id).expect("task reloads");

        assert_eq!(reloaded.status, TaskStatus::Succeeded);
        assert_eq!(
            reloaded.result,
            Some(json!({ "tests_passed": true }))
        );

        let events = store.recent_events(20).expect("events load");

        assert_eq!(events.len(), 4);
        assert_eq!(events[0].event_type, "task_created");
        assert_eq!(events[3].event_type, "task_succeeded");

        drop(store);
        fs::remove_dir_all(root).expect("test data removed");
    }

    #[test]
    fn approval_rejection_prevents_task_start() {
        let root = test_root();
        let store = TaskStore::open(&root).expect("store opens");

        let task = store
            .create_task("Opération sensible")
            .expect("task created");

        store
            .save_plan(&task.id, "Préparer puis demander validation")
            .expect("plan saved");

        store
            .request_approval(
                &task.id,
                "test-operator",
                json!({ "reason": "action with external impact" }),
            )
            .expect("approval requested");

        assert!(store.start_task(&task.id).is_err());

        let rejected = store
            .resolve_approval(
                &task.id,
                "test-operator",
                false,
                Some("Non autorisé"),
            )
            .expect("approval rejected");

        assert_eq!(rejected.status, TaskStatus::Cancelled);
        assert!(store.start_task(&task.id).is_err());

        drop(store);
        fs::remove_dir_all(root).expect("test data removed");
    }

    #[test]
    fn failed_task_can_be_requeued_without_losing_attempt_count() {
        let root = test_root();
        let store = TaskStore::open(&root).expect("store opens");

        let task = store
            .create_task("Construire un programme")
            .expect("task created");

        store
            .save_plan(&task.id, "Écrire le code et lancer les tests")
            .expect("plan saved");

        store.start_task(&task.id).expect("task starts");

        store
            .fail_task(&task.id, "Erreur de test contrôlée")
            .expect("task failed");

        let requeued = store
            .requeue_failed_task(&task.id)
            .expect("task requeued");

        assert_eq!(requeued.status, TaskStatus::Planned);
        assert_eq!(requeued.attempts, 1);
        assert!(requeued.last_error.is_none());

        drop(store);
        fs::remove_dir_all(root).expect("test data removed");
    }

    #[test]
    fn concurrent_threads_create_distinct_tasks() {
        let root = test_root();
        let store = TaskStore::open(&root).expect("store opens");
        let mut workers = Vec::new();

        for index in 0..8 {
            let worker_store = store.clone();

            workers.push(thread::spawn(move || {
                worker_store
                    .create_task(&format!("Tâche de test {index}"))
                    .expect("task created")
                    .id
            }));
        }

        let mut ids = Vec::new();

        for worker in workers {
            ids.push(worker.join().expect("worker completed"));
        }

        ids.sort();
        ids.dedup();

        assert_eq!(ids.len(), 8);

        let records = store.list_tasks(100).expect("tasks list");
        assert_eq!(records.len(), 8);

        drop(store);
        fs::remove_dir_all(root).expect("test data removed");
    }

    #[test]
    fn invalid_uuid_cannot_be_used_as_a_file_path() {
        let root = test_root();
        let store = TaskStore::open(&root).expect("store opens");

        assert!(store.get_task("../../outside").is_err());

        drop(store);
        fs::remove_dir_all(root).expect("test data removed");
    }
}
