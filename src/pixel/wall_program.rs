use sdl2::pixels::Color;

use crate::{pixel::utility::{rgba, worley_noise}, pixel_prog::PixelProgram};

const WALL_SEED: u64 = 0;

pub struct WallProgram{
	worley: Vec<f32>
}

impl PixelProgram for WallProgram {

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32) -> u32 {
    	rgba(self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize], self.worley[(x+y*64) as usize])
    }
}

impl WallProgram {
	pub fn new() -> Self{
		WallProgram { worley: worley_noise(64, 5, WALL_SEED) }
	}
}

unsafe impl Send for WallProgram {

}

unsafe impl Sync for WallProgram {

}
