use std::{f32::consts::PI, ops::{Add, Div, Mul, Rem, Sub}};

use nalgebra::{Matrix, Matrix3, Point, Vector2};
use noise::{NoiseFn, Perlin};
use rand::{RngExt, SeedableRng, TryRng};

use crate::image::{BufferedImage, Color4f};

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
		brick_colors: Vec<Color4f>, t: f32) -> BufferedImage<Color4f> {
	let mut buffer : BufferedImage<Color4f> = BufferedImage::new(texture_size, texture_size);
	let mut cells : Vec<Vector2<i32>> = vec![];
	let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

	let cell_size = texture_size / cell_amount;

	for c in 0..(cell_amount*cell_amount){

		let c_x = c % cell_amount;
		let c_y = c / cell_amount;

		let x: f32 = rng.random_range(0.0..1.0)*(t.cos());
		let y: f32 = rng.random_range(0.0..1.0)*(t.sin());

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

pub fn worley_noise_border(texture_size: usize, cell_amount: usize, seed: u64, t: f32) -> BufferedImage<f32> {
	let mut buffer : BufferedImage<f32> = BufferedImage::new(texture_size, texture_size);
	let mut cells : Vec<Vector2<i32>> = vec![];
	let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

	let cell_size = texture_size / cell_amount;

	for c in 0..(cell_amount*cell_amount){

		let c_x = c % cell_amount;
		let c_y = c / cell_amount;

		let x: f32 = rng.random_range(0.0..1.0)*(t.cos());
		let y: f32 = rng.random_range(0.0..1.0)*(t.sin());

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

			let mut closest = Vector2::new(i32::MAX, i32::MAX);
			let xy = Vector2::new(x as i32, y as i32);
			let mut closest_2 = Vector2::new(i32::MAX, i32::MAX);
			for &c in cells.iter(){
				// toroidal distance

				if toroidal_dist2(c, xy, texture_size as i32) < toroidal_dist2(closest, xy, texture_size as i32){
					closest_2 = closest;

					closest = c;
				} else if toroidal_dist2(c, xy, texture_size as i32) < toroidal_dist2(closest_2, xy, texture_size as i32){
					closest_2 = c;
				}

			}

			buffer[(x as usize,y as usize)] =
				((toroidal_dist2(closest, xy, texture_size as i32) as f32).sqrt()
			 								/ (cell_size as f32 *(2. as f32).sqrt()) as f32
				- (toroidal_dist2(closest_2, xy, texture_size as i32) as f32).sqrt()
											/ (cell_size as f32 *(2. as f32).sqrt()) as f32).abs();

		}
	}

	return buffer;
}

pub fn fsin(x: f32) -> f32{
	let x = x.rem_euclid(PI);
	(x / PI) * (1. - (x / PI).abs()) * 4.
}

pub fn turbulence(texture_size: usize, turbulence_size: usize, seed: u64) -> BufferedImage<f32>{
	let mut buffer : BufferedImage<f32> = BufferedImage::new(texture_size, texture_size);
	let perlin = Perlin::new(1);

	let mut min: f32 = 1.;
	let mut max: f32 = 0.;

	for y in 0..texture_size{
		for x in 0..texture_size{
			let mut size: f64 = turbulence_size as f64;

			buffer[(x, y)] = 0.;

			while size >= 1.{
				let nx = ((x as f64)/(texture_size as f64) / size) * 10.;
				let ny = ((y as f64)/(texture_size as f64) / size) * 10.;

				buffer[(x, y)] += perlin.get([nx, ny, seed as f64]) as f32 * size as f32;
				size /= 2.;
			}

			buffer[(x, y)] /= turbulence_size as f32;

			if buffer[(x, y)] < min{
				min = buffer[(x, y)];
			}
			if buffer[(x, y)] > max{
				max = buffer[(x, y)];
			}

			println!("{}", buffer[(x, y)]);

		}
	}

	for y in 0..texture_size{
		for x in 0..texture_size{
			buffer[(x, y)] = (buffer[(x, y)] - min)/(max - min);
		}
	}
	buffer
}

pub fn sine_pattern(texture_size: usize) -> BufferedImage<f32>{
	let mut buffer : BufferedImage<f32> = BufferedImage::new(texture_size, texture_size);

	for y in 0..(texture_size){
		for x in 0..(texture_size){
			buffer[(x, y)] = ((y+x) as f32 ).sin();
		}
	}

	buffer
}

pub fn wood_pattern(turbulence: &BufferedImage<f32>) -> BufferedImage<f32>{
	let mut buffer: BufferedImage<f32> = BufferedImage::new(turbulence.width, turbulence.height);

	let power = 10.;

	for y in 0..(turbulence.height){
		for x in 0..(turbulence.width){
			buffer[(x, y)] = ((
				(x+y) as f32 + turbulence[(x, y)] * power
				).sin().max(0.) + 0.3).min(1.0);
		}
	}

	buffer
}

pub fn crate_pattern(texture_size: usize) -> BufferedImage<f32>{
	let mut buffer: BufferedImage<f32> = BufferedImage::new(texture_size, texture_size);

	for y in 0..texture_size{
		for x in 0..texture_size{
			if ((y - x) as f32).abs() <= 6.{
				buffer[(x, y)] = 0.2;
			}
			else if ((-(texture_size as i32 - 1) + y as i32 + x as i32) as f32).abs() <= 3.{
				buffer[(x, y)] = 0.2;
			}
			else if (x) < 3 || (y) > texture_size - 4 || x > texture_size - 4 || y < 3{
				buffer[(x, y)] = 0.2;
			}
			else {
				buffer[(x, y)] = 1.;
			}
		}
	}
	buffer
}
