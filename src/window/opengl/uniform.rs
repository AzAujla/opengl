use std::ffi::CString;

use gl::types::GLint;

#[derive(Clone)]
pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: u32, name: &str) -> Result<Self, String> {
        let cname = CString::new(name).expect("CString::new failed");
        let location = unsafe { gl::GetUniformLocation(program, cname.as_ptr()) };
        if location == -1 {
            return Err(format!("Couldn't get location for {}", name));
        }
        Ok(Self { id: location })
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}
