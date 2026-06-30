use opengl::entity::graphics::{animation::Animation, sprite::ToSprite};

use crate::game::players::male::spritesheet::MalePlayerSpriteStates;

pub fn get_walking_down_anim(dir: u8) -> Animation {
    let sprites = match dir {
        1 => vec![
            MalePlayerSpriteStates::WalkingUp.to_sprite().into_owned(),
            MalePlayerSpriteStates::WalkingUpAlt
                .to_sprite()
                .into_owned(),
        ],
        2 => vec![
            MalePlayerSpriteStates::WalkingLeft.to_sprite().into_owned(),
            MalePlayerSpriteStates::WalkingLeftAlt
                .to_sprite()
                .into_owned(),
        ],
        3 => vec![
            MalePlayerSpriteStates::WalkingRight
                .to_sprite()
                .into_owned(),
            MalePlayerSpriteStates::WalkingRightAlt
                .to_sprite()
                .into_owned(),
        ],
        _ => vec![
            MalePlayerSpriteStates::WalkingDown.to_sprite().into_owned(),
            MalePlayerSpriteStates::WalkingDownAlt
                .to_sprite()
                .into_owned(),
        ],
    };
    Animation::new("walking", sprites, 4, true)
}
