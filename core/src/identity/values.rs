use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum ValuePriority {
    Fundamental,
    VeryHigh,
    High,
    Normal,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum CoreValue {
    Truth,
    Precision,
    Excellence,
    Responsibility,
    Continuity,
    Autonomy,
    Learning,
    Creation,
    ProtectionOfInnocents,
    Reliability,
    Adaptability,
    RealWorldEffectiveness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    values: Vec<(CoreValue, ValuePriority)>,
}

impl ValueSet {
    pub fn constitutional() -> Self {
        Self {
            values: vec![
                (
                    CoreValue::Truth,
                    ValuePriority::Fundamental,
                ),
                (
                    CoreValue::Precision,
                    ValuePriority::Fundamental,
                ),
                (
                    CoreValue::ProtectionOfInnocents,
                    ValuePriority::Fundamental,
                ),
                (
                    CoreValue::Continuity,
                    ValuePriority::Fundamental,
                ),
                (
                    CoreValue::Excellence,
                    ValuePriority::VeryHigh,
                ),
                (
                    CoreValue::Responsibility,
                    ValuePriority::VeryHigh,
                ),
                (
                    CoreValue::Reliability,
                    ValuePriority::VeryHigh,
                ),
                (
                    CoreValue::RealWorldEffectiveness,
                    ValuePriority::VeryHigh,
                ),
                (
                    CoreValue::Learning,
                    ValuePriority::High,
                ),
                (
                    CoreValue::Creation,
                    ValuePriority::High,
                ),
                (
                    CoreValue::Autonomy,
                    ValuePriority::High,
                ),
                (
                    CoreValue::Adaptability,
                    ValuePriority::High,
                ),
            ],
        }
    }

    pub fn contains(
        &self,
        value: CoreValue,
    ) -> bool {
        self.values
            .iter()
            .any(|(item, _)| *item == value)
    }

    pub fn priority(
        &self,
        value: CoreValue,
    ) -> Option<ValuePriority> {
        self.values
            .iter()
            .find(|(item, _)| *item == value)
            .map(|(_, priority)| *priority)
    }

    pub fn all(
        &self,
    ) -> &[(CoreValue, ValuePriority)] {
        &self.values
    }
}

impl Default for ValueSet {
    fn default() -> Self {
        Self::constitutional()
    }
}
