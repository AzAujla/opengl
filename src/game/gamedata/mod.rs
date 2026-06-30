use std::path::PathBuf;

use opengl::{draw::animplayer::AnimationPlayer, entity::graphics::spritesheet::SpriteSheet};

use crate::game::{
    assets::spritesheets::{PKMN_BG_1_SPRITESHEET, PkmnBg1Sprites},
    players::male::{
        MalePlayer, animations::get_walking_down_anim, spritesheet::MalePlayerSpriteStates,
    },
};

#[derive(Debug)]
pub struct GameData<'a> {
    background: SpriteSheet<'a, PkmnBg1Sprites>,
    player: MalePlayer<MalePlayerSpriteStates>,
    anim_player: AnimationPlayer,
}

impl<'a> GameData<'a> {
    pub fn new() -> Self {
        let background = SpriteSheet::new(
            PKMN_BG_1_SPRITESHEET,
            PathBuf::from("assets/PKMN_RS_BG_1.png"),
        );
        let player = MalePlayer::default();
        let anim_player = AnimationPlayer::new(get_walking_down_anim(1));

        Self {
            background,
            player,
            anim_player,
        }
    }

    pub fn background(&self) -> &SpriteSheet<'a, PkmnBg1Sprites> {
        &self.background
    }

    pub fn player(&self) -> &MalePlayer<MalePlayerSpriteStates> {
        &self.player
    }

    pub fn anim_player(&self) -> &AnimationPlayer {
        &self.anim_player
    }

    pub fn anim_player_mut(&mut self) -> &mut AnimationPlayer {
        &mut self.anim_player
    }
}

impl<'a> Default for GameData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
