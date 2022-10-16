use std::collections::HashMap;
use winit::event::VirtualKeyCode;

pub enum KeyState {
    Pressed,
    Down,
    Released,
    None,
}

#[derive(Default)]
pub struct Input {
    down: HashMap<VirtualKeyCode, KeyState>,
    pressed_buffer: Vec<VirtualKeyCode>,
    released_buffer: Vec<VirtualKeyCode>,
}
impl Input {
    /// Same as `Self::default()`
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns `true` if the key is currently being pressed.
    pub fn is_down(&self, key: VirtualKeyCode) -> bool {
        matches!(
            self.down.get(&key),
            Some(KeyState::Pressed) | Some(KeyState::Down)
        )
    }
    /// Returns `true` if the key was first pressed this update.
    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        matches!(self.down.get(&key), Some(KeyState::Pressed))
    }
    /// Returns `true` if the key was released this update.
    pub fn is_released(&self, key: VirtualKeyCode) -> bool {
        matches!(self.down.get(&key), Some(KeyState::Released))
    }
    /// Update the input state.
    /// This should be called once per frame.
    /// Pressed goes to down, released goes to none.
    /// Pressed and released buffers are flushed.
    pub fn update(&mut self) {
        for v in &mut self.down.values_mut() {
            match v {
                KeyState::Pressed => *v = KeyState::Down,
                KeyState::Released => *v = KeyState::None,
                _ => {}
            }
        }
        for key in &self.pressed_buffer {
            self.down.insert(*key, KeyState::Pressed);
        }
        for key in &self.released_buffer {
            self.down.insert(*key, KeyState::Released);
        }
        self.pressed_buffer.clear();
        self.released_buffer.clear();
    }
    /// Add a key to the `pressed` buffer.
    /// Note: this doesn't actually handle the event, it just buffers it for a later `update`.
    pub fn process_pressed(&mut self, key: VirtualKeyCode) {
        self.pressed_buffer.push(key);
    }
    /// Add a key to the `released` buffer.
    /// Note: this doesn't actually handle the event, it just buffers it for a later `update`.
    pub fn process_released(&mut self, key: VirtualKeyCode) {
        self.released_buffer.push(key);
    }
}
