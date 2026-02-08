use crate::entity::Entity;

pub struct Physics {
    m_fixed_delta_time: f32,
    m_accumulator: f32,
    m_interpolation_fraction: f32,
    m_use_interpolation: bool,
}

impl Physics {
    pub fn new(ticks_per_second: u32, use_interpolation: bool) -> Self {
        Self {
            m_fixed_delta_time: 1.0 / (ticks_per_second as f32),
            m_accumulator: 0.0,
            m_interpolation_fraction: 0.0,
            m_use_interpolation: use_interpolation,
        }
    }

    pub fn get_fixed_delta_time(&self) -> f32 {
        self.m_fixed_delta_time
    }

    pub fn get_interpolation_fraction(&self) -> f32 {
        self.m_interpolation_fraction
    }

    pub fn tick_entities<'a, F, I>(&mut self, frame_delta_time: f32, mut entities: F)
    where
        F: FnMut() -> I,
        I: Iterator<Item = &'a mut Entity>,
    {
        self.m_accumulator += frame_delta_time;

        while self.m_accumulator >= self.m_fixed_delta_time {
            for entity in entities() {
                entity.physics_tick(self.m_fixed_delta_time);
            }

            self.m_accumulator -= self.m_fixed_delta_time;
        }

        self.m_interpolation_fraction = if self.m_use_interpolation {
            self.m_accumulator / self.m_fixed_delta_time
        } else {
            0.0
        }
    }
}
