use sdl2::{image::LoadSurface, pixels::PixelFormatEnum, surface::Surface};

use crate::image::BufferedImage;

pub type TextureDataStatic = BufferedImage<u32>;

pub struct TextureManager {
    textures_static: Vec<Option<TextureDataStatic>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures_static: (0..256).map(|_| None).collect(),
        }
    }

    pub fn load_static(&mut self, id: u8, path: &str) {
        let surface = Surface::from_file(path).expect("Failed to load texture");

        let opt_surface = surface
            .convert_format(PixelFormatEnum::ABGR8888)
            .expect("Failed to convert surface format");

        let width = opt_surface.width() as usize;
        let height = opt_surface.height() as usize;

        let data = opt_surface.with_lock(|data| unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4).to_vec()
        });

        self.textures_static[id as usize] = Some(TextureDataStatic {
            width,
            height,
            data,
        });
    }

    pub fn get_texture(&self, id: u8) -> Option<&TextureDataStatic> {
        self.textures_static[id as usize].as_ref()
    }
}
