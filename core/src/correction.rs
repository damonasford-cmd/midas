use crate::learning::Learning;

#[derive(Debug, Clone)]
pub struct Correction {
    pub required: bool,
    pub instruction: String,
}

pub fn correct(learning: &Learning) -> Correction {
    Correction {
        required: false,
        instruction: format!(
            "Aucune correction nécessaire: {}",
            learning.lesson
        ),
    }
}
