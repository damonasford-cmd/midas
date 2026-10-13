
use crate::research::domain::{
    ResearchMethodKind, ResearchObjective, ResearchTask,
};

pub fn decompose_objective(
    objective: &ResearchObjective,
) -> anyhow::Result<Vec<ResearchTask>> {
    objective.validate()?;

    let title = objective.title.to_lowercase();
    let outcome = objective.desired_outcome.to_lowercase();
    let combined = format!("{title} {outcome}");

    let mut tasks = vec![
        ResearchTask::new(
            objective.id,
            "Clarifier le résultat",
            format!(
                "Définir les critères mesurables de réussite pour : {}",
                objective.desired_outcome
            ),
            ResearchMethodKind::GeneralAnalysis,
            10,
        ),
        ResearchTask::new(
            objective.id,
            "Étudier la faisabilité",
            format!(
                "Évaluer la faisabilité technique, opérationnelle, financière et temporelle de : {}",
                objective.desired_outcome
            ),
            ResearchMethodKind::GeneralAnalysis,
            10,
        ),
        ResearchTask::new(
            objective.id,
            "Identifier les ressources",
            format!(
                "Identifier les connaissances, outils, compétences, matériels, services et ressources nécessaires à : {}",
                objective.desired_outcome
            ),
            ResearchMethodKind::GeneralAnalysis,
            8,
        ),
        ResearchTask::new(
            objective.id,
            "Identifier les risques et obligations",
            format!(
                "Identifier les risques, contraintes, obligations légales et autorisations applicables à : {}",
                objective.desired_outcome
            ),
            ResearchMethodKind::GeneralAnalysis,
            9,
        ),
        ResearchTask::new(
            objective.id,
            "Déterminer les prochaines étapes",
            format!(
                "Déterminer les étapes concrètes, les dépendances et les critères de vérification pour réaliser : {}",
                objective.desired_outcome
            ),
            ResearchMethodKind::GeneralAnalysis,
            9,
        ),
    ];

    if contains_any(&combined, &[
        "robot", "robotique", "machine", "mécanique", "matériel", "physique",
    ]) {
        tasks.push(ResearchTask::new(
            objective.id,
            "Étude matérielle et physique",
            "Étudier les composants, matériaux, actionneurs, capteurs, contraintes physiques, fabrication et essais nécessaires.",
            ResearchMethodKind::GeneralAnalysis,
            8,
        ));
    }

    if contains_any(&combined, &[
        "application", "logiciel", "plateforme", "saas", "site web", "trading",
    ]) {
        tasks.push(ResearchTask::new(
            objective.id,
            "Étude logicielle",
            "Étudier l'architecture, les interfaces, les données, les intégrations, les tests, la sécurité et le déploiement.",
            ResearchMethodKind::GeneralAnalysis,
            8,
        ));
    }

    if contains_any(&combined, &[
        "banque", "finance", "paiement", "monnaie", "coin", "crypto", "trading",
    ]) {
        tasks.push(ResearchTask::new(
            objective.id,
            "Étude financière et réglementaire",
            "Étudier les modèles économiques, les risques financiers, les obligations réglementaires et les autorisations nécessaires selon les juridictions concernées.",
            ResearchMethodKind::GeneralAnalysis,
            10,
        ));
    }

    if contains_any(&combined, &[
        "ville", "bâtiment", "construction", "infrastructure", "immobilier",
    ]) {
        tasks.push(ResearchTask::new(
            objective.id,
            "Étude territoriale et de construction",
            "Étudier le terrain, les infrastructures, les études techniques, les autorisations, le financement et les intervenants habilités.",
            ResearchMethodKind::GeneralAnalysis,
            9,
        ));
    }

    for (index, question) in objective.questions.iter().enumerate() {
        tasks.push(ResearchTask::new(
            objective.id,
            format!("Question spécifique {}", index + 1),
            question.clone(),
            ResearchMethodKind::GeneralAnalysis,
            7,
        ));
    }

    // Toutes les tâches de base doivent être terminées avant la synthèse finale.
    let base_ids: Vec<_> = tasks.iter().map(|task| task.id).collect();

    for task in tasks.iter_mut().skip(1) {
        task.depends_on = vec![base_ids[0]];
    }

    Ok(tasks)
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
