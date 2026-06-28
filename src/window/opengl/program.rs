use std::{collections::HashMap, path::Path, ptr::null_mut};

use gl::types::{GLchar, GLint, GLuint};

use crate::utils::string::create_empty_cstring_with_len;

use super::shader::{Shader, ShaderType};

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub struct Programs {
    programs: HashMap<usize, Program>,
    textures: HashMap<String, u32>,
    active_shader: ShaderType,
}

impl Programs {
    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> u32 {
        let mut texture_id: u32 = 0;

        let img = image::open(&path)
            .expect("Failed to open sprite file")
            .to_rgba8();
        let (width, height) = img.dimensions();
        let raw_pixels = img.into_raw();

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32, // Internal GPU format
                width as i32,
                height as i32,
                0,
                gl::RGBA, // Format of raw data
                gl::UNSIGNED_BYTE,
                raw_pixels.as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            let err = gl::GetError();
            if err != gl::NO_ERROR {
                eprintln!("GL Error after TexImage2D: 0x{:X}", err);
            }

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.textures
            .insert(path.as_ref().to_string_lossy().into_owned(), texture_id);

        texture_id
    }

    pub fn new(kind: ShaderType) -> Self {
        Self {
            programs: HashMap::new(),
            textures: HashMap::new(),
            active_shader: kind,
        }
    }

    pub fn add_shaders(&mut self, kind: ShaderType, shaders: &[Shader]) -> Result<(), String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            Err(error.to_string_lossy().into_owned())
        } else {
            // let uniform = Uniform::new(id, format!("u_resolution_{}", kind.to_usize()).as_str())?;

            self.programs.insert(kind.to_usize(), Program { id });
            Ok(())
        }
    }

    pub fn set(&self, kind: ShaderType) {
        if let Some(id) = self.programs.get(&kind.to_usize()) {
            unsafe {
                gl::UseProgram(id.id().to_owned());
            }
        } else {
            println!("Shader not Found! type: {}", kind.to_usize());
        }
    }

    pub fn id(&self, kind: ShaderType) -> Option<u32> {
        self.programs.get(&kind.to_usize()).map(|p| p.id())
    }
}
impl Drop for Programs {
    fn drop(&mut self) {
        for p_id in self.programs.values() {
            unsafe {
                gl::DeleteProgram(p_id.id());
            }
        }
    }
}
