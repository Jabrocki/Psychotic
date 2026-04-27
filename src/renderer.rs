use nalgebra::Vector2;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{player::Player, texture_manager::TextureManager, world::World};

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    window_width: u32,
    window_height: u32,
    screen_buffer: Vec<u32>,
    texture: Texture<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        window_width: u32,
        window_height: u32,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let screen_buffer = vec![0; (window_width * window_height) as usize];

        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA8888, window_width, window_height)
            .unwrap();
        Self {
            canvas,
            window_width,
            window_height,
            screen_buffer,
            texture,
        }
    }

    pub fn render(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        for pixel in self.screen_buffer.iter_mut() {
            *pixel = 0;
        }

        self.render_world(world, player, texture_manager);

        self.texture
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

        let _ = self.canvas.copy(&self.texture, None, None);
        self.render_debug_info(world, player);
        self.canvas.present();
    }

    fn render_world(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        let plane = Vector2::new(-player.direction().y * 0.66, player.direction().x * 0.66);

        // Floor casting
        for y_s in 0..self.window_height {
            let ray_dir0 = player.direction() - plane;
            let ray_dir1 = player.direction() + plane;

            let p = y_s as i32 - (self.window_height / 2) as i32;
            let pos_z = 0.5 * self.window_height as f32;

            let row_distance = pos_z / p as f32;

            let floor_step = row_distance * (ray_dir1 - ray_dir0) / self.window_width as f32;
            let mut floor_pos = player.position() + row_distance * ray_dir0;

            for x in 0..self.window_width {
                let tx = ((64.0 * (floor_pos.x - floor_pos.x.floor())) as u32) & 63;
                let ty = ((64.0 * (floor_pos.y - floor_pos.y.floor())) as u32) & 63;

                floor_pos += floor_step;

                let mut color_floor = texture_manager
                    .get_texture_color(3, tx, ty)
                    .unwrap_or(Color::RGB(255, 255, 255));

                let mut color_ceiling = texture_manager
                    .get_texture_color(4, tx, ty)
                    .unwrap_or(Color::RGB(255, 255, 255));

                color_floor = Color::RGB(
                    (color_floor.r as f32 * 0.5) as u8,
                    (color_floor.g as f32 * 0.5) as u8,
                    (color_floor.b as f32 * 0.5) as u8,
                );

                color_ceiling = Color::RGB(
                    (color_ceiling.r as f32 * 0.5) as u8,
                    (color_ceiling.g as f32 * 0.5) as u8,
                    (color_ceiling.b as f32 * 0.5) as u8,
                );

                let pixel_val_floor = ((color_floor.r as u32) << 24)
                    | ((color_floor.g as u32) << 16)
                    | ((color_floor.b as u32) << 8)
                    | 255;

                let pixel_val_ceiling = ((color_ceiling.r as u32) << 24)
                    | ((color_ceiling.g as u32) << 16)
                    | ((color_ceiling.b as u32) << 8)
                    | 255;

                // Rysujemy podłogę
                self.screen_buffer[y_s as usize * self.window_width as usize + x as usize] =
                    pixel_val_floor;

                // Rysujemy sufit
                self.screen_buffer[(self.window_height - 1 - y_s) as usize
                    * self.window_width as usize
                    + x as usize] = pixel_val_ceiling;
            }
        }
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
