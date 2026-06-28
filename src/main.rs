use opengl::window::SDLWindow;

fn main() {
    let mut window = SDLWindow::new().unwrap().set_title("Open GL Test");
    window.run();
}
