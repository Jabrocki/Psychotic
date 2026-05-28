use std::mem::transmute;
use std::ops::DerefMut;
use std::{isize, usize};

use nalgebra::max;

use crate::image::Color4f;
use crate::{entity::unit::UnitEntity, world::World};
use crate::{player::Player, renderer::Renderer};
impl<'a> Renderer<'a> {
    pub fn render_ui(&mut self, world: &World, player: &Player) {
        self.render_health_bar(player);
        self.render_minimap(world, player);
    }

    fn render_health_bar(&mut self, player: &Player) {
        let hp_ratio = player.hp() as f32 / player.max_hp() as f32;
        let bar_width = 600;
        let bar_height = 50;

        let start_x = (self.window_width.saturating_sub(bar_width)) / 2;
        let start_y = self.window_height.saturating_sub(bar_height + 100);

        let bg_color = Color4f(64.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0, 1.0);
        self.draw_noisy_moving_rect(
            start_x, start_y, bar_width, bar_height, bg_color, 10.0, 8.0, 10.0,
        );

        let hp_color = Color4f(214.0 / 255.0, 9.0 / 255.0, 9.0 / 255.0, 1.0);
        let filled_width = (bar_width as f32 * hp_ratio) as usize;
        let filled_height = (bar_height as f32 * 0.8) as usize;
        let health_start_x = start_x;
        let health_start_y = start_y + (bar_height - filled_height) / 2;

        self.draw_noisy_moving_rect(
            health_start_x,
            health_start_y,
            filled_width,
            filled_height,
            hp_color,
            8.0,
            6.0,
            8.0,
        );
    }

    fn render_minimap(&mut self, world: &World, player: &Player) {
        // Draw back ground fo the map
        let map_size = 300;
        let start_x = self.window_width.saturating_sub(map_size + 20);
        let start_y = 20;
        // 122, 81, 22
        let bg_color = Color4f(122.0 / 255.0, 81.0 / 255.0, 22.0 / 255.0, 0.8);
        self.draw_noisy_moving_rect(
            start_x, start_y, map_size, map_size, bg_color, 12.0, 10.0, 10.0,
        );

        let position = player.position();

        let center_x = start_x + (map_size / 2);
        let center_y = start_y + (map_size / 2);

        // Draw world
        let wall_color = Color4f(0.1, 0.1, 0.1, 1.0);
        let wall_size: isize = 22; // Ściana powiększona do 22
        let grid_scale = 15.0;

        for dx in -10isize..=10isize {
            for dy in -10isize..=10isize {
                let world_x_f = (position.x + dx as f32).floor();
                let world_y_f = (position.y + dy as f32).floor();

                if world.get_wall_tile(world_x_f as usize, world_y_f as usize) != 0 {
                    let dist_x = (world_x_f + 0.5) - position.x;
                    let dist_y = (world_y_f + 0.5) - position.y;

                    let tile_screen_center_x = center_x as f32 + (dist_x * grid_scale);
                    let tile_screen_center_y = center_y as f32 + (dist_y * grid_scale);

                    let wall_x = (tile_screen_center_x - (wall_size as f32 / 2.0)) as isize;
                    let wall_y = (tile_screen_center_y - (wall_size as f32 / 2.0)) as isize;

                    let start_x_i = start_x as isize;
                    let start_y_i = start_y as isize;
                    let map_size_i = map_size as isize;

                    if wall_x < start_x_i
                        || wall_x > start_x_i + map_size_i - wall_size
                        || wall_y < start_y_i
                        || wall_y > start_y_i + map_size_i - wall_size
                    {
                        continue;
                    }

                    self.draw_noisy_moving_rect(
                        wall_x as usize,
                        wall_y as usize,
                        wall_size as usize,
                        wall_size as usize,
                        wall_color,
                        5.0,
                        5.0,
                        7.0,
                    );
                }
            }
        }

        // ... (rysowanie jednostek entities) ...
        // Dla entity_dot_size również warto ustawić mniejszą wartość, np. 8, z tego samego powodu.

        // Player in the center of the map
        let player_dot_size = 8; // Zmniejszone do 8, aby zapobiec nakładaniu przy wall_size = 22
        let player_dot_color = Color4f(0.9, 0.1, 0.1, 1.0);

        let player_dot_x = start_x + (map_size / 2) - (player_dot_size / 2);
        let player_dot_y = start_y + (map_size / 2) - (player_dot_size / 2);
        self.draw_noisy_moving_rect(
            player_dot_x,
            player_dot_y,
            player_dot_size,
            player_dot_size,
            player_dot_color,
            3.0,
            3.0,
            5.0,
        );
        // Draw entities
        let entity_dot_size = 12;
        let entity_dot_color = Color4f(0.2, 0.1, 0.6, 1.0);
        for entity_ptr in world.get_entities() {
            let entity_position = entity_ptr.position();
            let dist_x = entity_position.x - position.x;
            let dist_y = entity_position.y - position.y;

            let entity_x = (center_x as isize + (dist_x * grid_scale) as isize
                - (entity_dot_size as isize / 2)) as usize;
            let entity_y = (center_y as isize + (dist_y * grid_scale) as isize
                - (entity_dot_size as isize / 2)) as usize;

            if entity_x < start_x
                || entity_x > start_x + map_size - entity_dot_size
                || entity_y < start_y
                || entity_y > start_y + map_size - entity_dot_size
            {
                continue;
            }

            self.draw_noisy_moving_rect(
                entity_x,
                entity_y,
                entity_dot_size,
                entity_dot_size,
                entity_dot_color,
                3.0,
                3.0,
                5.0,
            );
        }

        // 4. Zmniejsz rozmiar gracza, aby fizycznie wyglądał jak obiekt mniejszy niż kafelek ściany
        let player_dot_size = 12;
        let player_dot_color = Color4f(0.9, 0.1, 0.1, 1.0);

        let player_dot_x = start_x + (map_size / 2) - (player_dot_size / 2);
        let player_dot_y = start_y + (map_size / 2) - (player_dot_size / 2);
        self.draw_noisy_moving_rect(
            player_dot_x,
            player_dot_y,
            player_dot_size,
            player_dot_size,
            player_dot_color,
            3.0,
            3.0,
            5.0,
        );
    }

    fn render_hand(&mut self) {}

    fn draw_noisy_moving_rect(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        base_color: Color4f,
        top_crave: f32,
        side_crave: f32,
        edge_fade: f32,
    ) {
        let time = self.render_t as f32;

        let tentacle_carve = |pos: f32, t: f32, phase: f32, max_carve: f32| -> f32 {
            let wave1 = ((pos * 0.15 + t * 0.05 + phase).sin() * 0.5 + 0.5).powi(3);
            let wave2 = ((pos * 0.25 - t * 0.03 + phase).cos() * 0.5 + 0.5).powi(2);
            let base = (pos * 0.05 + t * 0.1).sin() * 0.5 + 0.5;

            (wave1 * 0.6 + wave2 * 0.3 + base * 0.1) * max_carve
        };

        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                let x_f = x as f32;
                let y_f = y as f32;

                let dist_left = (x_f - start_x as f32) - tentacle_carve(y_f, time, 4.0, top_crave);
                let dist_right =
                    ((start_x + width) as f32 - x_f) - tentacle_carve(y_f, time, 2.0, top_crave);
                let dist_top = (y_f - start_y as f32) - tentacle_carve(x_f, time, 1.0, side_crave);
                let dist_bottom =
                    ((start_y + height) as f32 - y_f) - tentacle_carve(x_f, time, 3.0, side_crave);

                let min_dist_x = dist_left.min(dist_right);
                let min_dist_y = dist_top.min(dist_bottom);
                let min_dist = min_dist_x.min(min_dist_y);

                if min_dist < 0.0 {
                    continue;
                }

                let mut shade = 1.0;
                if min_dist < edge_fade {
                    shade = min_dist / edge_fade;
                }

                let seed = (x_f * 12.9898) + (y_f * 78.233);
                let noise = (seed.sin() * 43758.5453).fract();

                let final_color = base_color * shade * (1.0 - noise * 0.15);
                self.screen_buffer[(x, y)] = final_color.to_u32();
            }
        }
    }
}
