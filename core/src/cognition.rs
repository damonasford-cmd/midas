use crate::perception::Perception;

#[derive(Debug, Clone)]
pub struct Understanding {
    pub summary: String,
    pub facts: Vec<String>,
}

pub fn understand(
    goal: &str,
    perceptions: &[Perception],
) -> Understanding {
    let facts = perceptions
        .iter()
        .map(|p| format!("{}: {}", p.source, p.content))
        .collect();

    Understanding {
        summary: format!("Objectif compris: {goal}"),
        facts,
    }
}
