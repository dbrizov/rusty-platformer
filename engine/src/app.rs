use sdl2::event::Event;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::path::Path;

use crate::assets::AssetDatabase;
use crate::entity::{EntityId, EntityRef, EntitySpawner};
use crate::input::Input;
use crate::time::Time;

const FPS: u32 = 60;
const WINDOW_TITLE: &str = "Rusty Platform";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[allow(dead_code)]
pub struct App {
    m_sdl: Sdl,
    m_video: VideoSubsystem,
    m_canvas: Canvas<Window>,
    m_texture_creator: TextureCreator<WindowContext>,
    m_image_context: Sdl2ImageContext,
    m_event_pump: EventPump,
    m_asset_db: AssetDatabase,
    m_time: Time,
    m_input: Input,
    m_entity_spawner: EntitySpawner,
}

impl App {
    pub fn init() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let image_context: Sdl2ImageContext = sdl2::image::init(InitFlag::PNG).unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let asset_db = AssetDatabase::new();
        let time = Time::new(&sdl, FPS).unwrap();
        let input = Input::new().unwrap();
        let entity_spawner = EntitySpawner::new();

        Self {
            m_sdl: sdl,
            m_video: video,
            m_canvas: canvas,
            m_texture_creator: texture_creator,
            m_image_context: image_context,
            m_event_pump: event_pump,
            m_asset_db: asset_db,
            m_time: time,
            m_input: input,
            m_entity_spawner: entity_spawner,
        }
    }

    pub fn set_assets_root<P>(&mut self, assets_root: P)
    where
        P: AsRef<Path>,
    {
        self.m_asset_db.set_assets_root(assets_root);
    }

    pub fn run(&mut self) {
        self.m_input.on_input_event.push(Box::new(|event| {
            println!("Event: {:?}", event);
        }));

        let mut events: Vec<Event> = Vec::new();

        // Debug render
        self.m_canvas.set_draw_color(Color::RGB(14, 219, 248));
        let image_path = self
            .m_asset_db
            .asset_path(&["images", "entities", "player", "idle", "00.png"]);
        let image = self
            .m_asset_db
            .load_texture(&self.m_texture_creator, image_path)
            .unwrap();

        'running: loop {
            events.clear();
            for event in self.m_event_pump.poll_iter() {
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

            self.m_time.tick();

            self.m_entity_spawner.resolve();

            let delta_time: f32 = self.m_time.get_delta_time();
            let scaled_delta_time: f32 = delta_time * self.m_time.get_time_scale();

            // tick()
            self.m_input
                .tick(delta_time, &self.m_event_pump.keyboard_state());

            for mut entity in self.m_entity_spawner.entity_iter_mut() {
                entity.tick(scaled_delta_time);
            }

            // physics_tick()

            // render_tick()
            self.m_canvas.clear();

            let query = image.query();
            let dst = Rect::new(50, 50, query.width * 2, query.height * 2);
            if let Err(err) = self.m_canvas.copy(&image, None, dst) {
                eprintln!("Render error: {}", err);
            }

            for mut entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time);
            }

            self.m_canvas.present();
        }
    }

    pub fn spawn_entity(&mut self, entity: EntityRef) {
        self.m_entity_spawner.spawn(entity);
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        self.m_entity_spawner.destroy(entity_id);
    }
}
