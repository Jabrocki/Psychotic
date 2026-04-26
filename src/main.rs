use std::f32;

use psychotic::{event_handler::EventHandler, player::Player, renderer::Renderer, world::World};

fn main() {
    let mut world: World = World::new();
    let mut player: Player = Player::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", WIDTH, HEIGHT)
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let mut renderer: Renderer = Renderer::new(canvas, WIDTH, HEIGHT);
    let mut event_handler: EventHandler = EventHandler::new(event_pump);
    let mut delta_time: f32;

    for x in 0..64 {
        for y in 0..64 {
            if x == 0 || x == 63 || y == 0 || y == 63 {
                world.set_tile(x, y, 1); // Zewnętrzne mury
            } else {
                world.set_tile(x, y, 0); // Pustka
            }
        }
    }

    for x in 50..54 {
        for y in 50..54 {
            world.set_tile(x, y, 1);
        }
    }

    for i in (10..54).step_by(8) {
        world.set_tile(i, 10, 1);
        world.set_tile(i, 50, 1);
    }

    for i in 0..6 {
        world.set_tile(10 + i, 20 + i, 1);
    }

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    let mut last_time = std::time::Instant::now();

    loop {
        renderer.render(&world, &player);
        event_handler.pull_events(&mut player);
        let current_time = std::time::Instant::now();
        delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;
        player.update(delta_time);
    }
}
