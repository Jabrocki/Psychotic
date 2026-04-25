use psychotic::world::{Tile, World};
use sdl2::{event::*, keyboard::*, pixels::Color, rect::{Point, Rect}, sys::*};

fn main() {
	let mut world: World = World::new();

	for y in 0..64{

		world.set_tile(0, y, 1);

		world.set_tile(63, y, 1);

	}

	for x in 0..64{

		world.set_tile(x, 0, 1);

		world.set_tile(x, 63, 1);

	}

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 800).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
   		canvas.set_draw_color(Color::RGB(0, 0, 0));
   		canvas.clear();
     	canvas.set_draw_color(Color::RGB(255, 255, 255));
   		//canvas.draw_line(Point::new(0, 0), Point::new(832, 576));

     	for y in 0..world.height(){
    		for x in 0..world.width(){
   				let tile = world.reg().get(&world.get_tile(x, y)).unwrap();
   				canvas.set_draw_color(tile.color());

       			let rect = Rect::new((x as i32)*10, (y as i32)*10, 10, 10);

       			canvas.fill_rect(rect);
      		}
      	}

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
    }
}
