pub struct Time {
    m_timer: sdl2::TimerSubsystem,
    m_fps: u32,
    m_time_scale: f32,
    m_play_time: f32,
    m_delta_time: f32,
    m_last_ticks: u32
}

impl Time {
    pub fn new(sdl: &sdl2::Sdl, fps: u32) -> Time {
        Time {
            m_timer: sdl.timer().unwrap(),
            m_fps: fps,
            m_time_scale: 1.0,
            m_play_time: 0.0,
            m_delta_time: 0.0,
            m_last_ticks: 0
        }
    }

    pub fn tick(&mut self) {
        let current_ticks: u32 = self.m_timer.ticks();
        let mut delta_ms: u32 = current_ticks - self.m_last_ticks;        
        let target_ms: u32 = 1000 / self.m_fps;

        if delta_ms < target_ms {
            self.m_timer.delay(target_ms - delta_ms);

            let after_delay_ticks = self.m_timer.ticks();
            delta_ms = after_delay_ticks - self.m_last_ticks;
        }
        
        self.m_last_ticks = self.m_timer.ticks();
        self.m_delta_time = delta_ms as f32 / 1000.0;
        self.m_play_time += self.m_delta_time;
    }

    pub fn get_fps(&self) -> u32 {
        self.m_fps
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.m_fps = fps;
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
