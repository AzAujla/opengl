use cgmath::{Matrix, Matrix4, ortho};
use gl::types::GLsizei;
use sdl2::{
    Sdl,
    event::Event,
    video::{GLContext, GLProfile, SwapInterval, Window},
};

use crate::{
    draw::{Draw, vertex::Vertex},
    window::opengl::{
        create_program, ibo::Ibo, program::Programs, shader::ShaderType, vao::Vao, vbo::Vbo,
    },
};

pub mod opengl;

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

    pub drawer: Draw,

    pub on_init: Option<fn(&mut SDLWindow)>,
    pub on_update: Option<fn(&mut SDLWindow, Event, f64)>,
}

impl SDLWindow {
    /**
     * Creates a default game window of logical size 240x160
     * And Window Size of 800x600
     */
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem
            .window("New Window", 800, 600)
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context()?;
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
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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
            drawer: Draw::new((240, 160), 16, 5),

            on_init: None,
            on_update: None,
        })
    }

    fn get_viewport_rect(&self) -> (i32, i32, i32, i32) {
        let (logical_w, logical_h) = (self.logical_size.0 as f32, self.logical_size.1 as f32);
        let (window_w, window_h) = (self.window_size.0 as f32, self.window_size.1 as f32);

        let target_aspect = logical_w / logical_h;

        // Try fitting by width first
        let mut view_w = window_w;
        let mut view_h = view_w / target_aspect;

        // If height ends up too tall for the window, fit by height instead
        if view_h > window_h {
            view_h = window_h;
            view_w = view_h * target_aspect;
        }

        // Center the viewport inside the window
        let view_x = ((window_w - view_w) / 2.0) as i32;
        let view_y = ((window_h - view_h) / 2.0) as i32;

        (view_x, view_y, view_w as i32, view_h as i32)
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.window.set_title(title).unwrap();
        self
    }

    pub fn set_size(mut self, w: u32, h: u32) -> Self {
        self.window.set_size(w, h).unwrap();
        self
    }

    pub fn set_logical_size(mut self, w: u32, h: u32) -> Self {
        self.logical_size = (w, h);
        self.drawer.set_logical_size(w, h);
        self
    }

    pub fn set_sprite_size(mut self, sprite_size: u32) -> Self {
        self.drawer.set_sprite_size(sprite_size);
        self
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        if let Some(init) = self.on_init {
            init(self);
        }

        let mut start = std::time::Instant::now();

        'running: loop {
            let end = std::time::Instant::now();
            let delta = (end - start).as_secs_f64();
            start = end;

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    sdl2::event::Event::Window {
                        win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
                        ..
                    } => {
                        self.window_size = (w as u32, h as u32);
                    }
                    _ => (),
                }

                if let Some(update) = self.on_update {
                    update(self, event, delta)
                }
            }

            // Generate Static IBO
            let draw_logical_size = self.drawer.logical_size();
            let draw_sprite_size = self.drawer.sprite_size();
            let max_quads = (((draw_logical_size.0 / draw_sprite_size) + 2)
                * ((draw_logical_size.1 / draw_sprite_size) + 2))
                * self.drawer.layers();
            let indices_per_quad = 6;

            let mut indices = Vec::with_capacity((indices_per_quad * max_quads) as usize);
            let mut offset: u32 = 0;

            for _ in 0..max_quads {
                indices.push(offset);
                indices.push(offset + 1);
                indices.push(offset + 2);

                indices.push(offset + 2);
                indices.push(offset + 3);
                indices.push(offset);

                offset += 4;
            }

            let proj: Matrix4<f32> = ortho(0.0, 240.0, 160.0, 0.0, -1.0, 1.0);
            let (vx, vy, vw, vh) = self.get_viewport_rect();
            let index_count = ((self.drawer.vertices().len() / 4) * 6) as i32;

            unsafe {
                self.programs.set(ShaderType::Pixel);
                self.drawer.texture_mgr().bind();
                let location = gl::GetUniformLocation(
                    self.programs.id(ShaderType::Pixel).unwrap(),
                    c"u_Projection".as_ptr() as *const _,
                );

                gl::UniformMatrix4fv(location, 1, gl::FALSE, proj.as_ptr());

                self.vao.set();
                self.vbo.set();

                self.vao.setup();
                self.ibo.bind();

                let max_vertices = max_quads * 4;
                let total_vbo_bytes =
                    (max_vertices * std::mem::size_of::<Vertex>() as u32) as gl::types::GLsizeiptr;
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    total_vbo_bytes,
                    std::ptr::null(),
                    gl::DYNAMIC_DRAW,
                );

                let initial_vertex_bytes =
                    std::mem::size_of_val(self.drawer.vertices()) as gl::types::GLsizeiptr;
                if initial_vertex_bytes > 0 {
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        initial_vertex_bytes,
                        self.drawer.vertices().as_ptr() as *const _,
                    );
                }

                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                    indices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );

                gl::Viewport(0, 0, self.window_size.0 as i32, self.window_size.1 as i32);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Viewport(vx, vy, vw, vh);

                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Enable(gl::SCISSOR_TEST);
                gl::Scissor(vx, vy, vw, vh);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::DrawElements(
                    gl::TRIANGLES,
                    index_count,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::Disable(gl::SCISSOR_TEST);

                gl::BindVertexArray(0);
            }

            self.window.gl_swap_window();
        }
    }

    pub fn drawer_mut(&mut self) -> &mut Draw {
        &mut self.drawer
    }

    pub fn drawer(&self) -> &Draw {
        &self.drawer
    }
}
