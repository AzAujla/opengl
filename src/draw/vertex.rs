#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: (f32, f32),
    tex_coords: (f32, f32),
    color: u32,
    texture_id: u32,
}
