use nalgebra::Vector2;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::Canvas,
    video::Window,
};

use crate::{player::Player, texture_manager::TextureManager, world::World};

pub struct Renderer {
    canvas: Canvas<Window>,
    window_width: u32,
    window_height: u32,
    screen_buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>, window_width: u32, window_height: u32) -> Self {
        let screen_buffer = vec![0; (window_width * window_height) as usize];
        Self {
            canvas,
            window_width,
            window_height,
            screen_buffer,
        }
    }

    pub fn render(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        for pixel in self.screen_buffer.iter_mut() {
            *pixel = 0;
        }

        self.render_world(world, player, texture_manager);

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA8888,
                self.window_width,
                self.window_height,
            )
            .unwrap();

        texture
            .update(
                None,
                unsafe {
                    std::slice::from_raw_parts(
                        self.screen_buffer.as_ptr() as *const u8,
                        self.screen_buffer.len() * 4,
                    )
                },
                (self.window_width * 4) as usize,
            )
            .unwrap();

        let _ = self.canvas.copy(&texture, None, None);
        self.render_debug_info(world, player);
        self.canvas.present();
    }

    fn render_world(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        let plane = Vector2::new(-player.direction().y * 0.66, player.direction().x * 0.66);

        for x_s in 0..self.window_width {
            let camera_x = 2.0 * x_s as f32 / self.window_width as f32 - 1.0;
            let ray_dir = player.direction() + plane * camera_x;

            let mut map_pos = Vector2::new(player.position().x as i32, player.position().y as i32);
            let delta_dist = Vector2::new((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());
            let step = ray_dir.map(|c| c.signum() as i32);

            let mut side_dist = Vector2::new(
                if ray_dir.x < 0.0 {
                    (player.position().x - map_pos.x as f32) * delta_dist.x
                } else {
                    (map_pos.x as f32 + 1.0 - player.position().x) * delta_dist.x
                },
                if ray_dir.y < 0.0 {
                    (player.position().y - map_pos.y as f32) * delta_dist.y
                } else {
                    (map_pos.y as f32 + 1.0 - player.position().y) * delta_dist.y
                },
            );

            let mut side = 0;
            let mut hit = false;

            while !hit {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    side = 0;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;
                    side = 1;
                }

                if map_pos.x < 0
                    || map_pos.x >= world.width() as i32
                    || map_pos.y < 0
                    || map_pos.y >= world.height() as i32
                    || world.get_tile(map_pos.x as usize, map_pos.y as usize) != 0
                {
                    hit = true;
                }
            }

            let m_x = map_pos.x.clamp(0, world.width() as i32 - 1) as usize;
            let m_y = map_pos.y.clamp(0, world.height() as i32 - 1) as usize;

            let perp_wall_dist = if side == 0 {
                (side_dist.x - delta_dist.x).max(0.01)
            } else {
                (side_dist.y - delta_dist.y).max(0.01)
            };

            let line_height = (self.window_height as f32 / perp_wall_dist) as i32;
            let start = (-line_height / 2 + self.window_height as i32 / 2).max(0);
            let end = (line_height / 2 + self.window_height as i32 / 2)
                .min(self.window_height as i32 - 1);

            let mut wall_x = if side == 0 {
                player.position().y + perp_wall_dist * ray_dir.y
            } else {
                player.position().x + perp_wall_dist * ray_dir.x
            };
            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * 64.0) as u32;
            if (side == 0 && ray_dir.x > 0.0) || (side == 1 && ray_dir.y < 0.0) {
                tex_x = 63 - (tex_x & 63);
            }
            tex_x &= 63;

            let step_tex = 64.0 / line_height as f32;
            let mut tex_pos = (start as f32 - self.window_height as f32 / 2.0
                + line_height as f32 / 2.0)
                * step_tex;

            for y in start..end {
                let tex_y = (tex_pos as u32) & 63;
                tex_pos += step_tex;

                let color = texture_manager
                    .get_texture_color(world.get_tile(m_x, m_y), tex_x, tex_y)
                    .unwrap_or(Color::RGB(255, 255, 255));

                let (r, g, b) = if side == 1 {
                    (color.r / 2, color.g / 2, color.b / 2)
                } else {
                    (color.r, color.g, color.b)
                };

                let pixel_val =
                    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 255_u32;
                self.screen_buffer[y as usize * self.window_width as usize + x_s as usize] =
                    pixel_val;
            }
        }
    }

    fn render_debug_info(&mut self, world: &World, player: &Player) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..world.height() {
            for x in 0..world.width() {
                let rect = Rect::new((x as i32) * 3, (y as i32) * 3, 3, 3);
                if world.get_tile(x, y) != 0 {
                    let _ = self.canvas.fill_rect(rect);
                }
            }
        }
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            (player.position().x * 3.0) as i32,
            (player.position().y * 3.0) as i32,
            3,
            3,
        );
        let _ = self.canvas.fill_rect(rect);
    }
}
