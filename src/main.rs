use std::path::PathBuf;

use opengl::{entity::sprite::Sprite, window::SDLWindow};
use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");
    window.on_init = Some(game_start);
    window.on_update = Some(game_update);

    let bg1: Sprite = Sprite::new(
        PathBuf::from("assets/PKMN_RS_BG_1.png"),
        (192, 4, 416, 147),
        false,
        false,
    );
    let player: Sprite = Sprite::new(
        PathBuf::from("assets/PKMN_RS_MC_M.png"),
        (8, 7, 22, 28),
        false,
        false,
    );

    let texture_mgr = &mut window.texture_mgr;
    let drawer = &mut window.drawer;
    drawer
        .sprite(texture_mgr, 0, 0, &bg1)
        .sprite(texture_mgr, 50, 70, &player);

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
