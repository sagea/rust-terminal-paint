use std::{collections::HashSet, sync::mpsc::Receiver};

use termion::event::Key;

use crate::{
  bounds::Bounds,
  brush::BrushState,
  canvas::CanvasState,
  mouse::MouseEventTracker,
  point::Point,
  term::{self, TEvent},
  tool::ToolState,
};

pub struct State {
  pub brush: BrushState,
  pub tools: ToolState,
  pub brush_menu_width: u16,
  pub pressed_keys: HashSet<Key>,
  pub mouse_events: MouseEventTracker,
  pub terminal_size: Point,
  pub canvas_state: CanvasState,
}

impl State {
  pub fn new() -> Self {
    State {
      ..Default::default()
    }
  }
  pub fn was_key_pressed(&self, key: &Key) -> bool {
    self.pressed_keys.contains(key)
  }
  pub fn track_terminal_events(&mut self, recv: &Receiver<TEvent>) {
    loop {
      self.terminal_size = term::size();
      if let Ok(result) = recv.try_recv() {
        match &result {
          TEvent::Key(key) => {
            self.pressed_keys.insert(*key);
          }
          e => self.mouse_events.handle_mouse_event(e),
        }
      } else {
        break;
      }
    }
  }
  pub fn reset_terminal_events(&mut self) {
    self.mouse_events.reset();
    self.pressed_keys.clear();
  }
}

impl Default for State {
  fn default() -> Self {
    let terminal_size = term::size();
    let brush_menu_width = 20;
    let canvas_offset = Point::new(brush_menu_width, 0);

    State {
      brush: BrushState::new(),
      tools: ToolState::new(),
      brush_menu_width: 20,
      pressed_keys: HashSet::new(),
      mouse_events: MouseEventTracker::new(),
      terminal_size,
      canvas_state: CanvasState::new(Bounds {
        size: terminal_size - canvas_offset,
        offset: canvas_offset,
      }),
    }
  }
}
