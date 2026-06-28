use std::{collections::HashMap, ptr::null_mut};

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
    active_shader: ShaderType,
}

impl Programs {
    pub fn new(kind: ShaderType) -> Self {
        Self {
            programs: HashMap::new(),
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
