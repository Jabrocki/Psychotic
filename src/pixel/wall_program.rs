use noise::{NoiseFn, Perlin};
use sdl2::pixels::Color;

use crate::{image::{BufferedImage, Color4f}, pixel::utility::{step, step_x, worley_noise, worley_noise_border, worley_noise_bricks}, pixel_prog::PixelProgram};

const WALL_SEED: u64 = 2;

pub struct WallProgram{
	worley: BufferedImage<Color4f>,
	worley_pure: BufferedImage<f32>
}

impl PixelProgram for WallProgram {

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32) -> u32 {
        ((Color4f::fill((self.worley_pure[(x as usize, y as usize)]))
        		* self.worley[(x as usize, y as usize)]) * 3.).to_u32()
    }
}

impl WallProgram {
	pub fn new() -> Self{
		WallProgram {
			worley: worley_noise_bricks(64, 5, WALL_SEED, vec![
				Color4f(0.33,0.33,0.33,1.),
				Color4f(0.28,0.21,0.09,1.),
				Color4f(0.25,0.25,0.25,1.),
			 	Color4f(0.36,0.24,0.12,1.)]),
			worley_pure: worley_noise_border(64, 5, WALL_SEED),
		}
	}
}

unsafe impl Send for WallProgram {

}

unsafe impl Sync for WallProgram {

}
