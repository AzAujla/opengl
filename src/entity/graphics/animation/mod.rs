use crate::entity::graphics::sprite::Sprite;

#[derive(Debug)]
pub struct Animation {
    name: String,
    timeline: Vec<Sprite>,
    frames_per_second: u16,
    is_looped: bool,
}

impl Animation {
    pub fn new<S: ToString>(
        name: S,
        timeline: Vec<Sprite>,
        frames_per_second: u16,
        is_looped: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            timeline,
            frames_per_second,
            is_looped,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_timeline(&mut self, timeline: Vec<Sprite>) {
        self.timeline = timeline;
    }

    pub fn timeline(&self) -> &[Sprite] {
        &self.timeline
    }

    pub fn frames_per_second(&self) -> u16 {
        self.frames_per_second
    }

    pub fn set_frames_per_second(&mut self, frames_per_second: u16) {
        self.frames_per_second = frames_per_second;
    }

    pub fn is_looped(&self) -> bool {
        self.is_looped
    }

    pub fn set_is_looped(&mut self, is_looped: bool) {
        self.is_looped = is_looped;
    }
}
