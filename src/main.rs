use opengl::window::SDLWindow;
use sdl2::{event::Event, keyboard::Keycode};

use crate::game::{assets::spritesheets::PkmnBg1Sprites, gamedata::GameData};

pub mod game;

fn main() {
    let mut window = SDLWindow::new(GameData::new())
        .unwrap()
        .set_title("Open GL Test");
    window.on_init = Some(game_start);
    window.on_update = Some(game_update);

    window.run();
}

fn game_start(window: &mut SDLWindow<GameData>) {
    println!(
        "Game Window Start Res: {}x{}",
        window.logical_size.0, window.logical_size.1
    );
    window.data_mut().anim_player_mut().play();
}

fn game_update(window: &mut SDLWindow<GameData>, events: Vec<Event>, delta: f64) {
    for event in events {
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

    let data = &mut window.data;
    let drawer = &mut window.drawer;

    data.anim_player_mut().update(delta);

    drawer
        .sprite(
            0,
            0,
            &data.background().get_sprite(&PkmnBg1Sprites::Office3),
        )
        .sprite(
            45,
            35,
            &data.background().get_sprite(&PkmnBg1Sprites::Chair),
        )
        .sprite(
            20,
            20,
            &data.background().get_sprite(&PkmnBg1Sprites::Umbrella2),
        )
        .sprite(60, 60, data.anim_player_mut().get_current_sprite())
        .sprite(50, 70, data.player().player().current_state());
}
