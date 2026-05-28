use sdl2::pixels::Color;

use crate::{
    image::{BufferedImage, Color4f},
    pixel::utility::{crate_pattern, sine_pattern, turbulence, wood_pattern},
    pixel_prog::PixelProgram,
};

const WOOD_SEED: u64 = 10;

pub struct WoodTexture {
    turbulence_texture: BufferedImage<f32>,
}

impl PixelProgram for WoodTexture {
    fn update(&mut self, t: f32) {}

    fn color(&self, state: &crate::pixel_prog::PipelineState, x: u32, y: u32, t: f32) -> u32 {
        let tx = (x % 64) as usize;
        let ty = (y % 64) as usize;
        let tex_size = 64.0;

        let freq_x = 5.0;
        let freq_y = 20.0;
        let power = 5.0;
        let speed = 0.02;

        let xy_value = (tx as f32 * freq_x / tex_size) + (ty as f32 * freq_y / tex_size);

        let total_value = xy_value + (self.turbulence_texture[(tx, ty)] * power) - (t * speed);

        let mut wood_val = (total_value * std::f32::consts::PI).sin().abs();

        let seed = (tx as f32 * 12.9898) + (ty as f32 * 78.233);
        let mut noise = (seed.sin() * 43758.5453);
        noise = noise - noise.floor();
        wood_val = (wood_val - (noise * 0.15)).max(0.0);

        let mut shade = 1.0;
        let edge = 6.0;

        if (tx as f32) < edge {
            shade *= tx as f32 / edge;
        } else if (tx as f32) > tex_size - 1.0 - edge {
            shade *= (tex_size - 1.0 - tx as f32) / edge;
        }

        if (ty as f32) < edge {
            shade *= ty as f32 / edge;
        } else if (ty as f32) > tex_size - 1.0 - edge {
            shade *= (tex_size - 1.0 - ty as f32) / edge;
        }

        shade = shade * 0.6 + 0.4;

        // --- FINAŁ ---
        let base_color = Color4f(0.74, 0.45, 0.2, 1.0);
        (base_color * wood_val * shade).to_u32()
    }
}

impl WoodTexture {
    pub fn new() -> Self {
        WoodTexture {
            turbulence_texture: turbulence(64, 3, WOOD_SEED),
        }
    }
}
