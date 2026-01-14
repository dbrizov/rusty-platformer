use crate::entity;
use crate::input;
use crate::time;

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "SLD2 Window";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[allow(dead_code)]
pub struct Game {
    m_sdl: sdl2::Sdl,
    m_video: sdl2::VideoSubsystem,
    m_window: sdl2::video::Window,
    m_event_pump: sdl2::EventPump,
    m_time: time::Time,
    m_input: input::Input,
    m_entity_spawner: entity::EntitySpawner,
}

impl Game {
    pub fn init() -> Self {
        let sdl: sdl2::Sdl = sdl2::init().unwrap();
        let video: sdl2::VideoSubsystem = sdl.video().unwrap();
        let window: sdl2::video::Window = video
            .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();
        let time: time::Time = time::Time::new(&sdl, FPS).unwrap();
        let input: input::Input = input::Input::new().unwrap();
        let entity_spawner: entity::EntitySpawner = entity::EntitySpawner::new();

        Self {
            m_sdl: sdl,
            m_video: video,
            m_window: window,
            m_event_pump: event_pump,
            m_time: time,
            m_input: input,
            m_entity_spawner: entity_spawner,
        }
    }

    pub fn run(&mut self) {
        self.m_input.on_input_event.push(Box::new(|event| {
            println!("Event: {:?}", event);
        }));

        let mut events: Vec<sdl2::event::Event> = Vec::new();

        'running: loop {
            events.clear();
            for event in self.m_event_pump.poll_iter() {
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

            self.m_time.tick();

            self.m_entity_spawner.resolve();

            let delta_time: f32 = self.m_time.get_delta_time();
            let scaled_delta_time: f32 = delta_time * self.m_time.get_time_scale();

            // tick()
            self.m_input
                .tick(delta_time, &self.m_event_pump.keyboard_state());

            for entity in self.m_entity_spawner.entity_iter_mut() {
                entity.tick(scaled_delta_time);
            }

            // physics_tick()

            // render_tick()
            for entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time);
            }
        }
    }

    pub fn spawn_entity(&mut self, entity: Box<entity::Entity>) {
        self.m_entity_spawner.spawn(entity);
    }

    pub fn destroy_entity(&mut self, entity_id: entity::EntityId) {
        self.m_entity_spawner.destroy(entity_id);
    }
}
