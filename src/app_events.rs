use termion::event::Key;

use crate::point::Point;

#[derive(Debug)]
pub enum AppEvent {
  #[allow(dead_code)]
  Render,
  #[allow(dead_code)]
  Update,
  MouseDown(Point),
  MouseUp(Point),
  Drag(Point),
  Key(Key),
}
