use std::{
    mem::transmute,
    sync::{Arc, Mutex},
    thread,
};

use nalgebra::{Matrix2, Vector2};
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    entities::EntityPtr,
    pixel_prog::*,
    player::Player,
    texture_manager::{TextureManager},
    world::World,
};

struct RenderJobWrapper {
    texture_manager: *const TextureManager,
    player: *const Player,
    world: *const World,
    screen_buffer: *mut Vec<u32>,
}

unsafe impl Send for RenderJobWrapper {}

unsafe impl Sync for RenderJobWrapper {}

const JOB_AMOUNT: usize = 5;

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    window_width: usize,
    window_height: usize,
    screen_buffer: Vec<u32>,
    z_buffer: Vec<f32>,
    texture: Texture<'a>,
    prog_manager: PixelProgManager,
    render_t: f32,
}

// TODO: generalny refactoring renderera, żeby był bardziej modularny i czytelny, można podzielić
// na pliki
// TODO: zrób drugą mapę w WORLD ale dla podług i sufitów
// TODO: ciekawe sprity typu drzwi, które można otwierać i zamykać
// TODO: do drzwi przyciski i inne fajne rzeczy

impl<'a> Renderer<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        window_width: usize,
        window_height: usize,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let screen_buffer = vec![0; window_width * window_height];
        // Initilize screen texture
        let texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ABGR8888,
                window_width as u32,
                window_height as u32,
            )
            .unwrap();

        Self {
            canvas,
            window_width,
            window_height,
            screen_buffer,
            texture,
            z_buffer: vec![0.0; window_width],
            prog_manager: PixelProgManager::new(),
            render_t: 0.
        }
    }

    pub fn render(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        self.screen_buffer.fill(0);
        self.render_world(world, player, texture_manager);

        self.texture
            .update(
                None,
                bytemuck::cast_slice(self.screen_buffer.as_slice()),
                self.window_width * 4,
            )
            .unwrap();
        let _ = self.canvas.copy(&self.texture, None, None);
        self.render_debug_info(world, player);
        self.canvas.present();

        self.render_t+=1.;

        self.prog_manager.update(self.render_t);
    }

    fn render_world(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        // TODO: make it argument for better control over fov
        let fov_multiplier = 1.2;
        let plane = Vector2::new(
            -player.direction().y * fov_multiplier,
            player.direction().x * fov_multiplier,
        );
        self.floor_cast(world, player, texture_manager, plane);
        self.wall_cast(world, player, plane, texture_manager);
        self.sprite_cast(player, world, texture_manager, plane);
    }

    fn floor_cast(
        &mut self,
        world: &World,
        player: &Player,
        texture_manager: &TextureManager,
        plane: Vector2<f32>,
    ) {
        let ray_dir0 = player.direction() - plane;
        let ray_dir1 = player.direction() + plane;

        let mut tasks: Vec<thread::JoinHandle<()>> = vec![];

        let len = self.window_height - (self.window_height / 2 + 1) + 1;

        let mut start = self.window_height / 2 + 1;
        let chunk_size = len / JOB_AMOUNT;

        for _ in 0..JOB_AMOUNT {
            let texture_manager: &TextureManager = unsafe { transmute(texture_manager) };
            let player: &Player = unsafe { transmute(player) };

            let world: &World = unsafe { transmute(world) };

            let window_width = self.window_width;
            let window_height = self.window_height;

            let screen_buffer: &mut Vec<u32> = unsafe { transmute(&mut self.screen_buffer) };

            let joined = thread::spawn(move || {
                for y_s in (start)..((start + chunk_size).min(window_height)) {
                    let p = (y_s - (window_height / 2)) as f32;
                    let row_distance = (0.5 * window_height as f32) / p;

                    let floor_step = row_distance * (ray_dir1 - ray_dir0) / window_width as f32;
                    let mut floor_pos = player.position() + row_distance * ray_dir0;

                    for x in 0..window_width {
                        let map_x = floor_pos.x as usize;
                        let map_y = floor_pos.y as usize;
                        let floor_id = world.get_floor_tile(map_x, map_y);
                        let ceil_id = world.get_ceil_tile(map_x, map_y);

                        let floor_tex = texture_manager.get_texture(floor_id);
                        let ceil_tex = texture_manager.get_texture(ceil_id);

                        if floor_tex.is_none() || ceil_tex.is_none() {
                            floor_pos += floor_step;
                            continue;
                        }

                        let f_t = floor_tex.unwrap();
                        let c_t = ceil_tex.unwrap();
                        let tx = (f_t.width as f32 * (floor_pos.x - floor_pos.x.floor())) as u32
                            % f_t.width;
                        let ty = (f_t.height as f32 * (floor_pos.y - floor_pos.y.floor())) as u32
                            % f_t.height;

                        let f_pixel = f_t.data[(ty * f_t.width + tx) as usize];
                        let c_pixel = c_t.data[(ty * c_t.width + tx) as usize];

                        let f_shaded = (f_pixel & 0xFEFEFEFE) >> 1 | 0xFF000000;

                        screen_buffer[y_s * window_width + x] = f_shaded;
                        screen_buffer[(window_height - 1 - y_s) * window_width + x] = c_pixel;

                        floor_pos += floor_step;
                    }
                }
            });

            tasks.push(joined);
            start += chunk_size;
        }

        for t in tasks {
            _ = t.join();
        }
    }

    fn wall_cast(
        &mut self,
        world: &World,
        player: &Player,
        plane: Vector2<f32>,
        texture_manager: &TextureManager,
    ) {
	   	let mut start = 0;
	    let chunk_size = self.window_width / JOB_AMOUNT;
		let mut tasks: Vec<thread::JoinHandle<()>> = vec![];

        for _ in 0..JOB_AMOUNT {
	       	let texture_manager: &TextureManager = unsafe { transmute(texture_manager) };
	        let player: &Player = unsafe { transmute(player) };

	        let world: &World = unsafe { transmute(world) };

	        let window_width = self.window_width;
	        let window_height = self.window_height;

	        let screen_buffer: &mut Vec<u32> = unsafe { transmute(&mut self.screen_buffer) };
			let z_buffer: &mut Vec<f32> = unsafe { transmute(&mut self.z_buffer) };

			let prog_manager: &PixelProgManager = unsafe { transmute(&self.prog_manager) };

			let render_t = self.render_t;

        	let joined = thread::spawn(move || {
	            for x_s in (start)..((start + chunk_size).min(window_width)) {
	                let camera_x = 2.0 * x_s as f32 / window_width as f32 - 1.0;
	                let ray_dir = player.direction() + plane * camera_x;

	                let mut map_pos =
	                    Vector2::new(player.position().x as i32, player.position().y as i32);
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
	                    if world.get_wall_tile(map_pos.x as usize, map_pos.y as usize) != 0 {
	                        hit = true;
	                    }
	                }

	                let perp_wall_dist = if side == 0 {
	                    (side_dist.x - delta_dist.x).max(0.01)
	                } else {
	                    (side_dist.y - delta_dist.y).max(0.01)
	                };

	                let line_height = (window_height as f32 / perp_wall_dist) as i32;
	                let start = (-line_height / 2 + window_height as i32 / 2).max(0);
	                let end = (line_height / 2 + window_height as i32 / 2)
	                    .min(window_height as i32 - 1);

	                let tile_id = world.get_wall_tile(map_pos.x as usize, map_pos.y as usize);
	                let wall_tex = texture_manager.get_texture(tile_id);
	                if wall_tex.is_none() {
	                    continue;
	                }
	                let wall_tex = wall_tex.unwrap();
	                let mut wall_x = if side == 0 {
	                    player.position().y + perp_wall_dist * ray_dir.y
	                } else {
	                    player.position().x + perp_wall_dist * ray_dir.x
	                };
	                wall_x -= wall_x.floor();

					let prog_t = prog_manager.dispatch_program(tile_id);

	                let mut tex_x = (wall_x * wall_tex.width as f32) as u32;
	                if (side == 0 && ray_dir.x > 0.0) || (side == 1 && ray_dir.y < 0.0) {
	                    tex_x = wall_tex.width - 1 - (tex_x % wall_tex.width);
	                }

	                let step_tex = wall_tex.height as f32 / line_height as f32;
	                let mut tex_pos = (start as f32 - window_height as f32 / 2.0
	                    + line_height as f32 / 2.0)
	                    * step_tex;

	                for y in start..end {
	                    let tex_y = (tex_pos as u32) % wall_tex.height;
	                    tex_pos += step_tex;

	                    let mut color = if prog_t.is_none() {wall_tex.data[(tex_y * wall_tex.width + tex_x) as usize]} else {
							prog_t.unwrap().color(
                                prog_manager.pipeline_state(),
                                tex_y as u32,
                                tex_x as u32,
                                render_t,
                            )
						};
	                    if side == 1 {
	                        color = (color & 0xFEFEFEFE) >> 1 | 0xFF000000;
	                    }
	                    screen_buffer[y as usize * window_width + x_s] = color;
	                }
		            z_buffer[x_s] = perp_wall_dist;
	            }
         	});

        	tasks.push(joined);
            start += chunk_size;
        }

        for t in tasks{
        	_ = t.join();
        }
    }

    fn sprite_cast(
        &mut self,
        player: &Player,
        world: &World,
        texture_manager: &TextureManager,
        plane: Vector2<f32>,
    ) {
        let entities = world.get_entities();
        let mut sorted_entities: Vec<&EntityPtr> = entities.iter().collect();
        sorted_entities.sort_unstable_by(|a, b| {
            let dist_a = (player.position() - a.position()).norm_squared();
            let dist_b = (player.position() - b.position()).norm_squared();
            dist_b.partial_cmp(&dist_a).unwrap()
        });

        for entity in sorted_entities {
            if entity.sprite_id().is_none() {
                continue;
            }

            let sprite_vec = entity.position() - player.position();

            let inv_det = 1.0 / (plane.x * player.direction().y - player.direction().x * plane.y);

            let transform = inv_det
                * Matrix2::new(
                    player.direction().y,
                    -player.direction().x,
                    -plane.y,
                    plane.x,
                )
                * sprite_vec;

            let sprite_screen_x =
                ((self.window_width as f32 / 2.0) * (1.0 + transform.x / transform.y)) as i32;
            let sprite_height = ((self.window_height as f32 / transform.y) as i32).abs();

            let start_y = (-sprite_height / 2 + (self.window_height as i32) / 2).max(0);
            let stop_y = (sprite_height / 2 + (self.window_height as i32) / 2)
                .min((self.window_height - 1) as i32);

            let sprite_width = ((self.window_height as f32 / transform.y) as i32).abs();

            let start_x = (-sprite_width / 2 + sprite_screen_x).max(0);
            let stop_x = (sprite_width / 2 + sprite_screen_x).min((self.window_width - 1) as i32);

            let sprite_id = entity.sprite_id().unwrap();
            let sprite_tex = texture_manager.get_texture(sprite_id);
            if sprite_tex.is_none() {
                continue;
            }
            let s_t = sprite_tex.unwrap();
            for stripe in start_x..stop_x {
                let tex_x = ((stripe - (sprite_screen_x - sprite_width / 2)) * s_t.width as i32)
                    / sprite_width;

                if transform.y > 0.0
                    && stripe > 0
                    && stripe < self.window_width as i32
                    && transform.y < self.z_buffer[stripe as usize]
                {
                    for y in start_y..stop_y {
                        let d = (y - (self.window_height as i32 / 2 - sprite_height / 2))
                            * s_t.height as i32;
                        let tex_y = d / sprite_height;

                        let s_pixel = s_t.data[(tex_y as u32 * s_t.width + tex_x as u32) as usize];

                        if (s_pixel & 0x00FFFFFF) != 0 {
                            self.screen_buffer[y as usize * self.window_width + stripe as usize] =
                                s_pixel | 0xFF000000;
                        }
                    }
                }
            }
        }
    }

    fn render_debug_info(&mut self, world: &World, player: &Player) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..world.height() {
            for x in 0..world.width() {
                if world.get_wall_tile(x, y) != 0 {
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
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        for entity in world.get_entities() {
            let _ = self.canvas.fill_rect(Rect::new(
                (entity.position().x * 3.0) as i32,
                (entity.position().y * 3.0) as i32,
                3,
                3,
            ));
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title).unwrap();
    }
}
