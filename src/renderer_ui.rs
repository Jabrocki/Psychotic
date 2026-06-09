use std::ops::Deref;

use nalgebra::Vector2;

use crate::image::Color4f;
use crate::texture_manager::{self, TextureManager};
use crate::{entity::unit::UnitEntity, world::World};
use crate::{player::Player, renderer::Renderer};

const HAND_ID: u8 = 20;

struct DrawRectCmd<'a> {
    start: Vector2<usize>,
    size: Vector2<usize>,
    edge_fade: f32,
    color: Color4f,
    tentacle_carve_cache_top: &'a [f32],
    tentacle_carve_cache_left: &'a [f32],
    tentacle_carve_cache_right: &'a [f32],
    tentacle_carve_cache_bottom: &'a [f32],
}

struct MiniMapInfo {
    start: Vector2<usize>,
    center: Vector2<usize>,
    entity_dot_size: usize,
    map_size: usize,
    grid_scale: f32,
}

impl<'a> Renderer<'a> {
    pub fn render_ui(&mut self, world: &World, player: &Player, texture_manager: &TextureManager) {
        self.render_hand(player, texture_manager);
        self.render_health_bar(player);
        self.render_minimap(world, player);

        for texel in self.screen_buffer.data.iter_mut(){
        	let other_colors = 1.0 - ((player.get_damage_effect() as f32) / 100. );
        	*texel = (Color4f::from_u32(texel.clone()) * Color4f(1., other_colors, other_colors, 1.)).to_u32();
        }
    }

    fn render_health_bar(&mut self, player: &Player) {
        let hp_ratio = player.hp() as f32 / player.max_hp() as f32;
        let bar_width = 600;
        let bar_height = 50;

        let start_x = (self.window_width.saturating_sub(bar_width)) / 2;
        let start_y = self.window_height.saturating_sub(bar_height + 100);

        let bg_color = Color4f(64.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0, 1.0);

        let bg_left = self.tentacle_carve(bar_height, 4.0, 10.0);
        let bg_right = self.tentacle_carve(bar_height, 2.0, 10.0);
        let bg_top = self.tentacle_carve(bar_width, 1.0, 8.0);
        let bg_bottom = self.tentacle_carve(bar_width, 3.0, 8.0);

        let bg_cmd = DrawRectCmd {
            start: Vector2::new(start_x, start_y),
            size: Vector2::new(bar_width, bar_height),
            edge_fade: 10.0,
            color: bg_color,
            tentacle_carve_cache_left: &bg_left,
            tentacle_carve_cache_right: &bg_right,
            tentacle_carve_cache_top: &bg_top,
            tentacle_carve_cache_bottom: &bg_bottom,
        };
        self.draw_noisy_moving_rect(&bg_cmd);

        let hp_color = Color4f(214.0 / 255.0, 9.0 / 255.0, 9.0 / 255.0, 1.0);
        let filled_width = (bar_width as f32 * hp_ratio) as usize;
        let filled_height = (bar_height as f32 * 0.8) as usize;
        let health_start_x = start_x;
        let health_start_y = start_y + (bar_height - filled_height) / 2;

        if filled_width > 0 && filled_height > 0 {
            let hp_left = self.tentacle_carve(filled_height, 4.0, 8.0);
            let hp_right = self.tentacle_carve(filled_height, 2.0, 8.0);
            let hp_top = self.tentacle_carve(filled_width, 1.0, 6.0);
            let hp_bottom = self.tentacle_carve(filled_width, 3.0, 6.0);

            let hp_cmd = DrawRectCmd {
                start: Vector2::new(health_start_x, health_start_y),
                size: Vector2::new(filled_width, filled_height),
                edge_fade: 8.0,
                color: hp_color,
                tentacle_carve_cache_left: &hp_left,
                tentacle_carve_cache_right: &hp_right,
                tentacle_carve_cache_top: &hp_top,
                tentacle_carve_cache_bottom: &hp_bottom,
            };
            self.draw_noisy_moving_rect(&hp_cmd);
        }
    }

    fn render_minimap(&mut self, world: &World, player: &Player) {
        let map_size = 300;
        let start_x = self.window_width.saturating_sub(map_size + 20);
        let start_y = 20;

        let minimap_info = MiniMapInfo {
            start: Vector2::new(start_x, start_y),
            center: Vector2::new(start_x + (map_size / 2), start_y + (map_size / 2)),
            entity_dot_size: 12,
            map_size,
            grid_scale: 15.0,
        };

        self.render_minimap_background(&minimap_info);
        self.render_minimap_walls(world, player, &minimap_info);
        self.render_minimap_entities(world, player, &minimap_info);
        self.render_minimap_player(player, &minimap_info);
    }

    fn render_minimap_background(&mut self, minimap_info: &MiniMapInfo) {
        let bg_color = Color4f(122.0 / 255.0, 81.0 / 255.0, 22.0 / 255.0, 0.8);
        let top_carve = 12.0;
        let side_carve = 10.0;

        let left_cache = self.tentacle_carve(minimap_info.map_size, 4.0, top_carve);
        let right_cache = self.tentacle_carve(minimap_info.map_size, 2.0, top_carve);
        let top_cache = self.tentacle_carve(minimap_info.map_size, 1.0, side_carve);
        let bottom_cache = self.tentacle_carve(minimap_info.map_size, 3.0, side_carve);

        let bg_cmd = DrawRectCmd {
            start: minimap_info.start,
            size: Vector2::new(minimap_info.map_size, minimap_info.map_size),
            edge_fade: 10.0,
            color: bg_color,
            tentacle_carve_cache_left: &left_cache,
            tentacle_carve_cache_right: &right_cache,
            tentacle_carve_cache_top: &top_cache,
            tentacle_carve_cache_bottom: &bottom_cache,
        };
        self.draw_noisy_moving_rect(&bg_cmd);
    }

    fn render_minimap_walls(&mut self, world: &World, player: &Player, minimap_info: &MiniMapInfo) {
        let position = player.position();
        let wall_color = Color4f(0.1, 0.1, 0.1, 1.0);
        let wall_size: isize = 22;
        let wall_usize = wall_size as usize;

        let start_i = Vector2::new(minimap_info.start.x as isize, minimap_info.start.y as isize);
        let map_size_i = minimap_info.map_size as isize;

        let top_carve = 5.0;
        let side_carve = 5.0;
        let cache_left = self.tentacle_carve(wall_usize, 4.0, top_carve);
        let cache_right = self.tentacle_carve(wall_usize, 2.0, top_carve);
        let cache_top = self.tentacle_carve(wall_usize, 1.0, side_carve);
        let cache_bottom = self.tentacle_carve(wall_usize, 3.0, side_carve);

        for dx in -10isize..=10isize {
            for dy in -10isize..=10isize {
                let world_x_f = (position.x + dx as f32).floor();
                let world_y_f = (position.y + dy as f32).floor();

                if world.get_wall_tile(world_x_f as usize, world_y_f as usize) != 0 {
                    let dist_x = (world_x_f + 0.5) - position.x;
                    let dist_y = (world_y_f + 0.5) - position.y;

                    let tile_center_x =
                        minimap_info.center.x as f32 + (dist_x * minimap_info.grid_scale);
                    let tile_center_y =
                        minimap_info.center.y as f32 + (dist_y * minimap_info.grid_scale);

                    let wall_x = (tile_center_x - (wall_size as f32 / 2.0)) as isize;
                    let wall_y = (tile_center_y - (wall_size as f32 / 2.0)) as isize;

                    if wall_x < start_i.x
                        || wall_x > start_i.x + map_size_i - wall_size
                        || wall_y < start_i.y
                        || wall_y > start_i.y + map_size_i - wall_size
                    {
                        continue;
                    }

                    let wall_cmd = DrawRectCmd {
                        start: Vector2::new(wall_x as usize, wall_y as usize),
                        size: Vector2::new(wall_usize, wall_usize),
                        edge_fade: 7.0,
                        color: wall_color,
                        tentacle_carve_cache_left: &cache_left,
                        tentacle_carve_cache_right: &cache_right,
                        tentacle_carve_cache_top: &cache_top,
                        tentacle_carve_cache_bottom: &cache_bottom,
                    };
                    self.draw_noisy_moving_rect(&wall_cmd);
                }
            }
        }
    }

    fn render_minimap_entities(
        &mut self,
        world: &World,
        player: &Player,
        minimap_info: &MiniMapInfo,
    ) {
        let position = player.position();
        let size = minimap_info.entity_dot_size;
        let entity_color = Color4f(0.2, 0.1, 0.6, 1.0);

        let top_carve = 3.0;
        let side_carve = 3.0;
        let cache_left = self.tentacle_carve(size, 4.0, top_carve);
        let cache_right = self.tentacle_carve(size, 2.0, top_carve);
        let cache_top = self.tentacle_carve(size, 1.0, side_carve);
        let cache_bottom = self.tentacle_carve(size, 3.0, side_carve);

        for entity_ptr in world.get_entities() {
            let entity_position = entity_ptr.position();
            let dist_x = entity_position.x - position.x;
            let dist_y = entity_position.y - position.y;

            let entity_x = (minimap_info.center.x as isize
                + (dist_x * minimap_info.grid_scale) as isize
                - (size as isize / 2)) as usize;
            let entity_y = (minimap_info.center.y as isize
                + (dist_y * minimap_info.grid_scale) as isize
                - (size as isize / 2)) as usize;

            if entity_x < minimap_info.start.x
                || entity_x > minimap_info.start.x + minimap_info.map_size - size
                || entity_y < minimap_info.start.y
                || entity_y > minimap_info.start.y + minimap_info.map_size - size
            {
                continue;
            }

            let entity_cmd = DrawRectCmd {
                start: Vector2::new(entity_x, entity_y),
                size: Vector2::new(size, size),
                edge_fade: 5.0,
                color: entity_color,
                tentacle_carve_cache_left: &cache_left,
                tentacle_carve_cache_right: &cache_right,
                tentacle_carve_cache_top: &cache_top,
                tentacle_carve_cache_bottom: &cache_bottom,
            };
            self.draw_noisy_moving_rect(&entity_cmd);
        }
    }

    fn render_minimap_player(&mut self, _player: &Player, minimap_info: &MiniMapInfo) {
        let size = 12;
        let player_color = Color4f(0.9, 0.1, 0.1, 1.0);

        let player_x = minimap_info.start.x + (minimap_info.map_size / 2) - (size / 2);
        let player_y = minimap_info.start.y + (minimap_info.map_size / 2) - (size / 2);

        let top_carve = 3.0;
        let side_carve = 3.0;

        let left_cache = self.tentacle_carve(size, 4.0, top_carve);
        let right_cache = self.tentacle_carve(size, 2.0, top_carve);
        let top_cache = self.tentacle_carve(size, 1.0, side_carve);
        let bottom_cache = self.tentacle_carve(size, 3.0, side_carve);

        let player_cmd = DrawRectCmd {
            start: Vector2::new(player_x, player_y),
            size: Vector2::new(size, size),
            edge_fade: 5.0,
            color: player_color,
            tentacle_carve_cache_left: &left_cache,
            tentacle_carve_cache_right: &right_cache,
            tentacle_carve_cache_top: &top_cache,
            tentacle_carve_cache_bottom: &bottom_cache,
        };
        self.draw_noisy_moving_rect(&player_cmd);
    }

    fn render_hand(&mut self, player: &Player, texture_manager: &TextureManager) {
        if let Some(hand_texture) = texture_manager.get_texture(HAND_ID) {
            let tex_width = hand_texture.width;
            let tex_height = hand_texture.height;

            let target_height = (self.window_height as f32 * 0.9) as usize;

            let aspect_ratio = tex_width as f32 / tex_height as f32;
            let target_width = (target_height as f32 * aspect_ratio) as usize;

            let start_x = (self.window_width.saturating_sub(target_width - 10));
            let start_y = self.window_height.saturating_sub(target_height - 100);

            for y in 0..target_height {
                for x in 0..target_width {
                    let tex_x = ((x as f32 / target_width as f32) * tex_width as f32) as usize;
                    let tex_y = ((y as f32 / target_height as f32) * tex_height as f32) as usize;

                    let clamped_tex_x = tex_x.min(tex_width - 1);
                    let clamped_tex_y = tex_y.min(tex_height - 1);

                    let tex_idx = clamped_tex_y * tex_width + clamped_tex_x;
                    let color = hand_texture.data[tex_idx];
                    let alpha = (color >> 24) & 0xFF;
                    if alpha == 0 {
                        continue;
                    }

                    let screen_x = start_x + x;
                    let screen_y = start_y + y;

                    if screen_x < self.window_width && screen_y < self.window_height {
                        let buffer_idx = screen_y * self.window_width + screen_x;
                        self.screen_buffer.data[buffer_idx] = color;
                    }
                }
            }
        }
    }

    fn tentacle_carve(&self, size: usize, phase: f32, max_carve: f32) -> Vec<f32> {
        let time = self.render_t as f32;

        let tentacle_carve_fn = |pos: f32, t: f32, phase: f32, max_carve: f32| -> f32 {
            let wave1 = ((pos * 0.15 + t * 0.05 + phase).sin() * 0.5 + 0.5).powi(3);
            let wave2 = ((pos * 0.25 - t * 0.03 + phase).cos() * 0.5 + 0.5).powi(2);
            let base = (pos * 0.05 + t * 0.1).sin() * 0.5 + 0.5;

            (wave1 * 0.6 + wave2 * 0.3 + base * 0.1) * max_carve
        };

        (0..size)
            .map(|i| tentacle_carve_fn(i as f32, time, phase, max_carve))
            .collect()
    }

    fn draw_noisy_moving_rect(&mut self, draw_cmd: &DrawRectCmd<'_>) {
        let start_y = draw_cmd.start.y;
        let end_y = draw_cmd.start.y + draw_cmd.size.y;
        let start_x = draw_cmd.start.x;
        let end_x = draw_cmd.start.x + draw_cmd.size.x;

        for y in start_y..end_y {
            let y_idx = y - start_y;
            let carve_left = draw_cmd.tentacle_carve_cache_left[y_idx];
            let carve_right = draw_cmd.tentacle_carve_cache_right[y_idx];
            let y_f = y as f32;

            for x in start_x..end_x {
                let x_idx = x - start_x;
                let x_f = x as f32;

                let dist_left = (x_f - start_x as f32) - carve_left;
                let dist_right = ((start_x + draw_cmd.size.x) as f32 - x_f) - carve_right;

                let dist_top = (y_f - start_y as f32) - draw_cmd.tentacle_carve_cache_top[x_idx];
                let dist_bottom = ((start_y + draw_cmd.size.y) as f32 - y_f)
                    - draw_cmd.tentacle_carve_cache_bottom[x_idx];

                let min_dist_x = dist_left.min(dist_right);
                let min_dist_y = dist_top.min(dist_bottom);
                let min_dist = min_dist_x.min(min_dist_y);

                if min_dist < 0.0 {
                    continue;
                }

                let mut shade = 1.0;
                if min_dist < draw_cmd.edge_fade {
                    shade = min_dist / draw_cmd.edge_fade;
                }

                let mut h =
                    (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(3266489917);
                h = h.rotate_left(17).wrapping_mul(668265263);
                let noise = (h as f32 / u32::MAX as f32) * 0.15;

                let final_color = draw_cmd.color * shade * (1.0 - noise);
                self.screen_buffer[(x, y)] = final_color.to_u32();
            }
        }
    }
}
