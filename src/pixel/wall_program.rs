use sdl2::pixels::Color;

use crate::{pixel::utility::{clamp, rgba, rgba_t, step, step_x, worley_noise, worley_noise_bricks}, pixel_prog::PixelProgram};

const WALL_SEED: u64 = 0;

pub struct WallProgram{
	worley: Vec<rgba_t>
}

impl PixelProgram for WallProgram {

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32) -> u32 {
    	//rgba(self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize])
     	let rgba_t(r, g, b, a) = self.worley[(x+y*64) as usize];

      	rgba(r, g, b, a)
    }
}

impl WallProgram {
	pub fn new() -> Self{
		WallProgram {
			worley: worley_noise_bricks(64, 5, WALL_SEED, vec![rgba_t(0.33,0.33,0.33,1.), rgba_t(0.28,0.21,0.09,1.), rgba_t(0.25,0.25,0.25,1.), rgba_t(0.36,0.24,0.12,1.)]),
		}
	}
}

unsafe impl Send for WallProgram {

}

unsafe impl Sync for WallProgram {

}
