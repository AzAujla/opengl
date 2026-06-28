use opengl::window::SDLWindow;

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");
    window
        .drawer_mut()
        .square_fill(10, 10, 50, sdl2::pixels::Color::RED);
    window.run();
}
