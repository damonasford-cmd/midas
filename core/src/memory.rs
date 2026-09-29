//! MÉMOIRE
//! Mémoire persistante de MIDAS.
//! Format : JSONL (une ligne par événement).

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct MemoryEvent {
    pub id: u64,
    pub timestamp: u64,
    pub cycle: u64,
    pub role: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl MemoryEvent {
    pub fn new(id: u64, cycle: u64, role: impl Into<String>, content: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            id,
            timestamp,
            cycle,
            role: role.into(),
            content: content.into(),
            tags: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

#[derive(Debug)]
pub struct Memory {
    events: Vec<MemoryEvent>,
    next_id: u64,
    path: PathBuf,
}

impl Memory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let mut mem = Self {
            events: Vec::new(),
            next_id: 1,
            path,
        };
        mem.load();
        mem
    }

    /// Enregistre un événement. Retourne son ID.
    pub fn record(
        &mut self,
        cycle: u64,
        role: impl Into<String>,
        content: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let event = MemoryEvent::new(id, cycle, role, content);
        self.events.push(event);
        self.persist_last();
        id
    }

    /// Recherche par mot-clé dans le contenu.
    pub fn search(&self, keyword: &str) -> Vec<&MemoryEvent> {
        self.events
            .iter()
            .filter(|e| e.content.contains(keyword))
            .collect()
    }

    /// Recherche par tag.
    pub fn search_by_tag(&self, tag: &str) -> Vec<&MemoryEvent> {
        self.events
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Retourne les N derniers événements.
    pub fn recent(&self, n: usize) -> Vec<&MemoryEvent> {
        let start = if self.events.len() > n {
            self.events.len() - n
        } else {
            0
        };
        self.events[start..].iter().collect()
    }

    /// Retourne le contexte récent sous forme de texte lisible.
    pub fn recent_context(&self, n: usize) -> String {
        self.recent(n)
            .iter()
            .map(|e| format!("[{}] {}: {}", e.id, e.role, e.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    fn load(&mut self) {
        if !self.path.exists() {
            return;
        }
        if let Ok(file) = File::open(&self.path) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                if let Some(event) = Self::parse_line(&line) {
                    if event.id >= self.next_id {
                        self.next_id = event.id + 1;
                    }
                    self.events.push(event);
                }
            }
        }
    }

    fn parse_line(line: &str) -> Option<MemoryEvent> {
        // Format: id|timestamp|cycle|role|content|tags
        let parts: Vec<&str> = line.splitn(6, '|').collect();
        if parts.len() < 5 {
            return None;
        }
        let id = parts[0].parse().ok()?;
        let timestamp = parts[1].parse().ok()?;
        let cycle = parts[2].parse().ok()?;
        let role = parts[3].to_string();
        let content = parts[4].to_string();
        let tags = if parts.len() >= 6 && !parts[5].is_empty() {
            parts[5].split(',').map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        };
        Some(MemoryEvent {
            id,
            timestamp,
            cycle,
            role,
            content,
            tags,
        })
    }

    fn persist_last(&self) {
        if let Some(event) = self.events.last() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
            {
                let tags = event.tags.join(",");
                let _ = writeln!(
                    file,
                    "{}|{}|{}|{}|{}|{}",
                    event.id, event.timestamp, event.cycle, event.role, event.content, tags
                );
            }
        }
    }
}
