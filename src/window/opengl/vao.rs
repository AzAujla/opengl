use gl::types::GLuint;

use crate::draw::vertex::Vertex;

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

    pub fn set(&self) {
        self.bind();
    }

    pub fn setup(&self) {
        let stride = std::mem::size_of::<Vertex>() as gl::types::GLint; // 24 bytes

        unsafe {
            // 1. Position (Location = 0) -> 2 floats
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                std::ptr::null(), // Starts at byte 0
            );

            // 2. Texture Coordinates (Location = 1) -> 2 floats
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                8 as *const _, // Starts at byte 8 (after position)
            );

            // 3. Color (Location = 2) -> 4 unsigned bytes (Packed RGBA)
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE, // ⚠️ TRUE means normalize! Converts 0-255 integers to 0.0-1.0 floats in the shader
                stride,
                16 as *const _, // Starts at byte 16 (after tex_coords)
            );

            // 4. Texture ID (Location = 3) -> 1 unsigned int
            // ⚠️ Note the "I" in VertexAttribIPointer. Use this when sending pure integers to the shader!
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribIPointer(
                3,
                1,
                gl::UNSIGNED_INT,
                stride,
                20 as *const _, // Starts at byte 20 (after color)
            );
        }
    }

    pub fn bind(&self) {
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
