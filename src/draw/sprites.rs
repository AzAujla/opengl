use crate::{
    draw::{Draw, vertex::Vertex},
    entity::graphics::sprite::ToSprite,
    utils::colors::sdl_color_to_u32,
};

impl Draw {
    pub fn sprite<T: ToSprite>(&mut self, x: u32, y: u32, sprite: &T) -> &mut Self {
        let sprite_cow = sprite.to_sprite();
        let (uv_left, uv_top, uv_right, uv_bottom) = sprite_cow.uv();
        let color = sdl_color_to_u32(sdl2::pixels::Color::WHITE);

        let (width, height) = (uv_right - uv_left, uv_bottom - uv_top);
        let texture_id = self.texture_mgr.get_or_load_layer(sprite_cow.path());

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
