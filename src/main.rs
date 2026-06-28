use std::path::PathBuf;

use opengl::{entity::sprite::Sprite, window::SDLWindow};

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");

    let mut bg: Sprite = Sprite::new(
        PathBuf::from("assets/PKMN_RS_BG_1.png"),
        (192, 4, 416, 297),
        false,
        false,
    );

    let bg_layer_id = window.texture_mgr.get_or_load_layer(bg.path());
    bg.set_texture_id(Some(bg_layer_id));

    window
        .drawer_mut()
        .square_fill(10, 10, 50, sdl2::pixels::Color::RED)
        .sprite(20, 20, &bg);

    window.run();
}
