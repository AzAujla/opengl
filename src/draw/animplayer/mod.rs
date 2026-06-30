use std::time::Instant;

use crate::entity::graphics::{animation::Animation, sprite::Sprite};

#[derive(Debug)]
pub struct AnimationPlayer {
    animation: Animation,
    start_time: Instant,
    elapsed_time: f64,
    is_playing: bool,
    total_time: f64,
}

impl AnimationPlayer {
    pub fn new(animation: Animation) -> Self {
        let now = Instant::now();
        let total_frames = animation.timeline().len();
        if total_frames == 0 {
            panic!("Animation has no frames!");
        }
        let total_time = total_frames as f64 / animation.frames_per_second() as f64;
        Self {
            animation,
            start_time: now,
            elapsed_time: 0.0,
            is_playing: false,
            total_time,
        }
    }
    pub fn animation(&self) -> &Animation {
        &self.animation
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn animation_mut(&mut self) -> &mut Animation {
        &mut self.animation
    }

    pub fn set_animation(&mut self, animation: Animation) {
        if animation.timeline().is_empty() {
            panic!("Animation has no frames!");
        }

        self.animation = animation;
    }

    pub fn anim_start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn playpause(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.elapsed_time = 0.0;
    }

    pub fn update(&mut self, delta: f64) {
        self.elapsed_time += delta;
    }

    pub fn get_current_sprite(&mut self) -> &Sprite {
        let total_frames = self.animation.timeline().len();

        if self.elapsed_time >= self.total_time {
            self.is_playing = self.animation.is_looped();
            if self.animation.is_looped() {
                self.reset();
            }
            return self.animation.timeline().get(total_frames - 1).unwrap();
        }

        let mut current_frame =
            (self.elapsed_time / self.total_time * (total_frames as f64)) as usize;
        if current_frame >= total_frames {
            current_frame = total_frames - 1;
        }

        self.animation.timeline().get(current_frame).unwrap()
    }
}
