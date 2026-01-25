use std::collections::{VecDeque, vec_deque::Drain};

use crate::{assets::TextureId, math::Vec2};

pub struct RenderStruct {
    pub texture_id: TextureId,
    pub position: Vec2,
    pub prev_position: Vec2, // needed for physics interpolation
    pub scale: Vec2,
}

impl RenderStruct {
    pub fn new(texture_id: TextureId, position: Vec2, prev_position: Vec2, scale: Vec2) -> Self {
        Self {
            texture_id: texture_id,
            position: position,
            prev_position: prev_position,
            scale: scale,
        }
    }
}

pub struct RenderQueue {
    m_deque: VecDeque<RenderStruct>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            m_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, render_struct: RenderStruct) {
        self.m_deque.push_back(render_struct);
    }

    pub fn dequeue(&mut self) -> Option<RenderStruct> {
        self.m_deque.pop_front()
    }

    pub fn drain(&mut self) -> Drain<'_, RenderStruct> {
        self.m_deque.drain(..)
    }
}
