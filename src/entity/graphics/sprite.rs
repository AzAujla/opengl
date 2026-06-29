use std::{borrow::Cow, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Sprite {
    path: PathBuf,
    uv: (u32, u32, u32, u32),
    flipped_h: bool,
    flipped_v: bool,
}

pub trait ToSprite {
    fn to_sprite(&self) -> Cow<'_, Sprite>;
}

impl ToSprite for Sprite {
    fn to_sprite(&self) -> Cow<'_, Sprite> {
        Cow::Borrowed(self)
    }
}

impl Sprite {
    pub fn new(path: PathBuf, uv: (u32, u32, u32, u32), flipped_h: bool, flipped_v: bool) -> Self {
        Self {
            path,
            uv,
            flipped_h,
            flipped_v,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
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
}
