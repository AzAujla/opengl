use sdl2::pixels::Color;

pub fn sdl_color_to_u32(color: Color) -> u32 {
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;
    let a = color.a as u32;

    // Packs channels into ABGR bitwise layout (Little Endian memory maps to RGBA order)
    (a << 24) | (b << 16) | (g << 8) | r
}
