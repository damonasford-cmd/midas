#[derive(Debug, Clone)]
pub struct Perception {
    pub source: String,
    pub content: String,
}

impl Perception {
    pub fn new(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            content: content.into(),
        }
    }
}
