use std::collections::VecDeque;

use super::memory_id::MemoryId;

#[derive(Debug, Clone)]
pub struct WorkingMemoryItem {
    pub memory_id: MemoryId,
    pub priority: u8,
}

#[derive(Debug)]
pub struct WorkingMemory {
    capacity: usize,
    items: VecDeque<WorkingMemoryItem>,
}

impl WorkingMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            items: VecDeque::new(),
        }
    }

    pub fn push(
        &mut self,
        item: WorkingMemoryItem,
    ) {
        self.items.push_back(item);

        while self.items.len() > self.capacity {
            self.items.pop_front();
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &WorkingMemoryItem> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
