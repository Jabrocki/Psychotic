use sdl2::pixels::Color;

use crate::{image::{BufferedImage, Color4f}, pixel::utility::{sine_pattern, turbulence, wood_pattern}, pixel_prog::PixelProgram};

const WOOD_SEED: u64 = 10;

pub struct WoodTexture{
	turbulence_texture: BufferedImage<f32>,
	sine_pattern: BufferedImage<f32>
}

impl PixelProgram for WoodTexture {
	fn update(&mut self, t: f32) {

    }

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32, t: f32) -> u32 {
    	if ((x) < 3 || (y) > 60 || x > 60 || y < 3){
     		return (Color4f(0.74, 0.45, 0.2, 1.0) * 0.7 *
       				self.turbulence_texture[(x as usize, y as usize)]).to_u32();
     	}

    	(Color4f(0.74, 0.45, 0.2, 1.0) *
     			self.sine_pattern[(x as usize, y as usize)]

        ).to_u32()
    }
}

impl WoodTexture {
	pub fn new() -> Self{

		WoodTexture {
			turbulence_texture: turbulence(64, 3, WOOD_SEED),
			sine_pattern: wood_pattern(&turbulence(64, 4, WOOD_SEED))
		}
	}
}

unsafe impl Send for WoodTexture {

}

unsafe impl Sync for WoodTexture {

}
