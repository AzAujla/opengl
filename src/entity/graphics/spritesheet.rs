use std::{borrow::Cow, hash::Hash, path::PathBuf};

use crate::entity::graphics::sprite::{Sprite, ToSprite};

pub type SpriteLookupArray<T> = [(T, (u16, u16, u16, u16, bool, bool))];

pub struct SpriteSheet<'b, T: Hash + Eq> {
    pub sprites: &'b SpriteLookupArray<T>,
    pub path: PathBuf,
}

pub struct SpriteSheetMapper<'a, 'b, T: Hash + Eq> {
    pub spritesheet: &'a SpriteSheet<'b, T>,
    pub index: &'a T,
}

impl<T: Hash + Eq + Clone> ToSprite for SpriteSheetMapper<'_, '_, T> {
    fn to_sprite(&self) -> std::borrow::Cow<'_, Sprite> {
        let coords = self
            .spritesheet
            .sprites
            .iter()
            .find(|(key, _)| key == self.index)
            .map(|(_, values)| values)
            .expect("Requested sprite key configuration missing from constant slice list!");

        Cow::Owned(Sprite::new(
            self.spritesheet.path.clone(),
            (
                coords.0 as u32,
                coords.1 as u32,
                coords.2 as u32,
                coords.3 as u32,
            ),
            coords.4,
            coords.5,
        ))
    }
}

impl<'b, T: Hash + Eq + Clone> SpriteSheet<'b, T> {
    pub fn new(sprites: &'b SpriteLookupArray<T>, path: PathBuf) -> Self {
        Self { sprites, path }
    }

    pub fn get_sprite<'a>(&'a self, index: &'a T) -> SpriteSheetMapper<'a, 'b, T> {
        SpriteSheetMapper {
            index,
            spritesheet: self,
        }
    }
}
