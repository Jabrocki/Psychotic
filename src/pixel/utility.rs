use std::ops::{Add, Mul, Sub};

use nalgebra::{Matrix, Matrix3, Point, Vector2};
use rand::{RngExt, SeedableRng, TryRng};

use crate::image::{BufferedImage, Color4f};

//

//

pub fn clamp(rgba: Color4f) -> Color4f{
	let Color4f(r, g, b, a) = rgba;
	Color4f(r.max(0.).min(1.), g.max(0.).min(1.), b.max(0.).min(1.), a.max(0.).min(1.))
}

pub fn step<T>(edge: T, x: T) -> T
where T: PartialOrd + Sub + Add + From<f32>{
	if x > edge{
		return T::from(1.);
	}
	return T::from(0.);
}

pub fn step_x<T>(edge: T, x: T) -> T
where T: PartialOrd + Sub + Add + From<f32>{
	if x > edge{
		return x;
	}
	return T::from(0.);
}

pub fn toroidal_dist2(vec1: Vector2<i32>, vec2: Vector2<i32>, size: i32) -> i32{
	let mut dx = (vec1.x - vec2.x).abs();
	let mut dy = (vec1.y - vec2.y).abs();

	dx = dx.min(size - dx);
	dy = dy.min(size - dy);

	return dx*dx + dy*dy;
}

pub fn worley_noise(texture_size: usize, cell_amount: usize, seed: u64) -> BufferedImage<f32> {
	let mut buffer : BufferedImage<f32> = BufferedImage::new(texture_size, texture_size);
	let mut cells : Vec<Vector2<i32>> = vec![];
	let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

	let cell_size = texture_size / cell_amount;

	for c in 0..(cell_amount*cell_amount){

		let c_x = c % cell_amount;
		let c_y = c / cell_amount;

		let x: f32 = rng.random_range(0.0..1.0);
		let y: f32 = rng.random_range(0.0..1.0);

		let x_final = ((cell_size as f32 * x) + (c_x * cell_size) as f32) as i32;
		let y_final = ((cell_size as f32 * y) + (c_y * cell_size) as f32) as i32;

		cells.push(
			Vector2::new(x_final, y_final)
		);
	}

	for y in 0..texture_size{
		for x in 0..texture_size{
			let x = x as i32;
			let y = y as i32;

			let mut closest = cells[0];
			let xy = Vector2::new(x as i32, y as i32);
			for &c in cells[1..].iter(){
				// toroidal distance

				if toroidal_dist2(c, xy, texture_size as i32) < toroidal_dist2(closest, xy, texture_size as i32){
					closest = c;
				}
			}
			buffer[(x as usize,y as usize)] =
			 (toroidal_dist2(closest, xy, texture_size as i32) as f32).sqrt() / (cell_size as f32 *(2. as f32).sqrt()) as f32;
		}
	}

	return buffer;
}

pub fn worley_noise_bricks(texture_size: usize, cell_amount: usize, seed: u64,
		brick_colors: Vec<Color4f>) -> BufferedImage<Color4f> {
	let mut buffer : BufferedImage<Color4f> = BufferedImage::new(texture_size, texture_size);
	let mut cells : Vec<Vector2<i32>> = vec![];
	let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

	let cell_size = texture_size / cell_amount;

	for c in 0..(cell_amount*cell_amount){

		let c_x = c % cell_amount;
		let c_y = c / cell_amount;

		let x: f32 = rng.random_range(0.0..1.0);
		let y: f32 = rng.random_range(0.0..1.0);

		let x_final = ((cell_size as f32 * x) + (c_x * cell_size) as f32) as i32;
		let y_final = ((cell_size as f32 * y) + (c_y * cell_size) as f32) as i32;

		cells.push(
			Vector2::new(x_final, y_final)
		);
	}

	for y in 0..texture_size{
		for x in 0..texture_size{
			let x = x as i32;
			let y = y as i32;

			let mut closest = cells[0];
			let mut cell_idx = 0;
			let xy = Vector2::new(x as i32, y as i32);
			for (i,&c) in cells[1..].iter().enumerate(){
				// toroidal distance

				if toroidal_dist2(c, xy, texture_size as i32) < toroidal_dist2(closest, xy, texture_size as i32){
					closest = c;
					cell_idx = i;
				}
			}

			let Color4f(r, g, b, a) = brick_colors[cell_idx % brick_colors.len()];
			//buffer[(y*texture_size as i32+x) as usize] =
			buffer[(x as usize, y as usize)] =
			 Color4f(r - 0.01*(cell_idx % (cell_amount-2)) as f32, g - 0.01*(cell_idx % (cell_amount-2)) as f32, b - 0.01*(cell_idx % (cell_amount-2)) as f32, a);
		}
	}

	return buffer;
}

/*pub fn apply_kernel(image: Vec<rgba_t>, kernel: Matrix3<f32>, width: usize, height: usize){

	for y in 0..width{
		for x in 0..height{
			let mut acc: rgba_t = rgba_t(0.,0.,0.,0.);

			for kernel_row in 0..3{
				for kernel_
			}
		}
	}

}*/
