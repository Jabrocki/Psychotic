use std::collections::HashMap;

use sdl2::{
    image::LoadSurface,
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

pub struct TextureManager {
    textures: Vec<Option<TextureData>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: (0..256).map(|_| None).collect(),
        }
    }

    pub fn load_static(&mut self, id: u8, path: &str) {
        let surface = Surface::from_file(path).expect("Failed to load texture");

        let opt_surface = surface
            .convert_format(PixelFormatEnum::ABGR8888)
            .expect("Failed to convert surface format");

        let width = opt_surface.width();
        let height = opt_surface.height();

        let data = opt_surface.with_lock(|data| unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4).to_vec()
        });

        self.textures[id as usize] = Some(TextureData {
            width,
            height,
            data,
        });
    }

    #[inline(always)]
    pub fn get_texture(&self, id: u8) -> Option<&TextureData> {
        self.textures[id as usize].as_ref()
    }
}
