use psychotic::game::Game;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Psychotic", 800, 600)
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new(800, 600, &texture_creator, canvas, event_pump);

    game.run();
}
