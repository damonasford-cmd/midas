use anyhow::{Context, Result};
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use super::types::Memory;

pub struct MemoryStore {
    path: PathBuf,
}

impl MemoryStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| {
                    format!(
                        "Impossible de créer le dossier mémoire {:?}",
                        parent
                    )
                })?;
        }

        if !path.exists() {
            fs::File::create(&path)
                .with_context(|| {
                    format!(
                        "Impossible de créer le fichier mémoire {:?}",
                        path
                    )
                })?;
        }

        Ok(Self { path })
    }

    pub fn write(&self, memory: &Memory) -> Result<()> {
        let serialized = serde_json::to_string(memory)
            .context("Impossible de sérialiser la mémoire")?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| {
                format!(
                    "Impossible d'ouvrir le fichier mémoire {:?}",
                    self.path
                )
            })?;

        writeln!(file, "{serialized}")?;
        file.flush()?;

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<Memory>> {
        let file = fs::File::open(&self.path)
            .with_context(|| {
                format!(
                    "Impossible de lire le fichier mémoire {:?}",
                    self.path
                )
            })?;

        let reader = BufReader::new(file);
        let mut memories = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let memory =
                serde_json::from_str::<Memory>(&line)
                    .context("Entrée mémoire invalide")?;

            memories.push(memory);
        }

        Ok(memories)
    }

    pub fn count(&self) -> Result<usize> {
        Ok(self.read_all()?.len())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
