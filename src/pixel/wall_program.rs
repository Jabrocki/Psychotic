use noise::{NoiseFn, Perlin};
use sdl2::pixels::Color;

use crate::{image::{BufferedImage, Color4f}, pixel::utility::{fsin, step, step_x, worley_noise, worley_noise_border, worley_noise_bricks}, pixel_prog::PixelProgram};

const WALL_SEED: u64 = 2;

pub struct WallProgram{
	worley: BufferedImage<Color4f>,
	worley_pure: BufferedImage<f32>
}

impl PixelProgram for WallProgram {
	fn update(&mut self, t: f32) {
		self.worley = worley_noise_bricks(64, 5, WALL_SEED + (t as u64)/30, vec![
			Color4f(0.33,0.33,0.33,1.),
			Color4f(0.28,0.21,0.09,1.),
			Color4f(0.25,0.25,0.25,1.),
		 	Color4f(0.36,0.24,0.12,1.)]);
		self.worley_pure = worley_noise_border(64, 5, WALL_SEED + (t as u64)/30);
    }

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32, t: f32) -> u32 {
    	let (x_sampled, y_sampled) = (x as usize, y as usize);

     	/*let sin_out = fsin(t as f32 / 300.);

     	let mushroom = Color4f(
      		(sin_out + 0.5),
        	(sin_out + 0.1),
         	(sin_out + 0.1),
          	1.
      	);*/

        ((Color4f::fill((self.worley_pure[(x_sampled, y_sampled)]))
        		* self.worley[(x_sampled, y_sampled)]) * 2.).to_u32()
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
