use std::path::PathBuf;

use opengl::{entity::graphics::spritesheet::SpriteSheet, window::SDLWindow};
use sdl2::{event::Event, keyboard::Keycode};

use crate::game::{
    assets::spritesheets::{PKMN_BG_1_SPRITESHEET, PkmnBg1Sprites},
    players::male::MalePlayer,
};

pub mod game;

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");
    window.on_init = Some(game_start);
    window.on_update = Some(game_update);

    let bg1 = SpriteSheet::new(
        PKMN_BG_1_SPRITESHEET,
        PathBuf::from("assets/PKMN_RS_BG_1.png"),
    );
    let player = MalePlayer::default();

    let drawer = &mut window.drawer;
    drawer
        .sprite(0, 0, &bg1.get_sprite(&PkmnBg1Sprites::Office3))
        .sprite(45, 35, &bg1.get_sprite(&PkmnBg1Sprites::Chair))
        .sprite(20, 20, &bg1.get_sprite(&PkmnBg1Sprites::Umbrella2))
        .sprite(50, 70, &player.player().current_state().clone());

    window.run();
}

fn game_start(window: &mut SDLWindow) {
    println!(
        "Game Window Start Res: {}x{}",
        window.logical_size.0, window.logical_size.1
    );
}

fn game_update(window: &mut SDLWindow, event: Event, delta: f64) {
    if let Event::KeyDown {
        keycode: Some(Keycode::Space),
        ..
    } = event
    {
        println!(
            "Spacebar pressed! Time slice duration: {}s with window size: {}x{}",
            delta, window.window_size.0, window.window_size.1
        );
    }
}
