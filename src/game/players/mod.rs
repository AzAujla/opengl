use std::hash::Hash;

pub mod male;

#[derive(Debug)]
pub struct Player<T: Hash + Eq + Clone> {
    start_state: T,
    current_state: T,
}

impl<T: Hash + Eq + Clone> Player<T> {
    pub fn new(start_state: T) -> Self {
        Self {
            start_state: start_state.clone(),
            current_state: start_state,
        }
    }

    pub fn start_state(&self) -> &T {
        &self.start_state
    }

    pub fn set_start_state(&mut self, start_state: T) {
        self.start_state = start_state;
    }

    pub fn current_state(&self) -> &T {
        &self.current_state
    }

    pub fn set_current_state(&mut self, current_state: T) {
        self.current_state = current_state;
    }
}
