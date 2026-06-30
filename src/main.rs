use opengl::window::SDLWindow;
use sdl2::{event::Event, keyboard::KeyboardState};

use crate::game::{
    assets::spritesheets::PkmnBg1Sprites,
    gamedata::GameData,
    players::male::animations::{get_standing_anim, get_walking_anim},
};

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
    window
        .data_mut()
        .player_mut()
        .player_mut()
        .set_position((60, 60));
}

fn game_update(
    window: &mut SDLWindow<GameData>,
    events: Vec<Event>,
    keystate: KeyboardState,
    delta: f64,
) {
    let mut has_moved: bool = false;
    for event in events {
        if let Event::KeyDown {
            keycode: Some(keycode),
            ..
        } = event
        {
            {
                println!(
                    "{} pressed! Time slice duration: {}s with window size: {}x{}",
                    keycode.name(),
                    delta,
                    window.window_size.0,
                    window.window_size.1
                );
            }
        }
    }

    window
        .data
        .player_mut()
        .player_mut()
        .walk(&keystate, &mut has_moved);

    let direction = window.data.player().player().direction();

    let data = &mut window.data;
    let drawer = &mut window.drawer;
    let anim_player = &mut data.anim_player;

    if has_moved {
        anim_player.set_animation(get_walking_anim(direction));
    } else {
        anim_player.set_animation(get_standing_anim(direction));
    }

    data.anim_player_mut().update(delta);

    drawer
        .sprite(
            0,
            0,
            &data.background().get_sprite(&PkmnBg1Sprites::Office3),
        )
        .sprite(
            data.player().player().position().0,
            data.player().player().position().1,
            data.anim_player_mut().get_current_sprite(),
        );
}
