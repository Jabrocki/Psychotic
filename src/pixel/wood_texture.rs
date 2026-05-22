use sdl2::pixels::Color;

use crate::{image::{BufferedImage, Color4f}, pixel::utility::{crate_pattern, sine_pattern, turbulence, wood_pattern}, pixel_prog::PixelProgram};

const WOOD_SEED: u64 = 10;

pub struct WoodTexture{
	turbulence_texture: BufferedImage<f32>,
	sine_pattern: BufferedImage<f32>,
	crate_buffer: BufferedImage<f32>
}

impl PixelProgram for WoodTexture {
	fn update(&mut self, t: f32) {

    }

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32, t: f32) -> u32 {

    	(Color4f(0.74, 0.45, 0.2, 1.0) *
     			self.sine_pattern[(x as usize, y as usize)]
        * self.crate_buffer[(x as usize, y as usize)]

        ).to_u32()
    }
}

impl WoodTexture {
	pub fn new() -> Self{

		WoodTexture {
			turbulence_texture: turbulence(64, 3, WOOD_SEED),
			sine_pattern: wood_pattern(&turbulence(64, 4, WOOD_SEED)),
			crate_buffer: crate_pattern(64)
		}
	}
}

unsafe impl Send for WoodTexture {

}

unsafe impl Sync for WoodTexture {

}
