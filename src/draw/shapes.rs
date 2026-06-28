use sdl2::pixels::Color;

use crate::{
    draw::{Draw, vertex::Vertex},
    utils::colors::sdl_color_to_u32,
};

impl Draw {
    pub fn square_fill(&mut self, x: u32, y: u32, size: u32, color: Color) -> &mut Self {
        self.rect_fill(x, y, size, size, color);
        self
    }

    pub fn rect_fill(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> &mut Self {
        let flat_color_id = 16u32;
        let uv = (0, 0);
        let color = sdl_color_to_u32(color);

        let top_left = Vertex {
            position: (x, y),
            tex_coords: uv,
            color,
            texture_id: flat_color_id,
        };
        let top_right = Vertex {
            position: (x + width, y),
            tex_coords: uv,
            color,
            texture_id: flat_color_id,
        };
        let bottom_right = Vertex {
            position: (x + width, y + height),
            tex_coords: uv,
            color,
            texture_id: flat_color_id,
        };
        let bottom_left = Vertex {
            position: (x, y + height),
            tex_coords: uv,
            color,
            texture_id: flat_color_id,
        };

        self.vertices.push(top_left);
        self.vertices.push(top_right);
        self.vertices.push(bottom_right);
        self.vertices.push(bottom_left);

        self
    }
}
