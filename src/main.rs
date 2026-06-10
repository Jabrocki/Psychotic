use psychotic::game::Game;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Psychotic", 1920, 1080)
        .fullscreen_desktop()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let event_pump = sdl_context.event_pump().unwrap();

    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut game = Game::new(1920, 1080, canvas, event_pump, &texture_creator);

    game.run();
}
