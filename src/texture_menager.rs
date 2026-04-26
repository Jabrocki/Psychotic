use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<u8, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn load_static(&mut self, id: u8, path: &str) {
        let texture = self.texture_creator.load_texture(path).unwrap();
        self.textures.insert(id, texture);
    }

    // TODO: Procedury tutaj
    pub fn init_procedural(&mut self, id: u8, width: u32, height: u32) {
        let texture = self
            .texture_creator
            .create_texture_target(None, width, height)
            .unwrap();

        self.textures.insert(id, texture);
    }

    pub fn get_texture(&self, id: u8) -> Option<&Texture<'a>> {
        self.textures.get(&id)
    }
}
