use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::observation::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Learning {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub lesson: String,
    pub successful: bool,
    pub observations: Vec<String>,
    pub improvements: Vec<String>,
    pub confidence: f32,
}

impl Learning {
    pub fn new(
        source: impl Into<String>,
        lesson: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            lesson: lesson.into(),
            successful: false,
            observations: Vec::new(),
            improvements: Vec::new(),
            confidence: 1.0,
        }
    }

    pub fn with_success(
        mut self,
        successful: bool,
    ) -> Self {
        self.successful = successful;
        self
    }

    pub fn with_confidence(
        mut self,
        confidence: f32,
    ) -> Self {
        self.confidence =
            confidence.clamp(0.0, 1.0);

        self
    }

    pub fn add_observation(
        &mut self,
        observation: impl Into<String>,
    ) {
        self.observations.push(
            observation.into(),
        );
    }

    pub fn add_improvement(
        &mut self,
        improvement: impl Into<String>,
    ) {
        self.improvements.push(
            improvement.into(),
        );
    }

    pub fn summary(&self) -> String {
        format!(
            "Leçon [{}] succès={} confiance={:.2} \
             observations={} améliorations={}",
            self.source,
            self.successful,
            self.confidence,
            self.observations.len(),
            self.improvements.len(),
        )
    }
}

#[derive(Debug, Default)]
pub struct LearningEngine;

impl LearningEngine {
    pub fn learn(
        observation: &Observation,
    ) -> Learning {
        let mut learning =
            if observation.success {
                Learning::new(
                    &observation.source,
                    "L'action observée a produit un résultat considéré comme réussi.",
                )
                .with_success(true)
                .with_confidence(0.90)
            } else {
                Learning::new(
                    &observation.source,
                    "L'action observée n'a pas produit le résultat attendu.",
                )
                .with_success(false)
                .with_confidence(0.75)
            };

        learning.add_observation(
            observation.result.clone(),
        );

        for change in
            &observation.changes_detected
        {
            learning.add_observation(
                change.clone(),
            );
        }

        if observation.has_anomalies() {
            for anomaly in
                &observation.anomalies
            {
                learning.add_observation(
                    format!(
                        "Anomalie détectée : {}",
                        anomaly
                    ),
                );

                learning.add_improvement(
                    format!(
                        "Analyser et corriger : {}",
                        anomaly
                    ),
                );
            }
        }

        if !observation.success {
            learning.add_improvement(
                "Identifier la cause de l'échec avant de répéter l'action."
                    .to_string(),
            );

            learning.add_improvement(
                "Vérifier les hypothèses utilisées lors de la décision."
                    .to_string(),
            );
        }

        if observation.success
            && observation.anomalies.is_empty()
        {
            learning.add_improvement(
                "Conserver le comportement validé comme référence."
                    .to_string(),
            );
        }

        learning
    }

    pub fn compare(
        previous: &Learning,
        current: &Learning,
    ) -> LearningComparison {
        let confidence_delta =
            current.confidence
                - previous.confidence;

        let success_changed =
            previous.successful
                != current.successful;

        LearningComparison {
            success_changed,
            confidence_delta,
            previous_lesson:
                previous.lesson.clone(),
            current_lesson:
                current.lesson.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LearningComparison {
    pub success_changed: bool,
    pub confidence_delta: f32,
    pub previous_lesson: String,
    pub current_lesson: String,
}

impl LearningComparison {
    pub fn improved(&self) -> bool {
        self.confidence_delta > 0.0
            || (!self.success_changed
                && self.confidence_delta == 0.0)
    }

    pub fn summary(&self) -> String {
        format!(
            "Évolution de l'apprentissage : \
             succès_modifié={}, delta_confiance={:.3}",
            self.success_changed,
            self.confidence_delta,
        )
    }
}
