use crate::time::Time;

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "SLD2 Window";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn run() {
    let sdl: sdl2::Sdl = sdl2::init().unwrap();
    let mut time: Time = Time::new(&sdl, FPS);
    let video: sdl2::VideoSubsystem = sdl.video().unwrap();
    let _window: sdl2::video::Window = video
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

    let mut running: bool = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    running = false;
                    break;
                }
                _ => {}
            }
        }

        time.tick();
    }
}
