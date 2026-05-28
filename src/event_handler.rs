use sdl2::{EventPump, event::*, keyboard::*, mouse::MouseButton};

use crate::player::Player;

pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }

    pub fn pull_events(&mut self, player: &mut Player) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0);
                }
                Event::MouseMotion { xrel, .. } => {
                    player.rotate((xrel as f32) * 0.005);
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    println!("Shoot!");
                    player.shoot()
                }
                _ => {}
            }
        }

        let keyboard = self.event_pump.keyboard_state();

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
    }
}
