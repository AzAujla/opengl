#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: (u32, u32),
    pub tex_coords: (u32, u32),
    pub color: u32,
    pub texture_id: u32,
}
