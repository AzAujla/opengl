use std::path::PathBuf;

pub struct Sprite {
    path: PathBuf,
    texture_id: Option<u32>,
    uv: (u32, u32, u32, u32),
    flipped_h: bool,
    flipped_v: bool,
}

impl Sprite {
    pub fn new(path: PathBuf, uv: (u32, u32, u32, u32), flipped_h: bool, flipped_v: bool) -> Self {
        Self {
            path,
            texture_id: None,
            uv,
            flipped_h,
            flipped_v,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn texture_id(&self) -> Option<u32> {
        self.texture_id
    }

    pub fn uv(&self) -> (u32, u32, u32, u32) {
        self.uv
    }

    pub fn flipped_h(&self) -> bool {
        self.flipped_h
    }

    pub fn flipped_v(&self) -> bool {
        self.flipped_v
    }

    pub fn set_texture_id(&mut self, texture_id: Option<u32>) {
        self.texture_id = texture_id;
    }
}
