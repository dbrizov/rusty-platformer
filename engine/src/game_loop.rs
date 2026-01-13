use crate::input;
use crate::time;

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "SLD2 Window";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn run() {
    let sdl: sdl2::Sdl = sdl2::init().unwrap();
    let mut time: time::Time = time::Time::new(&sdl, FPS).unwrap();
    let mut input: input::Input = input::Input::new().unwrap();
    let video: sdl2::VideoSubsystem = sdl.video().unwrap();
    let _window: sdl2::video::Window = video
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut events: Vec<sdl2::event::Event> = Vec::new();
    let mut event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

    'running: loop {
        events.clear();
        for event in event_pump.poll_iter() {
            events.push(event);
        }

        for event in &events {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        time.tick();

        let delta_time: f32 = time.get_delta_time();

        input.tick(delta_time, &event_pump.keyboard_state());
    }
}
