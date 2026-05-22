use std::ops::{Add, Index, IndexMut, Mul, Sub};


pub trait Texel : Mul<Output = Self>
			+ Add<Output = Self>
			+ Sub<Output = Self>
			+ Default{

}

impl Texel for f32 {

}

impl Texel for f64 {

}

impl Texel for i32 {

}

impl Texel for i64 {

}

impl Texel for u32{

}

impl Texel for u64 {

}

pub struct BufferedImage<T> where T: Texel{
	pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> BufferedImage<T> where T: Texel{
	pub fn new(width: usize, height: usize) -> Self{
		let mut texels = vec![];
		for _ in 0..width*height{
			texels.push(T::default());
		}
		BufferedImage { width, height, data: texels }
	}
}

impl<T> IndexMut<(usize, usize)> for BufferedImage<T> where T: Texel{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    	let (x, y) = index;

   		&mut self.data[(y*self.height+x) as usize]
    }
}

impl<T> Index<(usize, usize)> for BufferedImage<T> where T: Texel{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
    	let (x, y) = index;
     	//let (x, y) = (x.min(self.width-1).max(0), y.min(self.height-1).max(0));

    	&self.data[(y*self.height+x) as usize]
    }
}

//

#[derive(Clone, Copy)]
pub struct Color4f(pub f32, pub f32, pub f32, pub f32);

impl Color4f {
    pub fn to_u32(&self) -> u32{
	   	let r_c = (self.0*255.) as u32;
		let g_c = (self.1*255.) as u32;
		let b_c = (self.2*255.) as u32;
		let a_c = (self.3*255.) as u32;

		r_c | g_c << 8 | b_c << 16 | a_c << 24
    }

    pub fn fill(v: f32) -> Self{
   		Color4f(v, v, v, v)
    }
}

impl Add for Color4f{
    type Output = Color4f;

    fn add(self, rhs: Self) -> Self::Output {
    	let Color4f(r, g, b, a) = rhs;
     	Color4f(self.0+r, self.1+g, self.2+b, self.3+a)
    }
}

impl Sub for Color4f {
    type Output = Color4f;

    fn sub(self, rhs: Self) -> Self::Output {
    	let Color4f(r, g, b, a) = rhs;
    	Color4f(self.0-r, self.1-g, self.2-b, self.3-a)
    }
}

impl Mul<f32> for Color4f {
    type Output = Color4f;

    fn mul(self, rhs: f32) -> Self::Output {
    	Color4f(self.0*rhs, self.1*rhs, self.2*rhs, self.3*rhs)
    }
}

impl Mul<Self> for Color4f {
    type Output = Color4f;

    fn mul(self, rhs: Self) -> Self::Output {
    	Color4f(self.0*rhs.0, self.1*rhs.1, self.2*rhs.2, self.3*rhs.3)
    }
}

impl Default for Color4f {
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default(), Default::default())
    }
}

impl Texel for Color4f{

}

// utils

pub fn clamp(rgba: Color4f, min_val: f32, max_val: f32) -> Color4f{
	let Color4f(r, g, b, a) = rgba;
	Color4f(r.max(min_val).min(max_val), g.max(min_val).min(max_val), b.max(min_val).min(max_val), a.max(min_val).min(max_val))
}
