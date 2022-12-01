use std::{collections::HashSet, sync::mpsc::Receiver};

use termion::event::{Event, Key, MouseButton, MouseEvent};

use crate::{
  brush::BrushState,
  canvas::CanvasState,
  line_processor::plot_line,
  point::Point,
  term::{self, TEvent},
  tool::ToolState,
};

pub struct MouseEventTracker {
  pub left_pressed: Option<Point>,
  pub left_released: Option<Point>,
  pub left_held: Option<Point>,
  pub left_hover: Vec<Point>,
  pub left_last_known: Option<Point>,
}

impl MouseEventTracker {
  pub fn new() -> Self {
    MouseEventTracker {
      left_pressed: None,
      left_released: None,
      left_held: None,
      left_hover: vec![],
      left_last_known: None,
    }
  }

  pub fn handle_mouse_event(&mut self, event: &TEvent) {
    match event {
      TEvent::MouseDown(pos) => {
        self.left_pressed = Some(*pos);
        self.left_last_known = Some(*pos);
      }
      TEvent::MouseUp(pos) => {
        self.left_released = Some(*pos);
        self.left_last_known = None;
      }
      TEvent::Drag(pos) => {
        let p = *pos;
        if let Some(v) = self.left_last_known {
          self.left_held = Some(p);
          self.left_hover = plot_line(p, v);
        } else {
          self.left_hover = vec![p];
        }
        self.left_last_known = Some(p);
      }
      _ => (),
    }
  }

  pub fn reset(&mut self) {
    self.left_pressed = None;
    self.left_released = None;
    self.left_held = None;
    self.left_hover = vec![];
  }
}

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
    self.pressed_keys.contains(&key)
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
    State {
      brush: BrushState::new(),
      tools: ToolState::new(),
      brush_menu_width: 20,
      pressed_keys: HashSet::new(),
      mouse_events: MouseEventTracker::new(),
      terminal_size,
      canvas_state: CanvasState::new(terminal_size),
    }
  }
}
