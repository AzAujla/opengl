use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

use gl::types::{GLchar, GLenum, GLint, GLuint};

use crate::utils::string::create_empty_cstring_with_len;

#[derive(Clone)]
pub enum ShaderType {
    BasicFlatShader,
    TexShader,
}

impl ShaderType {
    pub fn to_shaders() -> (Vec<Self>, Vec<(CString, CString)>) {
        let mut shaders_vec = Vec::new();
        let mut types_vec = Vec::new();
        types_vec.push(ShaderType::BasicFlatShader);
        shaders_vec.push((
            CString::new(include_str!("shaders/flat.vert.glsl")).unwrap(),
            CString::new(include_str!("shaders/flat.frag.glsl")).unwrap(),
        ));

        types_vec.push(ShaderType::TexShader);
        shaders_vec.push((
            CString::new(include_str!("shaders/tex.vert.glsl")).unwrap(),
            CString::new(include_str!("shaders/tex.frag.glsl")).unwrap(),
        ));

        (types_vec, shaders_vec)
    }

    pub fn to_usize(&self) -> usize {
        match self {
            ShaderType::BasicFlatShader => 0,
            ShaderType::TexShader => 1,
        }
    }

    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::BasicFlatShader),
            1 => Some(Self::TexShader),
            _ => None,
        }
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &(source.as_ptr()), null());
            gl::CompileShader(id);
        }

        let mut success = 1;

        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);
            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
