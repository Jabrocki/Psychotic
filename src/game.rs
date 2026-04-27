use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{
    event_handler::EventHandler, player::Player, renderer::Renderer,
    texture_manager::TextureManager, world::World,
};

pub struct Game<'a> {
    world: World,
    player: Player,
    renderer: Renderer<'a>,
    event_handler: EventHandler,
    texture_manager: TextureManager,
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
        texture_manager.load_static(4, "images/colorstone.png");

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

        loop {
            self.renderer
                .render(&self.world, &self.player, &self.texture_manager);
            self.event_handler.pull_events(&mut self.player);
            let current_time = std::time::Instant::now();
            delta_time = (current_time - last_time).as_secs_f32();
            last_time = current_time;
            self.player.update(delta_time);
        }
    }
}
