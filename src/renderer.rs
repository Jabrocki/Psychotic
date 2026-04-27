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
            .create_texture_streaming(PixelFormatEnum::ABGR8888, window_width, window_height)
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
        self.screen_buffer.fill(0);

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
        let w = self.window_width as usize;
        let h = self.window_height as usize;

        let floor_tex = texture_manager.get_texture(3);
        let ceil_tex = texture_manager.get_texture(4);

        if let (Some(f_t), Some(c_t)) = (floor_tex, ceil_tex) {
            let ray_dir0 = player.direction() - plane;
            let ray_dir1 = player.direction() + plane;

            for y_s in (h / 2 + 1)..h {
                let p = y_s as i32 - (h / 2) as i32;
                let row_distance = (0.5 * h as f32) / p as f32;

                let floor_step = row_distance * (ray_dir1 - ray_dir0) / w as f32;
                let mut floor_pos = player.position() + row_distance * ray_dir0;

                for x in 0..w {
                    let tx = ((f_t.width as f32 * (floor_pos.x - floor_pos.x.floor())) as u32)
                        % f_t.width;
                    let ty = ((f_t.height as f32 * (floor_pos.y - floor_pos.y.floor())) as u32)
                        % f_t.height;
                    floor_pos += floor_step;

                    let f_pixel = f_t.data[(ty * f_t.width + tx) as usize];
                    let c_pixel = c_t.data[(ty * c_t.width + tx) as usize];

                    let f_shaded = (f_pixel & 0xFEFEFEFE) >> 1 | 0xFF000000;

                    self.screen_buffer[y_s * w + x] = f_shaded;
                    self.screen_buffer[(h - 1 - y_s) * w + x] = c_pixel;
                }
            }
        }

        for x_s in 0..w {
            let camera_x = 2.0 * x_s as f32 / w as f32 - 1.0;
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
                if world.get_tile(map_pos.x as usize, map_pos.y as usize) != 0 {
                    hit = true;
                }
            }

            let perp_wall_dist = if side == 0 {
                (side_dist.x - delta_dist.x).max(0.01)
            } else {
                (side_dist.y - delta_dist.y).max(0.01)
            };

            let line_height = (h as f32 / perp_wall_dist) as i32;
            let start = (-line_height / 2 + h as i32 / 2).max(0);
            let end = (line_height / 2 + h as i32 / 2).min(h as i32 - 1);

            let tile_id = world.get_tile(map_pos.x as usize, map_pos.y as usize);
            if let Some(wall_tex) = texture_manager.get_texture(tile_id) {
                let mut wall_x = if side == 0 {
                    player.position().y + perp_wall_dist * ray_dir.y
                } else {
                    player.position().x + perp_wall_dist * ray_dir.x
                };
                wall_x -= wall_x.floor();

                let mut tex_x = (wall_x * wall_tex.width as f32) as u32;
                if (side == 0 && ray_dir.x > 0.0) || (side == 1 && ray_dir.y < 0.0) {
                    tex_x = wall_tex.width - 1 - (tex_x % wall_tex.width);
                }

                let step_tex = wall_tex.height as f32 / line_height as f32;
                let mut tex_pos =
                    (start as f32 - h as f32 / 2.0 + line_height as f32 / 2.0) * step_tex;

                for y in start..end {
                    let tex_y = (tex_pos as u32) % wall_tex.height;
                    tex_pos += step_tex;

                    let mut color = wall_tex.data[(tex_y * wall_tex.width + tex_x) as usize];
                    if side == 1 {
                        color = (color & 0xFEFEFEFE) >> 1 | 0xFF000000;
                    }
                    self.screen_buffer[y as usize * w + x_s] = color;
                }
            }
        }
    }

    fn render_debug_info(&mut self, world: &World, player: &Player) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..world.height() {
            for x in 0..world.width() {
                if world.get_tile(x, y) != 0 {
                    let _ = self
                        .canvas
                        .fill_rect(Rect::new((x as i32) * 3, (y as i32) * 3, 3, 3));
                }
            }
        }
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let _ = self.canvas.fill_rect(Rect::new(
            (player.position().x * 3.0) as i32,
            (player.position().y * 3.0) as i32,
            3,
            3,
        ));
    }

    pub fn set_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title).unwrap();
    }
}
