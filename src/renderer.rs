use nalgebra::Vector2;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{player::Player, world::World};

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

    pub fn render(&mut self, world: &World, player: &Player) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.render_world(world, player);
        self.render_debug_info(world, player);

        self.canvas.present();
    }

    fn render_world(&mut self, world: &World, player: &Player) {
        let plane = Vector2::new(-player.direction().y * 0.66, player.direction().x * 0.66);

        for x_s in 0..self.window_width {
            let camera_x = 2.0 * x_s as f32 / self.window_width as f32 - 1.0;
            let ray_dir = player.direction() + plane * camera_x;

            let mut map_pos = Vector2::new(player.position().x as i32, player.position().y as i32);

            let delta_dist = Vector2::new(1.0, 1.0).component_div(&ray_dir).abs();

            let step = ray_dir.map(|c| c.signum() as i32);

            let mut side_dist = Vector2::new(
                // Współrzedna X
                if ray_dir.x < 0.0 {
                    (player.position().x - map_pos.x as f32) * delta_dist.x
                } else {
                    (map_pos.x as f32 + 1.0 - player.position().x) * delta_dist.x
                },
                // Współrzedna Y
                if ray_dir.y < 0.0 {
                    (player.position().y - map_pos.y as f32) * delta_dist.y
                } else {
                    (map_pos.y as f32 + 1.0 - player.position().y) * delta_dist.y
                },
            );

            let side = loop {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    if world.get_tile(map_pos.x as usize, map_pos.y as usize) != 0 {
                        break 0;
                    }
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;
                    if world.get_tile(map_pos.x as usize, map_pos.y as usize) != 0 {
                        break 1;
                    }
                }
            };

            let perp_wall_dist = if side == 0 {
                side_dist.x - delta_dist.x
            } else {
                side_dist.y - delta_dist.y
            };

            let line_height = (self.window_height as f32 / perp_wall_dist) as i32;

            let start = (-line_height / 2 + self.window_height as i32 / 2).max(0);
            let end = (line_height / 2 + self.window_height as i32 / 2)
                .min(self.window_height as i32 - 1);

            let mut color = world
                .reg()
                .get(&world.get_tile(map_pos.x as usize, map_pos.y as usize))
                .unwrap()
                .color();

            if side == 1 {
                color = Color::RGB(color.r / 2, color.g / 2, color.b / 2);
            }

            self.canvas.set_draw_color(color);
            self.canvas
                .draw_line(Point::new(x_s as i32, start), Point::new(x_s as i32, end));
        }
    }

    fn render_debug_info(&mut self, world: &World, player: &Player) {
        for y in 0..world.height() {
            for x in 0..world.width() {
                let tile = world.reg().get(&world.get_tile(x, y)).unwrap();
                self.canvas.set_draw_color(tile.color());

                let rect = Rect::new((x as i32) * 3, (y as i32) * 3, 3, 3);
                if world.get_tile(x, y) != 0 {
                    self.canvas.fill_rect(rect);
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
        self.canvas.fill_rect(rect);
    }
}
