use crate::draw::vertex::Vertex;

pub mod shapes;
pub mod vertex;

pub struct Draw {
    logical_size: (u32, u32),
    sprite_size: u32,
    layers: u32,
    vertices: Vec<Vertex>,
}

impl Draw {
    pub fn new(logical_size: (u32, u32), sprite_size: u32, layers: u32) -> Self {
        Self {
            logical_size,
            sprite_size,
            layers,
            vertices: Vec::new(),
        }
    }

    pub fn set_logical_size(&mut self, w: u32, h: u32) -> &mut Self {
        self.logical_size = (w, h);
        self
    }

    pub fn set_sprite_size(&mut self, sprite_size: u32) -> &mut Self {
        self.sprite_size = sprite_size;
        self
    }

    pub fn sprite_size(&self) -> u32 {
        self.sprite_size
    }

    pub fn logical_size(&self) -> (u32, u32) {
        self.logical_size
    }

    pub fn set_layers(&mut self, layers: u32) {
        self.layers = layers;
    }

    pub fn layers(&self) -> u32 {
        self.layers
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<Vertex> {
        &mut self.vertices
    }
}
