use opengl::entity::graphics::spritesheet::SpriteLookupArray;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PkmnBg1Sprites {
    Room1,
    Gym,
    Room2,
    Room3,
    Office1,
    Office2,
    Office3,
    Umbrella1,
    Umbrella2,
    Umbrella3,
    Chair,
}

pub const PKMN_BG_1_SPRITESHEET: &SpriteLookupArray<PkmnBg1Sprites> = &[
    (PkmnBg1Sprites::Room1, (192, 4, 416, 147, false, false)),
    (PkmnBg1Sprites::Gym, (6, 5, 182, 292, false, false)),
    (PkmnBg1Sprites::Room2, (193, 154, 417, 298, false, false)),
    (PkmnBg1Sprites::Room3, (13, 305, 237, 449, false, false)),
    (PkmnBg1Sprites::Office1, (569, 312, 873, 456, false, false)),
    (PkmnBg1Sprites::Office2, (624, 153, 928, 267, false, false)),
    (PkmnBg1Sprites::Office3, (250, 310, 554, 454, false, false)),
    (PkmnBg1Sprites::Umbrella1, (455, 69, 487, 106, false, false)),
    (PkmnBg1Sprites::Umbrella2, (535, 69, 567, 106, false, false)),
    (PkmnBg1Sprites::Umbrella2, (574, 69, 606, 106, false, false)),
    (PkmnBg1Sprites::Chair, (492, 73, 529, 105, false, false)),
];
