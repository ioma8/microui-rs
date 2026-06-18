use crate::{vec2, Context};

impl Context {
    pub fn input_mousemove(&mut self, x: i32, y: i32) {
        self.mouse_pos = vec2(x, y);
    }
}
