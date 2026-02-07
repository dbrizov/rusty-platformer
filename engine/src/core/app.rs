use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::FRect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl, TimerSubsystem};

use crate::core::assets::Assets;
use crate::core::input::Input;
use crate::core::render::RenderQueue;
use crate::core::timer::Timer;
use crate::entity::EntitySpawner;

pub struct Sdl2Context {
    _m_sdl2: Sdl,
    _m_sdl2_image: Sdl2ImageContext,

    m_canvas: Canvas<Window>,
    m_texture_creator: Rc<TextureCreator<WindowContext>>,
    m_timer_subsystem: Rc<TimerSubsystem>,
    m_event_pump: EventPump,
}

impl Sdl2Context {
    pub fn new(
        vsync_enabled: bool,
        window_title: &str,
        window_width: u32,
        window_height: u32,
    ) -> Self {
        let sdl2 = sdl2::init().unwrap();
        let sdl2_image = sdl2::image::init(InitFlag::PNG).unwrap();

        let video = sdl2.video().unwrap();
        let window = video
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas;
        let canvas_builder = window.into_canvas().accelerated();
        if vsync_enabled {
            canvas = canvas_builder.present_vsync().build().unwrap();
        } else {
            canvas = canvas_builder.build().unwrap();
        }

        let texture_creator = Rc::new(canvas.texture_creator());
        let timer_subsystem = Rc::new(sdl2.timer().unwrap());
        let event_pump = sdl2.event_pump().unwrap();

        Self {
            _m_sdl2: sdl2,
            _m_sdl2_image: sdl2_image,
            m_canvas: canvas,
            m_texture_creator: texture_creator,
            m_timer_subsystem: timer_subsystem,
            m_event_pump: event_pump,
        }
    }
}

pub struct App {
    // SDL context must be declared first so it is destroyed last.
    // Objects below depend on SDL resources.
    m_sdl2_context: Sdl2Context,
    m_timer: Timer,
    m_input: Rc<RefCell<Input>>,
    m_assets: Assets,
    m_render_queue: RenderQueue,
    m_entity_spawner: EntitySpawner,
}

impl App {
    pub fn new(
        target_fps: u32,
        vsync_enabled: bool,
        window_title: &str,
        window_width: u32,
        window_height: u32,
    ) -> Self {
        let sdl2_context =
            Sdl2Context::new(vsync_enabled, window_title, window_width, window_height);

        let timer = Timer::new(
            sdl2_context.m_timer_subsystem.clone(),
            target_fps,
            vsync_enabled,
        );

        let input = Rc::new(RefCell::new(Input::new().unwrap()));
        let assets = Assets::new(sdl2_context.m_texture_creator.clone());
        let render_queue = RenderQueue::new();
        let entity_spawner = EntitySpawner::new();

        Self {
            m_sdl2_context: sdl2_context,
            m_timer: timer,
            m_input: input,
            m_assets: assets,
            m_render_queue: render_queue,
            m_entity_spawner: entity_spawner,
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            self.m_timer.frame_start();

            for event in self.m_sdl2_context.m_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            self.m_entity_spawner.resolve_requests();

            let delta_time = self.m_timer.get_delta_time();
            let scaled_delta_time = delta_time * self.m_timer.get_time_scale();

            // input.tick()
            self.m_input.borrow_mut().tick(
                delta_time,
                &self.m_sdl2_context.m_event_pump.keyboard_state(),
            );

            // entities.tick()
            for entity in self.m_entity_spawner.entity_iter_mut() {
                if entity.is_ticking() {
                    entity.tick(scaled_delta_time);
                }
            }

            // entities.physics_tick()
            // TODO

            // entities.render_tick()
            for entity in self.m_entity_spawner.entity_iter_mut() {
                entity.render_tick(scaled_delta_time, &mut self.m_render_queue);
            }

            self.render_frame();

            self.m_timer.frame_end();
        }
    }

    pub fn get_timer(&mut self) -> &mut Timer {
        &mut self.m_timer
    }

    pub fn get_input(&mut self) -> Rc<RefCell<Input>> {
        self.m_input.clone()
    }

    pub fn get_assets(&mut self) -> &mut Assets {
        &mut self.m_assets
    }

    pub fn get_entity_spawner(&mut self) -> &mut EntitySpawner {
        &mut self.m_entity_spawner
    }

    fn render_frame(&mut self) {
        self.m_sdl2_context
            .m_canvas
            .set_draw_color(Color::RGB(14, 219, 248));

        self.m_sdl2_context.m_canvas.clear();

        for render_data in self.m_render_queue.drain() {
            if let Some(texture) = self.m_assets.get_texture(render_data.texture_id) {
                let query = texture.query();
                let destination = FRect::new(
                    render_data.position.x,
                    render_data.position.y,
                    (query.width as f32) * render_data.scale.x,
                    (query.height as f32) * render_data.scale.y,
                );

                if let Err(err) = self
                    .m_sdl2_context
                    .m_canvas
                    .copy_f(texture, None, destination)
                {
                    eprintln!("Render error: {}", err);
                }
            } else {
                eprintln!(
                    "Texture with 'texture_id={}' not found",
                    render_data.texture_id
                );
            }
        }

        self.m_sdl2_context.m_canvas.present();
    }
}
