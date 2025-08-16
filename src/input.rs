use std::collections::HashSet;

use crate::RendiumInstance;
pub use crate::input_wrapper::{Key, MouseButton};

pub struct RendiumInput {
    prev_keys: HashSet<Key>,
    curr_keys: HashSet<Key>,

    key_char: Option<String>,

    prev_mouse_button: HashSet<MouseButton>,
    curr_mouse_button: HashSet<MouseButton>,
}

impl RendiumInput {
    pub fn new() -> Self {
        Self {
            prev_keys: HashSet::new(),
            curr_keys: HashSet::new(),
            key_char: None,
            prev_mouse_button: HashSet::new(),
            curr_mouse_button: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        let _ = std::mem::replace(&mut self.prev_keys, self.curr_keys.clone());
        let _ = std::mem::replace(&mut self.prev_mouse_button, self.curr_mouse_button.clone());
    }

    pub fn add_key(&mut self, key: Key) {
        self.curr_keys.insert(key);
    }

    pub fn remove_key(&mut self, key: Key) {
        self.curr_keys.remove(&key);
    }

    pub fn add_mouse_button(&mut self, mb: MouseButton) {
        self.curr_mouse_button.insert(mb);
    }

    pub fn remove_mouse_button(&mut self, mb: MouseButton) {
        self.curr_mouse_button.remove(&mb);
    }

    pub fn set_key_char(&mut self, key_char: Option<winit::keyboard::SmolStr>) {
        match key_char {
            Some(ch) => self.key_char = Some(ch.to_string()),
            None => self.key_char = None,
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.curr_keys.contains(&key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        !self.curr_keys.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.curr_keys.contains(&key) && !self.prev_keys.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        !self.curr_keys.contains(&key) && self.prev_keys.contains(&key)
    }

    pub fn get_key_char(&self) -> Option<String> {
        self.key_char.clone()
    }

    pub fn is_mouse_button_down(&self, mb: MouseButton) -> bool {
        self.curr_mouse_button.contains(&mb)
    }

    pub fn is_mouse_button_up(&self, mb: MouseButton) -> bool {
        !self.curr_mouse_button.contains(&mb)
    }

    pub fn is_mouse_button_pressed(&self, mb: MouseButton) -> bool {
        self.curr_mouse_button.contains(&mb) && !self.prev_mouse_button.contains(&mb)
    }

    pub fn is_mouse_button_released(&self, mb: MouseButton) -> bool {
        !self.curr_mouse_button.contains(&mb) && self.prev_mouse_button.contains(&mb)
    }
}

// The user can't actually use these "is_key_*" func
// since input is private in RendiumInstance
pub trait Input {
    fn is_key_down(&self, key: Key) -> bool;
    fn is_key_up(&self, key: Key) -> bool;
    fn is_key_pressed(&self, key: Key) -> bool;
    fn is_key_released(&self, key: Key) -> bool;
    fn get_key_char(&self) -> Option<String>;
    fn is_mouse_button_down(&self, mb: MouseButton) -> bool;
    fn is_mouse_button_up(&self, mb: MouseButton) -> bool;
    fn is_mouse_button_pressed(&self, mb: MouseButton) -> bool;
    fn is_mouse_button_released(&self, mb: MouseButton) -> bool;
}

impl Input for RendiumInstance {
    fn is_key_down(&self, key: Key) -> bool {
        self.input.is_key_down(key)
    }

    fn is_key_up(&self, key: Key) -> bool {
        self.input.is_key_up(key)
    }

    fn is_key_pressed(&self, key: Key) -> bool {
        self.input.is_key_pressed(key)
    }

    fn is_key_released(&self, key: Key) -> bool {
        self.input.is_key_released(key)
    }

    fn get_key_char(&self) -> Option<String> {
        self.input.get_key_char()
    }

    fn is_mouse_button_down(&self, mb: MouseButton) -> bool {
        self.input.is_mouse_button_down(mb)
    }

    fn is_mouse_button_up(&self, mb: MouseButton) -> bool {
        self.input.is_mouse_button_up(mb)
    }

    fn is_mouse_button_pressed(&self, mb: MouseButton) -> bool {
        self.input.is_mouse_button_pressed(mb)
    }

    fn is_mouse_button_released(&self, mb: MouseButton) -> bool {
        self.input.is_mouse_button_released(mb)
    }
}
