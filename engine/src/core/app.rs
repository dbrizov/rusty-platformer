use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::FRect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl};

use crate::core::assets::Assets;
use crate::core::input::Input;
use crate::core::render::RenderQueue;
use crate::core::timer::Timer;
use crate::entity::{Entity, EntityId, EntitySpawner};

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "Rusty Platformer";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub struct Sdl2Context {
    _m_sdl2: Sdl,
    _m_sdl2_image: Sdl2ImageContext,

    m_canvas: Canvas<Window>,
    m_texture_creator: Rc<TextureCreator<WindowContext>>,
    m_event_pump: EventPump,
    m_timer: Timer,
    m_input: Rc<RefCell<Input>>,
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
        let texture_creator = Rc::new(canvas.texture_creator());
        let event_pump = sdl2.event_pump().unwrap();
        let timer = Timer::new(&sdl2, FPS).unwrap();
        let input = Rc::new(RefCell::new(Input::new().unwrap()));

        Self {
            _m_sdl2: sdl2,
            _m_sdl2_image: sdl2_image,
            m_canvas: canvas,
            m_texture_creator: texture_creator,
            m_event_pump: event_pump,
            m_timer: timer,
            m_input: input,
        }
    }

    pub fn get_texture_creator(&self) -> Rc<TextureCreator<WindowContext>> {
        self.m_texture_creator.clone()
    }

    pub fn get_input(&self) -> Rc<RefCell<Input>> {
        self.m_input.clone()
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

            sdl2.m_timer.tick();

            self.m_entity_spawner.resolve_requests();

            let delta_time = sdl2.m_timer.get_delta_time();
            let scaled_delta_time = delta_time * sdl2.m_timer.get_time_scale();

            // tick()
            sdl2.m_input
                .borrow_mut()
                .tick(delta_time, &sdl2.m_event_pump.keyboard_state());

            for entity in self.m_entity_spawner.entity_iter_mut() {
                if entity.is_ticking() {
                    entity.tick(scaled_delta_time);
                }
            }

            // physics_tick()
            // TODO

            // render_tick()
            sdl2.m_canvas.clear();

            for entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time, &mut self.m_render_queue);
            }

            for render_data in self.m_render_queue.drain() {
                if let Some(texture) = assets.get_texture(render_data.texture_id) {
                    let query = texture.query();
                    let destination = FRect::new(
                        render_data.position.x,
                        render_data.position.y,
                        (query.width as f32) * render_data.scale.x,
                        (query.height as f32) * render_data.scale.y,
                    );

                    if let Err(err) = sdl2.m_canvas.copy_f(texture, None, destination) {
                        eprintln!("Render error: {}", err);
                    }
                } else {
                    eprintln!(
                        "Texture with 'texture_id={}' not found",
                        render_data.texture_id
                    );
                }
            }

            sdl2.m_canvas.present();
        }
    }

    pub fn spawn_entity(&mut self, entity: Box<Entity>) {
        self.m_entity_spawner.spawn(entity);
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        self.m_entity_spawner.destroy(entity_id);
    }
}
