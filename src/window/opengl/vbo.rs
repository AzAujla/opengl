use gl::types::{GLsizeiptr, GLuint, GLvoid};

pub struct Vbo {
    id: GLuint,
}

impl Default for Vbo {
    fn default() -> Self {
        Self::new()
    }
}

impl Vbo {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id }
    }

    pub fn set(&self) {
        self.bind();
    }

    fn data(&self, vertices: &[f32]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}
