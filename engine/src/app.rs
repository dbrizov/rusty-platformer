use sdl2::event::Event;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl};
use std::path::Path;

use crate::assets::Assets;
use crate::entity::{EntityId, EntityRef, EntitySpawner};
use crate::input::Input;
use crate::time::Time;

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "Rusty Platform";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub struct Sdl2Instance {
    _m_sdl2: Sdl,
    _m_sdl2_image: Sdl2ImageContext,

    m_canvas: Canvas<Window>,
    m_texture_creator: TextureCreator<WindowContext>,
    m_event_pump: EventPump,
    m_time: Time,
    m_input: Input,
}

impl Sdl2Instance {
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
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl2.event_pump().unwrap();
        let time = Time::new(&sdl2, FPS).unwrap();
        let input = Input::new().unwrap();

        Self {
            _m_sdl2: sdl2,
            _m_sdl2_image: sdl2_image,
            m_canvas: canvas,
            m_texture_creator: texture_creator,
            m_event_pump: event_pump,
            m_time: time,
            m_input: input,
        }
    }
}

pub struct App<'a> {
    m_assets: Assets<'a>,
    m_entity_spawner: EntitySpawner,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            m_assets: Assets::new(),
            m_entity_spawner: EntitySpawner::new(),
        }
    }

    pub fn set_assets_root<P>(&mut self, assets_root: P)
    where
        P: AsRef<Path>,
    {
        self.m_assets.set_assets_root(assets_root);
    }

    pub fn run(&mut self, sdl2: &'a mut Sdl2Instance) {
        sdl2.m_input.on_input_event.push(Box::new(|event| {
            println!("Event: {:?}", event);
        }));

        let mut events: Vec<Event> = Vec::new();

        // Debug render
        sdl2.m_canvas.set_draw_color(Color::RGB(14, 219, 248));
        let image_path = self
            .m_assets
            .asset_path(&["images", "entities", "player", "idle", "00.png"]);
        let image_id = self
            .m_assets
            .load_texture(&sdl2.m_texture_creator, image_path)
            .unwrap();
        let image_texture = self.m_assets.get_texture(image_id).unwrap();

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

            // render_tick()
            sdl2.m_canvas.clear();

            let query = image_texture.query();
            let dst = Rect::new(50, 50, query.width * 2, query.height * 2);
            if let Err(err) = sdl2.m_canvas.copy(&image_texture, None, dst) {
                eprintln!("Render error: {}", err);
            }

            for mut entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time);
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
