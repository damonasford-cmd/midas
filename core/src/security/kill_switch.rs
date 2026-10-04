use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone, Default)]
pub struct KillSwitch {
    active: Arc<AtomicBool>,
}

impl KillSwitch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn activate(&self) {
        self.active.store(true, Ordering::SeqCst);
    }

    pub fn deactivate(&self) {
        self.active.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    pub fn can_execute(&self) -> bool {
        !self.is_active()
    }
}
