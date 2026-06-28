use std::path::PathBuf;

use opengl::{entity::sprite::Sprite, window::SDLWindow};

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");

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
