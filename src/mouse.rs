use crate::{app_events::AppEvent, line_processor::plot_line, point::Point, singleton};

singleton!(pub static MOUSE_EVENTS: MouseEventTracker = MouseEventTracker::new());

pub struct MouseEventTracker {
  pub left_pressed: Option<Point>,
  pub left_released: Option<Point>,
  pub left_drag: Option<Point>,
  pub left_held: Option<Point>,
  pub left_hover: Vec<Point>,
  pub left_last_known: Option<Point>,
}

impl MouseEventTracker {
  pub fn new() -> Self {
    MouseEventTracker {
      left_pressed: None,
      left_released: None,
      left_drag: None,
      left_held: None,
      left_hover: vec![],
      left_last_known: None,
    }
  }
  pub fn handle_terminal_event(&mut self, event: &AppEvent) {
    match event {
      AppEvent::MouseDown(pos) => {
        self.left_pressed = Some(*pos);
        self.left_last_known = Some(*pos);
      }
      AppEvent::MouseUp(pos) => {
        self.left_released = Some(*pos);
        self.left_last_known = None;
      }
      AppEvent::Drag(pos) => {
        let p = *pos;
        self.left_drag = Some(p);
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
    self.left_drag = None;
    self.left_hover = vec![];
  }
}
