//! MIDAS CORE
//! L'intelligence unifiée.
//! Les 4 noyaux et les 3 principes sont définis ici.

use serde::{Deserialize, Serialize};
use tracing::info;

/// Les 3 principes innés de MIDAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Principe {
    HautNiveau,
    HautDeGamme,
    HautePrecision,
}

/// Les 4 noyaux innés de MIDAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Noyau {
    LectureEcriture,
    Execution,
    Reflexion,
    Creation,
}

/// Les phases de la boucle fondamentale de MIDAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Phase {
    Percevoir,
    Comprendre,
    Raisonner,
    Decider,
    Agir,
    Observer,
    Apprendre,
    Ameliorer,
}

/// L'état interne du Core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtatCore {
    pub phase_actuelle: Phase,
    pub objectif: Option<String>,
    pub capacites_disponibles: Vec<String>,
    pub capacites_manquantes: Vec<String>,
}

impl Default for EtatCore {
    fn default() -> Self {
        Self {
            phase_actuelle: Phase::Percevoir,
            objectif: None,
            capacites_disponibles: Vec::new(),
            capacites_manquantes: Vec::new(),
        }
    }
}

/// Le Core unifié de MIDAS.
pub struct MidasCore {
    pub etat: EtatCore,
    pub principes: Vec<Principe>,
    pub noyaux: Vec<Noyau>,
}

impl MidasCore {
    /// Crée un nouveau Core avec les principes et noyaux innés.
    pub fn new() -> Self {
        Self {
            etat: EtatCore::default(),

            principes: vec![
                Principe::HautNiveau,
                Principe::HautDeGamme,
                Principe::HautePrecision,
            ],

            noyaux: vec![
                Noyau::LectureEcriture,
                Noyau::Execution,
                Noyau::Reflexion,
                Noyau::Creation,
            ],
        }
    }

    /// Point d'entrée de la boucle fondamentale.
    pub fn executer_boucle(&mut self, objectif: &str) -> anyhow::Result<()> {
        info!(
            "MIDAS Core: début de la boucle pour '{}'",
            objectif
        );

        self.etat.objectif = Some(objectif.to_string());
        self.etat.phase_actuelle = Phase::Percevoir;

        // La boucle sera développée progressivement.
        // Pour l'instant, nous validons la structure du Core.

        info!(
            "MIDAS Core: fin de la boucle pour '{}'",
            objectif
        );

        Ok(())
    }

    /// Détecte une capacité manquante.
    pub fn detecter_capacite_manquante(&mut self, capacite: &str) {
        let capacite = capacite.to_string();

        if !self
            .etat
            .capacites_disponibles
            .contains(&capacite)
            && !self
                .etat
                .capacites_manquantes
                .contains(&capacite)
        {
            self.etat.capacites_manquantes.push(capacite.clone());

            info!(
                "Capacité manquante détectée: {}",
                capacite
            );
        }
    }
}

impl Default for MidasCore {
    fn default() -> Self {
        Self::new()
    }
}
