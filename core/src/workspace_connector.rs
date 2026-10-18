
use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::connector::{Connector, ConnectorRequest, ConnectorResult};

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Connecteur de fichiers confiné à un répertoire précis.
///
/// Opérations prises en charge :
/// - list : lister les entrées directes d'un répertoire ;
/// - read : lire un fichier UTF-8 ;
/// - write : créer ou remplacer un fichier UTF-8.
///
/// Les chemins absolus, les traversées avec ".." et les accès qui
/// sortent de la racine autorisée sont refusés.
///
/// Ce connecteur ne fournit aucun accès au shell ni au système
/// de fichiers situé en dehors de sa racine.
pub struct WorkspaceConnector {
    root: PathBuf,
}

impl WorkspaceConnector {
    /// Crée le connecteur à partir d'un répertoire déjà existant.
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = fs::canonicalize(root.as_ref())
            .with_context(|| {
                format!(
                    "Impossible de résoudre le répertoire de travail : {}",
                    root.as_ref().display()
                )
            })?;

        if !root.is_dir() {
            return Err(anyhow!(
                "La racine du workspace doit être un répertoire"
            ));
        }

        Ok(Self { root })
    }

    /// Retourne la racine canonique du workspace.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn argument_path<'a>(
        request: &'a ConnectorRequest,
    ) -> Result<&'a str> {
        request
            .arguments
            .get("path")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .ok_or_else(|| {
                anyhow!(
                    "L'argument 'path' doit être une chaîne non vide"
                )
            })
    }

    /// Vérifie la syntaxe d'un chemin relatif.
    fn validate_relative_path(path: &str) -> Result<PathBuf> {
        let candidate = Path::new(path);

        if candidate.is_absolute() {
            return Err(anyhow!(
                "Les chemins absolus sont interdits dans le workspace"
            ));
        }

        for component in candidate.components() {
            match component {
                Component::Normal(_) | Component::CurDir => {}
                Component::ParentDir => {
                    return Err(anyhow!(
                        "Les segments '..' sont interdits"
                    ));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(anyhow!(
                        "Le chemin doit rester relatif à la racine"
                    ));
                }
            }
        }

        Ok(candidate.to_path_buf())
    }

    /// Résout un chemin existant et vérifie qu'il reste dans la racine.
    ///
    /// La canonicalisation permet notamment de détecter les liens
    /// symboliques qui conduiraient hors du workspace.
    fn resolve_existing(&self, path: &str) -> Result<PathBuf> {
        let relative = Self::validate_relative_path(path)?;
        let joined = self.root.join(relative);

        let resolved = fs::canonicalize(&joined)
            .with_context(|| {
                format!("Chemin introuvable : {}", joined.display())
            })?;

        if !resolved.starts_with(&self.root) {
            return Err(anyhow!(
                "Accès refusé : le chemin sort du workspace"
            ));
        }

        Ok(resolved)
    }

    fn list_directory(&self, path: &str) -> Result<ConnectorResult> {
        let directory = self.resolve_existing(path)?;

        if !directory.is_dir() {
            return Err(anyhow!(
                "Le chemin demandé n'est pas un répertoire"
            ));
        }

        let mut entries = Vec::new();

        for entry_result in fs::read_dir(&directory)? {
            let entry = entry_result?;
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let metadata = fs::symlink_metadata(entry.path())?;

            // Ne pas suivre les liens symboliques lors de l'inventaire.
            let file_type = metadata.file_type();

            entries.push(json!({
                "name": file_name,
                "is_file": file_type.is_file(),
                "is_directory": file_type.is_dir(),
                "is_symlink": file_type.is_symlink(),
                "size_bytes": metadata.len()
            }));
        }

        entries.sort_by(|a, b| {
            a["name"]
                .as_str()
                .unwrap_or("")
                .cmp(b["name"].as_str().unwrap_or(""))
        });

        Ok(ConnectorResult::success(
            self.name(),
            "list",
            format!("{} entrée(s) trouvée(s)", entries.len()),
            Some(json!({
                "path": path,
                "entries": entries
            })),
        ))
    }

    fn read_file(&self, path: &str) -> Result<ConnectorResult> {
        let resolved = self.resolve_existing(path)?;

        if !resolved.is_file() {
            return Err(anyhow!(
                "Le chemin demandé n'est pas un fichier ordinaire"
            ));
        }

        let content = fs::read_to_string(&resolved)
            .with_context(|| {
                format!(
                    "Impossible de lire le fichier UTF-8 : {}",
                    resolved.display()
                )
            })?;

        Ok(ConnectorResult::success(
            self.name(),
            "read",
            format!("Fichier lu : {path}"),
            Some(json!({
                "path": path,
                "content": content,
                "size_bytes": content.len()
            })),
        ))
    }

    fn write_file(&self, path: &str, content: &str) -> Result<ConnectorResult> {
        let relative = Self::validate_relative_path(path)?;

        if relative.as_os_str().is_empty() || relative == Path::new(".") {
            return Err(anyhow!(
                "Le chemin de destination doit désigner un fichier"
            ));
        }

        let destination = self.root.join(&relative);

        let parent = destination
            .parent()
            .ok_or_else(|| anyhow!("Le fichier doit avoir un répertoire parent"))?;

        // Le répertoire parent doit déjà exister.
        // Ce choix évite de créer implicitement une arborescence
        // dont les permissions et la destination seraient incertaines.
        let canonical_parent = fs::canonicalize(parent)
            .with_context(|| {
                format!(
                    "Le répertoire parent n'existe pas : {}",
                    parent.display()
                )
            })?;

        if !canonical_parent.starts_with(&self.root) {
            return Err(anyhow!(
                "Accès refusé : le répertoire parent sort du workspace"
            ));
        }

        let destination_name = destination
            .file_name()
            .ok_or_else(|| anyhow!("Nom de fichier invalide"))?;

        // Si la destination existe, refuser les liens symboliques et
        // les fichiers non ordinaires.
        match fs::symlink_metadata(&destination) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(anyhow!(
                        "L'écriture à travers un lien symbolique est interdite"
                    ));
                }

                if !metadata.is_file() {
                    return Err(anyhow!(
                        "La destination existe mais n'est pas un fichier ordinaire"
                    ));
                }

                let canonical_destination = fs::canonicalize(&destination)?;

                if !canonical_destination.starts_with(&self.root) {
                    return Err(anyhow!(
                        "Accès refusé : la destination sort du workspace"
                    ));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).context(
                    "Impossible de vérifier la destination",
                );
            }
        }

        // Écriture temporaire dans le même répertoire, puis renommage.
        // Cela réduit le risque de laisser un fichier partiellement écrit.
        let mut temporary_path = None;
        let mut temporary_file = None;

        for _ in 0..20 {
            let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);

            let temporary_name = format!(
                ".midas-write-{}-{counter}.tmp",
                std::process::id()
            );

            let candidate = canonical_parent.join(temporary_name);

            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&candidate)
            {
                Ok(file) => {
                    temporary_path = Some(candidate);
                    temporary_file = Some(file);
                    break;
                }
                Err(error)
                    if error.kind() == std::io::ErrorKind::AlreadyExists =>
                {
                    continue;
                }
                Err(error) => {
                    return Err(error).context(
                        "Impossible de créer le fichier temporaire",
                    );
                }
            }
        }

        let temporary_path = temporary_path
            .ok_or_else(|| anyhow!("Impossible de réserver un fichier temporaire"))?;

        let mut file = temporary_file
            .ok_or_else(|| anyhow!("Fichier temporaire indisponible"))?;

        let write_result = (|| -> Result<()> {
            file.write_all(content.as_bytes())
                .context("Échec de l'écriture du contenu")?;

            file.sync_all()
                .context("Impossible de synchroniser le fichier temporaire")?;

            drop(file);

            // Vérifier une dernière fois le parent avant le renommage.
            let parent_check = fs::canonicalize(parent)
                .context("Le répertoire parent a changé pendant l'écriture")?;

            if parent_check != canonical_parent
                || !parent_check.starts_with(&self.root)
            {
                return Err(anyhow!(
                    "Le répertoire parent a changé : écriture annulée"
                ));
            }

            // Le renommage remplace atomiquement la destination sur les
            // systèmes de fichiers usuels lorsque source et destination
            // appartiennent au même système de fichiers.
            fs::rename(&temporary_path, &destination)
                .context("Impossible de finaliser le fichier")?;

            Ok(())
        })();

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }

        Ok(ConnectorResult::success(
            self.name(),
            "write",
            format!("Fichier écrit : {path}"),
            Some(json!({
                "path": path,
                "bytes_written": content.len()
            })),
        ))
    }
}

impl Connector for WorkspaceConnector {
    fn name(&self) -> &str {
        "workspace"
    }

    fn description(&self) -> &str {
        "Lecture, inventaire et écriture de fichiers dans un répertoire autorisé"
    }

    fn supports(&self, operation: &str) -> bool {
        matches!(operation, "list" | "read" | "write")
    }

    fn execute(&self, request: &ConnectorRequest) -> Result<ConnectorResult> {
        if request.connector != self.name() {
            return Err(anyhow!(
                "Cette demande cible '{}', pas '{}'",
                request.connector,
                self.name()
            ));
        }

        match request.operation.as_str() {
            "list" => {
                let path = request
                    .arguments
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or(".");

                self.list_directory(path)
            }

            "read" => {
                let path = Self::argument_path(request)?;
                self.read_file(path)
            }

            "write" => {
                let path = Self::argument_path(request)?;

                let content = request
                    .arguments
                    .get("content")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        anyhow!(
                            "L'argument 'content' doit être une chaîne UTF-8"
                        )
                    })?;

                self.write_file(path, content)
            }

            _ => Err(anyhow!(
                "Opération non prise en charge : '{}'",
                request.operation
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            let path = std::env::temp_dir().join(format!(
                "midas-workspace-test-{}-{timestamp}",
                std::process::id()
            ));

            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn request(operation: &str, arguments: Value) -> ConnectorRequest {
        ConnectorRequest {
            connector: "workspace".to_owned(),
            operation: operation.to_owned(),
            arguments,
        }
    }

    #[test]
    fn writes_and_reads_a_file() {
        let directory = TestDirectory::new();
        let connector = WorkspaceConnector::new(&directory.0).unwrap();

        let write_result = connector
            .execute(&request(
                "write",
                json!({
                    "path": "example.txt",
                    "content": "MIDAS"
                }),
            ))
            .unwrap();

        assert!(write_result.success);

        let read_result = connector
            .execute(&request(
                "read",
                json!({"path": "example.txt"}),
            ))
            .unwrap();

        assert_eq!(
            read_result.data.unwrap()["content"],
            "MIDAS"
        );
    }

    #[test]
    fn refuses_parent_directory_traversal() {
        let directory = TestDirectory::new();
        let connector = WorkspaceConnector::new(&directory.0).unwrap();

        let result = connector.execute(&request(
            "read",
            json!({"path": "../outside.txt"}),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn lists_workspace_entries() {
        let directory = TestDirectory::new();
        fs::write(directory.0.join("visible.txt"), "test").unwrap();

        let connector = WorkspaceConnector::new(&directory.0).unwrap();

        let result = connector
            .execute(&request("list", json!({"path": "."})))
            .unwrap();

        let entries = result.data.unwrap()["entries"]
            .as_array()
            .unwrap()
            .clone();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["name"], "visible.txt");
    }

    #[test]
    fn rejects_missing_read_path() {
        let directory = TestDirectory::new();
        let connector = WorkspaceConnector::new(&directory.0).unwrap();

        let result = connector.execute(&request(
            "read",
            json!({"path": "missing.txt"}),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_paths() {
        let result = WorkspaceConnector::validate_relative_path(
            "/etc/passwd",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_write_content_argument() {
        let directory = TestDirectory::new();
        let connector = WorkspaceConnector::new(&directory.0).unwrap();

        let result = connector.execute(&request(
            "write",
            json!({"path": "example.txt"}),
        ));

        assert!(result.is_err());
    }
}
