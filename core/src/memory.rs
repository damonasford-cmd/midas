use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryKind {
    Working,
    Episodic,
    Semantic,
    Procedural,
    Decision,
    Error,
    Learning,
    Multimodal,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: MemoryKind,
    pub source: String,
    pub content: String,
    pub importance: f32,
    pub tags: Vec<String>,
}

impl Memory {
    pub fn new(
        kind: MemoryKind,
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            kind,
            source: source.into(),
            content: content.into(),
            importance: 1.0,
            tags: Vec::new(),
        }
    }
}

pub struct MemoryStore {
    path: PathBuf,
}

impl MemoryStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Impossible de créer {:?}", parent))?;
        }

        if !path.exists() {
            fs::File::create(&path)
                .with_context(|| format!("Impossible de créer {:?}", path))?;
        }

        Ok(Self { path })
    }

    pub fn write(&self, memory: &Memory) -> Result<()> {
        let serialized =
            serde_json::to_string(memory).context("Impossible de sérialiser la mémoire")?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("Impossible d'ouvrir {:?}", self.path))?;

        writeln!(file, "{serialized}")?;

        file.flush()?;

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<Memory>> {
        let file = fs::File::open(&self.path)
            .with_context(|| format!("Impossible de lire {:?}", self.path))?;

        let reader = BufReader::new(file);

        let mut memories = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let memory: Memory =
                serde_json::from_str(&line).context("Entrée mémoire invalide")?;

            memories.push(memory);
        }

        Ok(memories)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Memory>> {
        let query = query.to_lowercase();

        let memories = self.read_all()?;

        Ok(memories
            .into_iter()
            .filter(|memory| {
                memory.content.to_lowercase().contains(&query)
                    || memory.source.to_lowercase().contains(&query)
                    || memory
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect())
    }

    pub fn count(&self) -> Result<usize> {
        Ok(self.read_all()?.len())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
