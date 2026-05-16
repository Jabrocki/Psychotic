use sdl2::pixels::Color;

use crate::{image::{BufferedImage, Color4f}, pixel::utility::{clamp, step, step_x, worley_noise, worley_noise_bricks}, pixel_prog::PixelProgram};

const WALL_SEED: u64 = 0;

pub struct WallProgram{
	worley: BufferedImage<Color4f>
}

impl PixelProgram for WallProgram {

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32) -> u32 {
    	//rgba(self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize])
     	let Color4f(r, g, b, a) = self.worley[(x as usize, y as usize)];

      	Color4f(r, g, b, a).to_u32()
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
		}
	}
}

unsafe impl Send for WallProgram {

}

unsafe impl Sync for WallProgram {

}
