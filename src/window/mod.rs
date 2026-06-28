use gl::types::GLsizei;
use sdl2::{
    Sdl,
    video::{GLContext, GLProfile, SwapInterval, Window},
};

use crate::window::opengl::{create_program, ibo::Ibo, program::Programs, vao::Vao, vbo::Vbo};

mod opengl;

pub struct SDLWindow {
    sdl_context: Sdl,
    window: Window,

    pub logical_size: (u32, u32),
    pub window_size: (u32, u32),

    gl_context: GLContext,

    pub programs: Programs,

    pub vbo: Vbo,
    pub ibo: Ibo,
    pub ibo_len: GLsizei,
    pub vao: Vao,
}

impl SDLWindow {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem
            .window("New Window", 800, 600)
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        extern "system" fn debug_callback(
            source: u32,
            gltype: u32,
            id: u32,
            severity: u32,
            _length: i32,
            message: *const i8,
            _user_param: *mut std::ffi::c_void,
        ) {
            unsafe {
                let msg = std::ffi::CStr::from_ptr(message).to_string_lossy();
                let source_str = match source {
                    gl::DEBUG_SOURCE_API => "API",
                    gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
                    gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
                    gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
                    gl::DEBUG_SOURCE_APPLICATION => "Application",
                    gl::DEBUG_SOURCE_OTHER => "Other",
                    _ => "Unknown",
                };

                let type_str = match gltype {
                    gl::DEBUG_TYPE_ERROR => "Error",
                    gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
                    gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
                    gl::DEBUG_TYPE_PORTABILITY => "Portability",
                    gl::DEBUG_TYPE_PERFORMANCE => "Performance",
                    gl::DEBUG_TYPE_MARKER => "Marker",
                    gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
                    gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
                    gl::DEBUG_TYPE_OTHER => "Other",
                    _ => "Unknown",
                };

                let severity_str = match severity {
                    gl::DEBUG_SEVERITY_HIGH => "High",
                    gl::DEBUG_SEVERITY_MEDIUM => "Medium",
                    gl::DEBUG_SEVERITY_LOW => "Low",
                    gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
                    _ => "Unknown",
                };

                println!(
                    "[GL DEBUG] Source: {}, Type: {}, ID: {}, Severity: {}, Message: {}",
                    source_str, type_str, id, severity_str, msg
                );
            }
        }

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS); // ensures messages are delivered immediately
            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                std::ptr::null(),
                gl::TRUE,
            );
        }

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)?;

        let programs = create_program()?;

        let vbo = Vbo::new();
        let vao = Vao::new();
        let ibo = Ibo::new();

        Ok(Self {
            sdl_context,
            window,
            logical_size: (240, 160),
            window_size: (800, 600),
            gl_context,
            programs,
            vbo,
            ibo,
            ibo_len: 0,
            vao,
        })
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.window.set_title(title).unwrap();
        self
    }

    pub fn set_size(mut self, w: u32, h: u32) -> Self {
        self.window.set_size(w, h).unwrap();
        self
    }

    pub fn run(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'running;
                }
            }

            unsafe {
                gl::ClearColor(100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.gl_swap_window();
        }
    }
}
