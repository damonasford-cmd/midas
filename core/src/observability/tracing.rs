use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpan {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct TraceRegistry {
    spans: Vec<TraceSpan>,
}

impl TraceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(
        &mut self,
        name: impl Into<String>,
        parent_id: Option<Uuid>,
    ) -> Uuid {
        let span = TraceSpan {
            id: Uuid::new_v4(),
            parent_id,
            name: name.into(),
            started_at: Utc::now(),
            ended_at: None,
        };

        let id = span.id;
        self.spans.push(span);
        id
    }

    pub fn finish(&mut self, id: Uuid) -> bool {
        if let Some(span) = self.spans.iter_mut().find(|span| span.id == id) {
            span.ended_at = Some(Utc::now());
            true
        } else {
            false
        }
    }
}
