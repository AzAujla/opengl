use std::hash::Hash;

use sdl2::keyboard::{KeyboardState, Scancode};

pub mod male;

#[derive(Debug)]
pub struct Player<T: Hash + Eq + Clone> {
    start_state: T,
    current_state: T,
    position: (u32, u32),
    direction: u8,
    walk_pressed: u8,
    walk_friction: u8,
}

impl<T: Hash + Eq + Clone> Player<T> {
    pub fn new(start_state: T) -> Self {
        Self {
            start_state: start_state.clone(),
            current_state: start_state,
            position: (0, 0),
            direction: 4,
            walk_pressed: 0,
            walk_friction: 8,
        }
    }

    pub fn walk(&mut self, keystate: &KeyboardState, has_moved: &mut bool) {
        if keystate.is_scancode_pressed(Scancode::W) {
            self.walk_pressed += 1;
            *has_moved = true;
            if self.walk_pressed > self.walk_friction {
                self.walk_pressed = 0;
                self.sub_position((0, 1));
                self.set_direction(1);
            }
        } else if keystate.is_scancode_pressed(Scancode::S) {
            self.walk_pressed += 1;
            *has_moved = true;
            if self.walk_pressed > self.walk_friction {
                self.walk_pressed = 0;
                self.add_position((0, 1));
                self.set_direction(4);
            }
        } else if keystate.is_scancode_pressed(Scancode::A) {
            self.walk_pressed += 1;
            *has_moved = true;
            if self.walk_pressed > self.walk_friction {
                self.walk_pressed = 0;
                self.sub_position((1, 0));
                self.set_direction(2);
            }
        } else if keystate.is_scancode_pressed(Scancode::D) {
            self.walk_pressed += 1;
            *has_moved = true;
            if self.walk_pressed > self.walk_friction {
                self.walk_pressed = 0;
                self.add_position((1, 0));
                self.set_direction(3);
            }
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

    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    pub fn set_position(&mut self, position: (u32, u32)) {
        self.position = position;
    }

    pub fn add_position(&mut self, position: (u32, u32)) {
        self.position.0 = self.position.0.saturating_add(position.0);
        self.position.1 = self.position.1.saturating_add(position.1);
    }

    pub fn sub_position(&mut self, position: (u32, u32)) {
        self.position.0 = self.position.0.saturating_sub(position.0);
        self.position.1 = self.position.1.saturating_sub(position.1);
    }

    pub fn set_direction(&mut self, direction: u8) {
        self.direction = direction;
    }

    pub fn direction(&self) -> u8 {
        self.direction
    }

    pub fn set_walk_friction(&mut self, walk_friction: u8) {
        self.walk_friction = walk_friction;
    }
}
