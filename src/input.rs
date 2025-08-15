use std::collections::HashSet;

pub use crate::input_wrapper::Key;

pub struct RendiumInput {
    prev_keys: HashSet<Key>,
    curr_keys: HashSet<Key>,
}

impl RendiumInput {
    fn update(&mut self) {
        std::mem::replace(&mut self.prev_keys, self.curr_keys);
    }
}
