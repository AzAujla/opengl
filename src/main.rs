use opengl::window::SDLWindow;

fn main() {
    let window = SDLWindow::new().unwrap().set_title("Open GL Test");
    window.run();
}
