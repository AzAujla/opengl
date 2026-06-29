use std::{borrow::Cow, path::PathBuf};

use opengl::entity::graphics::sprite::{Sprite, ToSprite};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum MalePlayerSpriteStates {
    StandingLeft,
    WalkingLeft,
    WalkingLeftAlt,
    StandingRight,
    WalkingRight,
    WalkingRightAlt,
    StandingDown,
    WalkingDown,
    WalkingDownAlt,
    StandingUp,
    WalkingUp,
    WalkingUpAlt,
}

impl ToSprite for MalePlayerSpriteStates {
    fn to_sprite(&self) -> std::borrow::Cow<'_, opengl::entity::graphics::sprite::Sprite> {
        Cow::Owned(Sprite::new(
            PathBuf::from("assets/PKMN_RS_MC_M.png"),
            match self {
                Self::StandingLeft => (166, 7, 180, 28),
                Self::WalkingLeft => (184, 7, 198, 28),
                Self::WalkingLeftAlt => (202, 7, 216, 28),
                Self::StandingRight => (112, 7, 126, 28),
                Self::WalkingRight => (129, 7, 143, 28),
                Self::WalkingRightAlt => (146, 7, 160, 28),
                Self::StandingUp => (60, 7, 74, 28),
                Self::WalkingUp => (78, 7, 92, 28),
                Self::WalkingUpAlt => (96, 7, 110, 28),
                Self::StandingDown => (8, 7, 22, 28),
                Self::WalkingDown => (26, 7, 40, 28),
                Self::WalkingDownAlt => (43, 7, 57, 28),
            },
            false,
            false,
        ))
    }
}
