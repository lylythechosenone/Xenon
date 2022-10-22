use std::collections::HashMap;
pub use winit::event::{MouseButton, VirtualKeyCode};

use crate::math::{Box2D, Point2D};

pub enum KeyState {
    Pressed,
    Down,
    Released,
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseState {
    Clicked,
    Down,
    Released,
    None,
}
impl Default for MouseState {
    fn default() -> Self {
        MouseState::None
    }
}

#[derive(Default)]
pub struct Input {
    down: HashMap<VirtualKeyCode, KeyState>,
    pressed_buffer: Vec<VirtualKeyCode>,
    released_buffer: Vec<VirtualKeyCode>,
    mouse_position: Point2D,
    left_mouse_state: MouseState,
    middle_mouse_state: MouseState,
    right_mouse_state: MouseState,
    mouse_buffer: Vec<(MouseState, MouseButton)>,
    bounds: Box2D,
}
impl Input {
    /// Same as `Self::default()`
    pub fn new(bounds: Box2D) -> Self {
        Self {
            bounds,
            ..Default::default()
        }
    }
    pub(crate) fn set_bounds(&mut self, bounds: Box2D) {
        self.bounds = bounds;
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
    /// Returns the click position if the mouse was pressed this update.
    pub fn is_mouse_clicked(&self, mouse_button: MouseButton) -> Option<Point2D> {
        match mouse_button {
            MouseButton::Left => match matches!(self.left_mouse_state, MouseState::Clicked) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Middle => match matches!(self.middle_mouse_state, MouseState::Clicked) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Right => match matches!(self.right_mouse_state, MouseState::Clicked) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Other(n) => todo!("Mouse button {} is not supported", n),
        }
    }
    /// Returns the click position if the mouse is held down
    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> Option<Point2D> {
        match mouse_button {
            MouseButton::Left => match matches!(self.left_mouse_state, MouseState::Down) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Middle => match matches!(self.middle_mouse_state, MouseState::Down) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Right => match matches!(self.right_mouse_state, MouseState::Down) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Other(n) => todo!("Mouse button {} is not supported", n),
        }
    }
    /// Returns the click position if the mouse was released this update
    pub fn is_mouse_released(&self, mouse_button: MouseButton) -> Option<Point2D> {
        match mouse_button {
            MouseButton::Left => match matches!(self.left_mouse_state, MouseState::Released) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Middle => match matches!(self.middle_mouse_state, MouseState::Released) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Right => match matches!(self.right_mouse_state, MouseState::Released) {
                true if self.bounds.contains(self.mouse_position) => {
                    Some((self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max))
                }
                _ => None,
            },
            MouseButton::Other(n) => todo!("Mouse button {} is not supported", n),
        }
    }
    /// Returns the mouse position
    pub fn mouse_position(&self) -> Point2D {
        (self.mouse_position + self.bounds.min.to_vector()).min(self.bounds.max)
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
        self.left_mouse_state = match self.left_mouse_state {
            MouseState::Clicked => MouseState::Down,
            MouseState::Released => MouseState::None,
            state => state,
        };
        self.middle_mouse_state = match self.middle_mouse_state {
            MouseState::Clicked => MouseState::Down,
            MouseState::Released => MouseState::None,
            state => state,
        };
        self.right_mouse_state = match self.right_mouse_state {
            MouseState::Clicked => MouseState::Down,
            MouseState::Released => MouseState::None,
            state => state,
        };
        self.left_mouse_state = match self
            .mouse_buffer
            .iter()
            .position(|(_, button)| matches!(button, MouseButton::Left))
        {
            Some(n) => self.mouse_buffer.remove(n).0,
            None => self.left_mouse_state,
        };
        self.middle_mouse_state = match self
            .mouse_buffer
            .iter()
            .position(|(_, button)| matches!(button, MouseButton::Middle))
        {
            Some(n) => self.mouse_buffer.remove(n).0,
            None => self.middle_mouse_state,
        };
        self.right_mouse_state = match self
            .mouse_buffer
            .iter()
            .position(|(_, button)| matches!(button, MouseButton::Right))
        {
            Some(n) => self.mouse_buffer.remove(n).0,
            None => self.right_mouse_state,
        };
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
    /// Process a mouse movement
    pub fn process_mouse_move(&mut self, pos: Point2D) {
        self.mouse_position = pos;
    }
    /// Process a mouse click
    pub fn process_mouse_click(&mut self, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => self
                .mouse_buffer
                .push((MouseState::Clicked, MouseButton::Left)),
            MouseButton::Middle => self
                .mouse_buffer
                .push((MouseState::Clicked, MouseButton::Middle)),
            MouseButton::Right => self
                .mouse_buffer
                .push((MouseState::Clicked, MouseButton::Right)),
            MouseButton::Other(n) => todo!("Mouse button {} is not supported", n),
        }
    }
    /// Process a mouse release
    pub fn process_mouse_release(&mut self, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => self
                .mouse_buffer
                .push((MouseState::Released, MouseButton::Left)),
            MouseButton::Middle => self
                .mouse_buffer
                .push((MouseState::Released, MouseButton::Middle)),
            MouseButton::Right => self
                .mouse_buffer
                .push((MouseState::Released, MouseButton::Right)),
            MouseButton::Other(n) => todo!("Mouse button {} is not supported", n),
        }
    }
    /// Returns `true` if the `Input` needs update
    pub fn needs_update(&self) -> bool {
        !(self.pressed_buffer.is_empty()
            && self.released_buffer.is_empty()
            && self.mouse_buffer.is_empty())
    }
}
