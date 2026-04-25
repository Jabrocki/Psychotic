use nalgebra::Vector2;
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

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", WIDTH, HEIGHT)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_time = std::time::Instant::now();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        //canvas.draw_line(Point::new(0, 0), Point::new(832, 576));

        // raycaster

        let dir: Vector2<f32> = Vector2::new(1.0, 0.0);
        let plane: Vector2<f32> = Vector2::new(0.0, 0.66);

        for x_s in 0..WIDTH {
            let cameraProg = (((x_s as f32) / (WIDTH as f32)) - 0.5) * 2.0;
            let mut rayDir: Vector2<f32> = dir + plane * cameraProg;

            let mut map_pos: Vector2<f32> = Vector2::new(
                (player.position().x as i32) as f32,
                (player.position().y as i32) as f32,
            );

            let deltaDistX = (1.0 / rayDir.x).abs();
            let deltaDistY = (1.0 / rayDir.y).abs();

            let mut step: Vector2<f32> = Vector2::new(0.0, 0.0);

            let mut sideDistX = 0.0;
            let mut sideDistY = 0.0;

            //
            if rayDir.x < 0.0 {
                step.x = -1.0;
                sideDistX = (player.position().x - map_pos.x) * deltaDistX;
            } else {
                step.x = 1.0;
                sideDistX = (map_pos.x + 1.0 - player.position().x) * deltaDistX;
            }
            if rayDir.y < 0.0 {
                step.y = -1.0;
                sideDistY = (player.position().y - map_pos.y) * deltaDistY;
            } else {
                step.y = 1.0;
                sideDistY = (map_pos.y + 1.0 - player.position().y) * deltaDistY;
            }

            let mut hit = false;

            let mut side = 0;

            // DDA
            while !hit {
                if sideDistX < sideDistY {
                    sideDistX += deltaDistX;
                    map_pos.x += step.x;
                    side = 0;
                }
                else {
                	sideDistY += deltaDistY;
                 	map_pos.y += step.y;
                  	side = 1;
                }

                if world.get_tile(map_pos.x as usize, map_pos.y as usize) != 0{
                	hit = true;
                }
            }

            let mut perpWallDist: f32 = 0.0;

            if side == 0 {
           		perpWallDist = sideDistX - deltaDistX;
            } else {
            	perpWallDist = sideDistY - deltaDistY;
            }

            // based on dist from wall calculate line height
            let mut lineHeight = (HEIGHT as f32) / perpWallDist;

            let mut start = (HEIGHT / 2) as i32 + (-lineHeight / 2.0) as i32;
            let mut end = (HEIGHT / 2) as i32 + (lineHeight / 2.0) as i32;

            start = start.max(0);
            end = end.min((HEIGHT - 1) as i32);

            let mut tile_color = world.reg()
            	.get(&world.get_tile(map_pos.x as usize, map_pos.y as usize))
             	.unwrap().color();

            if side == 1{
          		tile_color = Color::RGB(tile_color.r/2, tile_color.g/2, tile_color.b/2);
            }

            canvas.set_draw_color(tile_color);
            canvas.draw_line(Point::new(x_s as i32, start), Point::new(x_s as i32, end));

        }

        //World Drawing
        for y in 0..world.height() {
            for x in 0..world.width() {
                let tile = world.reg().get(&world.get_tile(x, y)).unwrap();
                canvas.set_draw_color(tile.color());

                let rect = Rect::new((x as i32) * 3, (y as i32) * 3, 3, 3);

                canvas.fill_rect(rect);
            }
        }

        // Player Drawing
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            (player.position().x as i32) * 3,
            (player.position().y as i32) * 3,
            3,
            3,
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
