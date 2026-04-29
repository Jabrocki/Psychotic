use std::time::Instant;

use nalgebra::Vector2;
use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{
    entity_manger::EntityManager, event_handler::EventHandler, player::Player, renderer::Renderer,
    texture_manager::TextureManager, world::World,
};

pub struct Game<'a> {
    world: World,
    player: Player,
    renderer: Renderer<'a>,
    event_handler: EventHandler,
    texture_manager: TextureManager,
    entity_manager: EntityManager,
}

impl<'a> Game<'a> {
    pub fn new(
        width: u32,
        height: u32,
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: sdl2::EventPump,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        // Example 256 x 256 world
        let world = World::new(vec![
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1,
            ],
        ]);

        let player = Player::new();
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

        // Entity manager hardcoded for now
        let mut entity_manager = EntityManager::new();
        entity_manager.add_entity(Vector2::new(5.5, 5.5), 10);
        entity_manager.add_entity(Vector2::new(7.5, 5.5), 10);

        entity_manager.add_entity(Vector2::new(2.0, 2.0), 10);
        Self {
            world,
            player,
            renderer,
            event_handler,
            texture_manager,
            entity_manager,
        }
    }
    pub fn run(&mut self) {
        let mut delta_time: f32;
        let mut last_time = std::time::Instant::now();
        let mut frame_count = 0;
        let mut timer = Instant::now();

        loop {
            self.renderer.render(
                &self.world,
                &self.player,
                &self.texture_manager,
                &self.entity_manager,
            );
            self.event_handler.pull_events(&mut self.player);
            let current_time = std::time::Instant::now();
            delta_time = (current_time - last_time).as_secs_f32();
            last_time = current_time;
            self.player.update(delta_time);

            frame_count += 1;
            if timer.elapsed().as_secs() >= 1 {
                println!("FPS: {}", frame_count);
                frame_count = 0;
                timer = Instant::now();
            }
        }
    }
}
