use psychotic::{
    player::Player,
    world::{Tile, World},
};
use sdl2::{
    event::*,
    keyboard::*,
    pixels::Color,
    rect::{Point, Rect},
    sys::*,
};

fn main() {
    let mut world: World = World::new();
    let mut player: Player = Player::new();
    let mut delta_time: f32;

    for y in 0..64 {
        world.set_tile(0, y, 1);

        world.set_tile(63, y, 1);
    }

    for x in 0..64 {
        world.set_tile(x, 0, 1);

        world.set_tile(x, 63, 1);
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 800).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_time = std::time::Instant::now();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        //canvas.draw_line(Point::new(0, 0), Point::new(832, 576));

        //World Drawing
        for y in 0..world.height() {
            for x in 0..world.width() {
                let tile = world.reg().get(&world.get_tile(x, y)).unwrap();
                canvas.set_draw_color(tile.color());

                let rect = Rect::new((x as i32) * 10, (y as i32) * 10, 10, 10);

                canvas.fill_rect(rect);
            }
        }

        // Player Drawing
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            (player.position().x as i32) * 10,
            (player.position().y as i32) * 10,
            10,
            10,
        );
        canvas.fill_rect(rect);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => player.left(),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => player.right(),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => player.up(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => player.down(),
                _ => {}
            }
        }
        let current_time = std::time::Instant::now();
        delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;
        player.update(delta_time);

        canvas.present();
    }
}
