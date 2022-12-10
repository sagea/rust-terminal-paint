use std::collections::HashSet;

use termion::event::Key;

use crate::{app_events::AppEvent, singleton};

singleton!(pub static KEY_STATE: KeyState = KeyState::new());

#[derive(Default)]
pub struct KeyState {
  pressed_keys: HashSet<Key>,
}

impl KeyState {
  pub fn new() -> Self {
    KeyState {
      ..Default::default()
    }
  }
  pub fn reset(&mut self) {
    self.pressed_keys.clear();
  }
  pub fn handle_terminal_event(&mut self, event: &AppEvent) {
    if let AppEvent::Key(key) = &event {
      self.pressed_keys.insert(*key);
    }
  }
  pub fn was_key_pressed(&self, key: &Key) -> bool {
    self.pressed_keys.contains(key)
  }
}
