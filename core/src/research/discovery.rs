
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::research::{
    domain::{ResearchRun, ResearchStatus},
    evidence::Evidence,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryKind {
    Finding,
    KnowledgeGap,
    Contradiction,
    Opportunity,
    Risk,
    NextStep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovery {
    pub id: Uuid,
    pub objective_id: Uuid,
    pub kind: DiscoveryKind,
    pub title: String,
    pub description: String,
    pub related_evidence_ids: Vec<Uuid>,
    pub confidence: f64,
    pub requires_follow_up: bool,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub run: ResearchRun,
    pub evidence: Vec<Evidence>,
    pub discoveries: Vec<Discovery>,
    pub summary: String,
    pub generated_at: DateTime<Utc>,
}

pub fn build_discoveries(
    run: &ResearchRun,
    evidence: &[Evidence],
) -> Vec<Discovery> {
    let mut discoveries = Vec::new();

    if evidence.is_empty() {
        discoveries.push(Discovery {
            id: Uuid::new_v4(),
            objective_id: run.objective.id,
            kind: DiscoveryKind::KnowledgeGap,
            title: "Informations insuffisantes".to_owned(),
            description: "Aucune preuve exploitable n'a été collectée. Il faut connecter ou réparer les méthodes de recherche nécessaires, puis relancer les tâches bloquées.".to_owned(),
            related_evidence_ids: Vec::new(),
            confidence: 1.0,
            requires_follow_up: true,
            discovered_at: Utc::now(),
        });
    } else {
        let disputed: Vec<_> = evidence.iter()
            .filter(|item| {
                item.verification == crate::research::evidence::VerificationLevel::Disputed
            })
            .collect();

        if !disputed.is_empty() {
            discoveries.push(Discovery {
                id: Uuid::new_v4(),
                objective_id: run.objective.id,
                kind: DiscoveryKind::Contradiction,
                title: "Éléments contestés".to_owned(),
                description: format!(
                    "{} élément(s) sont marqués comme contestés et demandent une vérification indépendante.",
                    disputed.len()
                ),
                related_evidence_ids: disputed.iter().map(|item| item.id).collect(),
                confidence: 1.0,
                requires_follow_up: true,
                discovered_at: Utc::now(),
            });
        }

        let unchecked: Vec<_> = evidence.iter()
            .filter(|item| {
                item.verification == crate::research::evidence::VerificationLevel::Unchecked
            })
            .collect();

        if !unchecked.is_empty() {
            discoveries.push(Discovery {
                id: Uuid::new_v4(),
                objective_id: run.objective.id,
                kind: DiscoveryKind::KnowledgeGap,
                title: "Preuves non vérifiées".to_owned(),
                description: format!(
                    "{} élément(s) n'ont pas encore été vérifiés. Ne pas les traiter comme des faits établis.",
                    unchecked.len()
                ),
                related_evidence_ids: unchecked.iter().map(|item| item.id).collect(),
                confidence: 1.0,
                requires_follow_up: true,
                discovered_at: Utc::now(),
            });
        }
    }

    let incomplete = run.tasks.iter().any(|task| {
        !matches!(
            task.status,
            crate::research::domain::TaskStatus::Completed
                | crate::research::domain::TaskStatus::Skipped
        )
    });

    if incomplete || run.status == ResearchStatus::PartiallyCompleted {
        discoveries.push(Discovery {
            id: Uuid::new_v4(),
            objective_id: run.objective.id,
            kind: DiscoveryKind::NextStep,
            title: "Poursuivre les tâches inachevées".to_owned(),
            description: "Certaines tâches restent inachevées ou bloquées. Le moteur doit résoudre leurs dépendances ou connecter les moyens manquants avant de conclure.".to_owned(),
            related_evidence_ids: Vec::new(),
            confidence: 1.0,
            requires_follow_up: true,
            discovered_at: Utc::now(),
        });
    }

    discoveries
}

pub fn summarize(run: &ResearchRun, evidence: &[Evidence]) -> String {
    format!(
        "Objectif : {}. Tâches : {} au total, {} terminées, {} en échec ou bloquées. Éléments collectés : {}. Statut : {:?}.",
        run.objective.title,
        run.tasks.len(),
        run.tasks.iter().filter(|task| {
            task.status == crate::research::domain::TaskStatus::Completed
        }).count(),
        run.tasks.iter().filter(|task| {
            matches!(
                task.status,
                crate::research::domain::TaskStatus::Failed
                    | crate::research::domain::TaskStatus::Blocked
            )
        }).count(),
        evidence.len(),
        run.status
    )
}

pub fn create_report(run: ResearchRun, evidence: Vec<Evidence>) -> ResearchReport {
    let discoveries = build_discoveries(&run, &evidence);
    let summary = summarize(&run, &evidence);

    ResearchReport {
        run,
        evidence,
        discoveries,
        summary,
        generated_at: Utc::now(),
    }
}
