use sdl2::pixels::Color;

use crate::{image::Color4f, pixel::utility::turbulence, pixel_prog::PixelProgram};

const WOOD_SEED: u64 = 10;
const TEX_SIZE: usize = 64;

pub struct WoodTexture {
    static_val: Vec<f32>,
    noise_val: Vec<f32>,
    base_shade_color: Vec<Color4f>,
}

impl PixelProgram for WoodTexture {
    fn update(&mut self, _t: f32) {}

    fn color(&self, _state: &crate::pixel_prog::PipelineState, x: u32, y: u32, t: f32) -> u32 {
        let tx = (x & 63) as usize;
        let ty = (y & 63) as usize;

        let idx = (ty << 6) | tx;

        let time_off = t * 0.02;
        let total_value = self.static_val[idx] - time_off;

        let mut wood_val = (total_value * std::f32::consts::PI).sin().abs();
        wood_val = (wood_val - self.noise_val[idx]).max(0.0);

        (self.base_shade_color[idx] * wood_val).to_u32()
    }
}

impl WoodTexture {
    pub fn new() -> Self {
        let turbulence_texture = turbulence(64, 3, WOOD_SEED);

        let mut static_val = Vec::with_capacity(TEX_SIZE * TEX_SIZE);
        let mut noise_val = Vec::with_capacity(TEX_SIZE * TEX_SIZE);
        let mut base_shade_color = Vec::with_capacity(TEX_SIZE * TEX_SIZE);

        let tex_size_f = 64.0;
        let freq_x = 5.0;
        let freq_y = 20.0;
        let power = 5.0;
        let edge = 6.0;
        let base_color = Color4f(0.74, 0.45, 0.2, 1.0);

        for ty in 0..TEX_SIZE {
            for tx in 0..TEX_SIZE {
                let tx_f = tx as f32;
                let ty_f = ty as f32;

                let xy_value = (tx_f * freq_x / tex_size_f) + (ty_f * freq_y / tex_size_f);
                let t_val = xy_value + (turbulence_texture[(tx, ty)] * power);
                static_val.push(t_val);

                let seed = (tx_f * 12.9898) + (ty_f * 78.233);
                let mut noise = seed.sin() * 43758.5453;
                noise = noise - noise.floor();
                noise_val.push(noise * 0.15);

                let mut shade = 1.0;

                if tx_f < edge {
                    shade *= tx_f / edge;
                } else if tx_f > tex_size_f - 1.0 - edge {
                    shade *= (tex_size_f - 1.0 - tx_f) / edge;
                }

                if ty_f < edge {
                    shade *= ty_f / edge;
                } else if ty_f > tex_size_f - 1.0 - edge {
                    shade *= (tex_size_f - 1.0 - ty_f) / edge;
                }

                shade = shade * 0.6 + 0.4;

                base_shade_color.push(base_color * shade);
            }
        }

        WoodTexture {
            static_val,
            noise_val,
            base_shade_color,
        }
    }
}

impl Default for WoodTexture {
    fn default() -> Self {
        Self::new()
    }
}
