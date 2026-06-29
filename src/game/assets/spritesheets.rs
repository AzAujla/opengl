use opengl::entity::graphics::spritesheet::SpriteLookupArray;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PkmnBg1Sprites {
    Room1,
}

pub const PKMN_BG_1_SPRITESHEET: &SpriteLookupArray<PkmnBg1Sprites> =
    &[(PkmnBg1Sprites::Room1, (192, 4, 416, 147, false, false))];
