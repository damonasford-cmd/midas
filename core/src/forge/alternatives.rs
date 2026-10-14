use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredProperty {
    pub name: String,
    pub description: String,
    pub target_value: Option<Value>,
    pub tolerance: Option<Value>,
    pub measurement_method: Option<String>,
    pub mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalNeed {
    pub id: Uuid,
    pub name: String,
    pub intended_effect: String,
    pub operating_conditions: Value,
    pub required_properties: Vec<RequiredProperty>,
    pub constraints: Value,
}

impl FunctionalNeed {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.intended_effect.trim().is_empty() {
            bail!("A functional need requires a name and intended effect");
        }
        if self.required_properties.is_empty() {
            bail!("A functional need must declare its required properties");
        }
        for property in &self.required_properties {
            if property.name.trim().is_empty() {
                bail!("Required property names cannot be empty");
            }
            if property.mandatory && property.measurement_method.is_none() {
                bail!(
                    "Mandatory property '{}' needs a measurement method",
                    property.name
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateOrigin {
    ExistingMaterial,
    ExistingDevice,
    ModifiedExistingSolution,
    NewlyDesignedMaterial,
    NewlyDesignedDevice,
    HybridSolution,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStatus {
    Proposed,
    UnderResearch,
    FeasibleInTheory,
    PrototypeRequired,
    PrototypeBuilt,
    Measured,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeCandidate {
    pub id: Uuid,
    pub name: String,
    pub origin: CandidateOrigin,
    pub status: CandidateStatus,
    pub proposed_mechanism: String,
    pub supporting_evidence: Vec<Uuid>,
    pub unresolved_questions: Vec<String>,
    pub expected_properties: Value,
    pub safety_constraints: Value,
}

impl AlternativeCandidate {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.proposed_mechanism.trim().is_empty() {
            bail!("Alternative candidates need a name and a proposed mechanism");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyAssessment {
    pub property_name: String,
    pub target: Option<Value>,
    pub measured: Option<Value>,
    pub measurement_evidence: Option<Uuid>,
    pub meets_requirement: Option<bool>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeAssessment {
    pub candidate_id: Uuid,
    pub assessments: Vec<PropertyAssessment>,
    pub meets_all_mandatory_requirements: bool,
    pub remaining_gaps: Vec<String>,
    pub assessed_at: chrono::DateTime<chrono::Utc>,
}

impl AlternativeAssessment {
    pub fn from_measurements(
        need: &FunctionalNeed,
        candidate_id: Uuid,
        assessments: Vec<PropertyAssessment>,
    ) -> Result<Self> {
        need.validate()?;

        let mut remaining_gaps = Vec::new();
        let mut meets_all_mandatory_requirements = true;

        for required in &need.required_properties {
            let assessment = assessments
                .iter()
                .find(|item| item.property_name == required.name);

            match assessment {
                Some(item) if item.meets_requirement == Some(true) => {}
                Some(item) => {
                    if required.mandatory {
                        meets_all_mandatory_requirements = false;
                    }
                    remaining_gaps.push(format!(
                        "{}: {}",
                        required.name,
                        item.explanation
                    ));
                }
                None => {
                    if required.mandatory {
                        meets_all_mandatory_requirements = false;
                    }
                    remaining_gaps.push(format!(
                        "{}: no measurement assessment supplied",
                        required.name
                    ));
                }
            }
        }

        Ok(Self {
            candidate_id,
            assessments,
            meets_all_mandatory_requirements,
            remaining_gaps,
            assessed_at: chrono::Utc::now(),
        })
    }
}
