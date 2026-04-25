use std::f32;

use nalgebra::Vector2;
use psychotic::{player::Player, renderer::Renderer, world::World};
use sdl2::{
    event::*,
    keyboard::*,
    pixels::Color,
    rect::{Point, Rect},
};

fn main() {
    let mut world: World = World::new();
    let mut player: Player = Player::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut renderer: Renderer = Renderer::new(canvas, WIDTH, HEIGHT);
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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_time = std::time::Instant::now();

    'running: loop {
        renderer.render(&world, &player);

        // raycaster
        // POV

        //World Drawing

        // Player Drawing
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { xrel, .. } => {
                    player.rotate((xrel as f32) * 0.002);
                }
                _ => {}
            }
        }

        let keyboard = event_pump.keyboard_state();

        if keyboard.is_scancode_pressed(Scancode::Up) {
            player.up();
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            player.down();
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            player.left();
        }
        if keyboard.is_scancode_pressed(Scancode::Right) {
            player.right();
        }

        let current_time = std::time::Instant::now();
        delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;
        player.update(delta_time);
    }
}
