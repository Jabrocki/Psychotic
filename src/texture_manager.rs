use std::collections::HashMap;

use sdl2::{
    image::LoadSurface,
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

pub struct TextureData {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

pub struct TextureManager {
    textures: HashMap<u8, TextureData>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_static(&mut self, id: u8, path: &str) {
        let surface = Surface::from_file(path).expect("Failed to load texture");

        let opt_surface = surface
            .convert_format(PixelFormatEnum::RGB24)
            .expect("Failed to convert surface format");

        let width = opt_surface.width();
        let height = opt_surface.height();
        let pitch = opt_surface.pitch() as usize;

        let mut pixels = Vec::<Color>::with_capacity((width * height) as usize);
        let data = opt_surface.without_lock().unwrap();

        for y in 0..height {
            for x in 0..width {
                let offset = (y as usize * pitch) + (x as usize * 3);

                let color = Color::RGB(data[offset], data[offset + 1], data[offset + 2]);
                pixels.push(color);
            }
        }

        self.textures.insert(
            id,
            TextureData {
                width,
                height,
                data: pixels,
            },
        );
    }

    pub fn get_texture_color(&self, id: u8, x: u32, y: u32) -> Option<Color> {
        self.textures.get(&id).and_then(|texture| {
            if x < texture.width && y < texture.height {
                Some(texture.data[(y * texture.width + x) as usize])
            } else {
                None
            }
        })
    }
}
