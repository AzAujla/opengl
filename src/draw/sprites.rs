use crate::{
    draw::{Draw, vertex::Vertex},
    entity::sprite::Sprite,
    utils::colors::sdl_color_to_u32,
    window::opengl::texture_manager::DynamicTextureManager,
};

impl Draw {
    pub fn sprite(
        &mut self,
        text_mgr: &mut DynamicTextureManager,
        x: u32,
        y: u32,
        sprite: &Sprite,
    ) -> &mut Self {
        let (uv_left, uv_top, uv_right, uv_bottom) = sprite.uv();
        let color = sdl_color_to_u32(sdl2::pixels::Color::WHITE);

        let (width, height) = (uv_right - uv_left, uv_bottom - uv_top);
        let texture_id = text_mgr.get_or_load_layer(sprite.path());

        let top_left = Vertex {
            position: (x, y),
            tex_coords: (uv_left, uv_top),
            color,
            texture_id,
        };
        let top_right = Vertex {
            position: (x + width, y),
            tex_coords: (uv_right, uv_top),
            color,
            texture_id,
        };
        let bottom_right = Vertex {
            position: (x + width, y + height),
            tex_coords: (uv_right, uv_bottom),
            color,
            texture_id,
        };
        let bottom_left = Vertex {
            position: (x, y + height),
            tex_coords: (uv_left, uv_bottom),
            color,
            texture_id,
        };

        self.vertices.push(top_left);
        self.vertices.push(top_right);
        self.vertices.push(bottom_right);
        self.vertices.push(bottom_left);

        self
    }
}
