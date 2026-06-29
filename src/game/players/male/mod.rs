use std::hash::Hash;

use crate::game::players::{Player, male::spritesheet::MalePlayerSpriteStates};

pub mod controls;
pub mod spritesheet;

pub struct MalePlayer<T: Hash + Eq + Clone> {
    player: Player<T>,
}

impl<T: Hash + Eq + Clone> MalePlayer<T> {
    pub fn player(&self) -> &Player<T> {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player<T> {
        &mut self.player
    }
}

impl Default for MalePlayer<MalePlayerSpriteStates> {
    fn default() -> Self {
        MalePlayer {
            player: Player::new(MalePlayerSpriteStates::StandingDown),
        }
    }
}
