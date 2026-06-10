use std::time::Instant;

use nalgebra::Vector2;
use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{
    entities::EntityPtr,
    entity::enemy::Enemy,
    event_handler::EventHandler,
    level::{
        level::{Level, RoomData},
        room_file::RoomFile,
    },
    player::Player,
    renderer::Renderer,
    texture_manager::TextureManager,
    world::World,
};

pub struct Game<'a> {
    world: World,
    player: &'static mut Player,
    renderer: Renderer<'a>,
    event_handler: EventHandler,
    texture_manager: TextureManager,
}

impl<'a> Game<'a> {
    pub fn new(
        width: usize,
        height: usize,
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: sdl2::EventPump,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let mut world = World::new(vec![], vec![], vec![]);
        let mut player = Player::new();
        let renderer = Renderer::new(canvas, width, height, texture_creator);
        let event_handler = EventHandler::new(event_pump);
        let mut texture_manager = TextureManager::new();

        // Load textures
        texture_manager.load_static(1, "images/colorstone.png");
        texture_manager.load_static(2, "images/greystone.png");
        texture_manager.load_static(3, "images/wood.png");
        texture_manager.load_static(4, "images/redbrick.png");

        // Load sprites
        texture_manager.load_static(10, "images/pillar.png");
        texture_manager.load_static(11, "images/hand_1.png");
        texture_manager.load_static(12, "images/hand_2.png");

        // Load ui
        texture_manager.load_static(20, "images/pointing.png");

        // World hardcoded for now
        let small_room_file = RoomFile::from_file("rooms/small_room.json");
        let medium_room_file = RoomFile::from_file("rooms/medium_room.json");

        let room_data = vec![
            RoomData {
                room_file: small_room_file,
                count: 150,
            },
            RoomData {
                room_file: medium_room_file,
                count: 110,
            },
        ];

        let mut level_generator = Level::new(100, 100);
        level_generator.generate(room_data);

        world.width = level_generator.get_width();
        world.height = level_generator.get_height();
        world.wall_map = level_generator.wall_map;

        // Find first 0 in wall_map to place player
        let mut player_placed = false;
        for y in 0..world.height {
            for x in 0..world.width {
                if world.get_wall_tile(x, y) == 0 {
                    player.set_position(Vector2::new(x as f32 + 0.5, y as f32 + 0.5));
                    player_placed = true;
                    break;
                }
            }
            if player_placed {
                break;
            }
        }
        world.ceil_map = level_generator.ceil_map;
        world.floor_map = level_generator.floor_map;

        //world.add_entity_dummy(Vector2::new(player.position().x, player.position().y), 10);
        //world.add_entity_dummy(Vector2::new(7.5, 5.5), 10);
        let procent_of_enemies = 0.08;

        for _ in 0..((world.width * world.height) as f32 * procent_of_enemies) as usize {
            let enemy_position = loop {
                let x = (rand::random::<u32>() % world.width as u32) as usize;
                let y = (rand::random::<u32>() % world.height as u32) as usize;
                if world.get_wall_tile(x, y) == 0 {
                    break Vector2::new(x as f32 + 0.5, y as f32 + 0.5);
                }
            };
            let enemy = Enemy::new(enemy_position);
            world.force_add_entity_direct(enemy);
        }

        let procent_of_dummies = 0.01;

        for _ in 0..((world.width * world.height) as f32 * procent_of_dummies) as usize {
            let dummy_position = loop {
                let x = (rand::random::<u32>() % world.width as u32) as usize;
                let y = (rand::random::<u32>() % world.height as u32) as usize;
                if world.get_wall_tile(x, y) == 0 {
                    break Vector2::new(x as f32 + 0.5, y as f32 + 0.5);
                }
            };
            world.add_entity_dummy(dummy_position, 10);
        }

        let mut player_ptr = EntityPtr::new(player);
        let player = player_ptr.direct();

        world.player = Some(player);
        world.player_id = Some(player_ptr.id());

        world.force_add_entity(player_ptr);

        Self {
            world,
            player,
            renderer,
            event_handler,
            texture_manager,
        }
    }
    pub fn run(&mut self) {
        let mut delta_time: f32;
        let mut last_time = std::time::Instant::now();
        let mut frame_count = 0;
        let mut timer = Instant::now();

        loop {
            let current_time = std::time::Instant::now();
            delta_time = (current_time - last_time).as_secs_f32();
            last_time = current_time;
            self.renderer
                .render(&self.world, self.player, &self.texture_manager);
            self.event_handler.pull_events(self.player);

            self.world.update(delta_time);

            frame_count += 1;
            if timer.elapsed().as_secs() >= 1 {
                println!("FPS: {}", frame_count);
                frame_count = 0;
                timer = Instant::now();
            }
        }
    }
}
