use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoreValue {
    Truth,
    Precision,
    Excellence,
    Creation,
    Learning,
    Continuity,
    Responsibility,
    Autonomy,
    Utility,
    HumanSafety,
    Integrity,
    Improvement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuePriority {
    pub value: CoreValue,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    pub priorities: Vec<ValuePriority>,
}

impl Default for ValueSet {
    fn default() -> Self {
        Self {
            priorities: vec![
                ValuePriority {
                    value: CoreValue::HumanSafety,
                    priority: 1,
                },
                ValuePriority {
                    value: CoreValue::Truth,
                    priority: 2,
                },
                ValuePriority {
                    value: CoreValue::Integrity,
                    priority: 3,
                },
                ValuePriority {
                    value: CoreValue::Precision,
                    priority: 4,
                },
                ValuePriority {
                    value: CoreValue::Responsibility,
                    priority: 5,
                },
                ValuePriority {
                    value: CoreValue::Excellence,
                    priority: 6,
                },
                ValuePriority {
                    value: CoreValue::Creation,
                    priority: 7,
                },
                ValuePriority {
                    value: CoreValue::Learning,
                    priority: 8,
                },
                ValuePriority {
                    value: CoreValue::Improvement,
                    priority: 9,
                },
                ValuePriority {
                    value: CoreValue::Continuity,
                    priority: 10,
                },
                ValuePriority {
                    value: CoreValue::Utility,
                    priority: 11,
                },
                ValuePriority {
                    value: CoreValue::Autonomy,
                    priority: 12,
                },
            ],
        }
    }
}

impl ValueSet {
    pub fn priority_of(&self, value: &CoreValue) -> Option<u32> {
        self.priorities
            .iter()
            .find(|entry| &entry.value == value)
            .map(|entry| entry.priority)
    }

    pub fn contains(&self, value: &CoreValue) -> bool {
        self.priority_of(value).is_some()
    }
}
