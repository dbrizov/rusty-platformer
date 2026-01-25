use crate::assets::Assets;
use crate::entity::{EntityId, EntityRef, EntitySpawner};
use crate::input::Input;
use crate::render::RenderQueue;
use crate::time::Time;
use sdl2::event::Event;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl};

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "Rusty Platform";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub struct Sdl2Context {
    _m_sdl2: Sdl,
    _m_sdl2_image: Sdl2ImageContext,

    m_canvas: Canvas<Window>,
    m_event_pump: EventPump,
    m_time: Time,
    m_input: Input,
}

impl Sdl2Context {
    pub fn new() -> Self {
        let sdl2 = sdl2::init().unwrap();
        let sdl2_image = sdl2::image::init(InitFlag::PNG).unwrap();

        let video = sdl2.video().unwrap();
        let window = video
            .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event_pump = sdl2.event_pump().unwrap();
        let time = Time::new(&sdl2, FPS).unwrap();
        let input = Input::new().unwrap();

        Self {
            _m_sdl2: sdl2,
            _m_sdl2_image: sdl2_image,
            m_canvas: canvas,
            m_event_pump: event_pump,
            m_time: time,
            m_input: input,
        }
    }

    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.m_canvas.texture_creator()
    }
}

pub struct App {
    m_entity_spawner: EntitySpawner,
    m_render_queue: RenderQueue,
}

impl App {
    pub fn new() -> Self {
        Self {
            m_entity_spawner: EntitySpawner::new(),
            m_render_queue: RenderQueue::new(),
        }
    }

    pub fn run(&mut self, sdl2: &mut Sdl2Context, assets: &mut Assets) {
        sdl2.m_input.on_input_event.push(Box::new(|event| {
            println!("Event: {:?}", event);
        }));

        let mut events: Vec<Event> = Vec::new();

        // Debug render
        sdl2.m_canvas.set_draw_color(Color::RGB(14, 219, 248));

        'running: loop {
            events.clear();
            for event in sdl2.m_event_pump.poll_iter() {
                events.push(event);
            }

            for event in &events {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            sdl2.m_time.tick();

            self.m_entity_spawner.resolve();

            let delta_time: f32 = sdl2.m_time.get_delta_time();
            let scaled_delta_time: f32 = delta_time * sdl2.m_time.get_time_scale();

            // tick()
            sdl2.m_input
                .tick(delta_time, &sdl2.m_event_pump.keyboard_state());

            for mut entity in self.m_entity_spawner.entity_iter_mut() {
                entity.tick(scaled_delta_time);
            }

            // physics_tick()
            // TODO

            // render_tick()
            sdl2.m_canvas.clear();

            for mut entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time, &mut self.m_render_queue);
            }

            for render_struct in self.m_render_queue.drain() {
                if let Some(texture) = assets.get_texture(render_struct.texture_id) {
                    let query = texture.query();
                    let destination = Rect::new(
                        render_struct.position.x as i32,
                        render_struct.position.y as i32,
                        query.width * (render_struct.scale.x as u32),
                        query.height * (render_struct.scale.y as u32),
                    );

                    if let Err(err) = sdl2.m_canvas.copy(texture, None, destination) {
                        eprintln!("Render error: {}", err);
                    }
                } else {
                    eprintln!(
                        "Texture with 'texture_id={}' not found",
                        render_struct.texture_id
                    );
                }
            }

            sdl2.m_canvas.present();
        }
    }

    pub fn spawn_entity(&mut self, entity: EntityRef) {
        self.m_entity_spawner.spawn(entity);
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        self.m_entity_spawner.destroy(entity_id);
    }
}
