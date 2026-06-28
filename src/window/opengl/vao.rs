use std::ptr::null;

use gl::types::{GLint, GLuint};

pub struct Vao {
    id: GLuint,
}

impl Default for Vao {
    fn default() -> Self {
        Self::new()
    }
}

impl Vao {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    pub fn set(&self, textured: bool) {
        self.bind();
        self.setup(textured);
    }

    fn setup(&self, textured: bool) {
        if textured {
            // Vertex attributes
            let stride = (4 * std::mem::size_of::<f32>()) as i32;

            unsafe {
                // Position: location = 0
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());

                // TexCoord: location = 1
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (2 * std::mem::size_of::<f32>()) as *const _,
                );
            }
        } else {
            unsafe {
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (2 * std::mem::size_of::<f32>()) as GLint,
                    null(),
                );
            }
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}
