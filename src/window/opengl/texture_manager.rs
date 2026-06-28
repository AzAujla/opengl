use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct DynamicTextureManager {
    array_id: u32,
    max_layers: u32,
    sheet_width: u32,
    sheet_height: u32,

    // CPU Tracking maps: Maps a file path to its active GPU layer slice index
    loaded_assets: HashMap<PathBuf, u32>,
    next_empty_layer: u32,
}

impl DynamicTextureManager {
    /// Pre-allocates an empty multi-layer storage rack on the GPU
    pub fn new(max_layers: u32, sheet_width: u32, sheet_height: u32) -> Self {
        let mut array_id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut array_id);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, array_id);

            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );

            // 🔑 Allocate empty structural frames (null data) for future assets to stream into
            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA8 as i32,
                sheet_width as i32,
                sheet_height as i32,
                max_layers as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0);
        }

        Self {
            array_id,
            max_layers,
            sheet_width,
            sheet_height,
            loaded_assets: HashMap::new(),
            next_empty_layer: 0,
        }
    }

    pub fn get_or_load_layer<P: AsRef<Path>>(&mut self, path: P) -> u32 {
        let path_buf = path.as_ref().to_path_buf();

        if let Some(&layer_index) = self.loaded_assets.get(&path_buf) {
            return layer_index;
        }

        if self.next_empty_layer >= self.max_layers {
            panic!("GPU Texture Array storage exhausted! Increase max_layers capacity.");
        }

        let img = image::open(&path_buf)
            .expect("Failed to load runtime sprite asset path")
            .to_rgba8();

        let (img_w, img_h) = img.dimensions();
        let raw_pixels = img.into_raw();

        let assigned_layer = self.next_empty_layer;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.array_id);

            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                0,
                0,
                assigned_layer as i32, // Target slot coordinates
                img_w as i32,
                img_h as i32,
                1, // Depth step: exactly 1 file layer slice
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                raw_pixels.as_ptr() as *const _,
            );

            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0);
        }

        // 4. Save to our tracking database and increment our layer pointer
        self.loaded_assets.insert(path_buf, assigned_layer);
        self.next_empty_layer += 1;

        assigned_layer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.array_id);
        }
    }
}
