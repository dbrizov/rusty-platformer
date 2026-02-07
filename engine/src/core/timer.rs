use std::rc::Rc;

pub struct Timer {
    m_timer_subsystem: Rc<sdl2::TimerSubsystem>,
    m_target_fps: u32,
    m_vsync_enabled: bool,
    m_time_scale: f32,
    m_play_time: f32,
    m_delta_time: f32,
    m_frequency: u64,
    m_frame_start_ticks: u64,
    m_last_frame_start_ticks: u64,
}

impl Timer {
    pub fn new(
        timer_subsystem: Rc<sdl2::TimerSubsystem>,
        target_fps: u32,
        vsync_enabled: bool,
    ) -> Self {
        let frequency = timer_subsystem.performance_frequency();
        let frame_start_ticks = timer_subsystem.performance_counter();
        let last_frame_start_ticks = timer_subsystem.performance_counter();

        Timer {
            m_timer_subsystem: timer_subsystem,
            m_target_fps: target_fps,
            m_vsync_enabled: vsync_enabled,
            m_time_scale: 1.0,
            m_play_time: 0.0,
            m_delta_time: 0.0,
            m_frequency: frequency,
            m_frame_start_ticks: frame_start_ticks,
            m_last_frame_start_ticks: last_frame_start_ticks,
        }
    }

    pub fn frame_start(&mut self) {
        let now_ticks = self.m_timer_subsystem.performance_counter();
        let diff_ticks = now_ticks - self.m_last_frame_start_ticks;

        self.m_frame_start_ticks = now_ticks; // Remember when this frame started (for frame_end)
        self.m_last_frame_start_ticks = now_ticks;

        let mut dt_seconds = (diff_ticks as f64) / (self.m_frequency as f64);

        // After stalls (debugger, window focus loss, OS scheduling),
        // dt can become very large. Clamp it to avoid excessive physics
        // catch-up and the "spiral of death".
        if dt_seconds > 0.25 {
            dt_seconds = 0.25;
        }

        self.m_delta_time = dt_seconds as f32;
        self.m_play_time += self.m_delta_time;
    }

    pub fn frame_end(&mut self) {
        if self.m_vsync_enabled || self.m_target_fps == 0 {
            return;
        }

        let target_frame_seconds = 1.0 / (self.m_target_fps as f64);

        loop {
            let now_ticks = self.m_timer_subsystem.performance_counter();
            let elapsed_seconds =
                ((now_ticks - self.m_frame_start_ticks) as f64) / (self.m_frequency as f64);

            if elapsed_seconds >= target_frame_seconds {
                break;
            }

            // Sleep most of the remaining time (avoid oversleep, because SDL_Delay tends to oversleep)
            let remaining_seconds = target_frame_seconds - elapsed_seconds;
            if remaining_seconds > 0.002 {
                // If there are more the 2ms remaining:
                // - We subtract 1ms from the remaining time to prevent oversleep.
                // - 1ms is usually larger than the OS wake-up jitter.
                // - It gives us a safety margin so we don’t overshoot.
                let sleep_ms = ((remaining_seconds - 0.001) * 1000.0) as u32;
                self.m_timer_subsystem.delay(sleep_ms);
            }
        }
    }

    pub fn get_fps(&self) -> u32 {
        self.m_target_fps
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.m_target_fps = fps;
    }

    pub fn get_time_scale(&self) -> f32 {
        self.m_time_scale
    }

    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.m_time_scale = time_scale;
    }

    pub fn get_play_time(&self) -> f32 {
        self.m_play_time
    }

    pub fn get_delta_time(&self) -> f32 {
        self.m_delta_time
    }
}
