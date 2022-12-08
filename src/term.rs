use std::io::Stdout;
use std::{io::stdout, sync::mpsc};
use termion::event::{Key, MouseEvent};
use termion::raw::IntoRawMode;
use termion::{color, cursor, terminal_size};
use termion::{event::Event, input::MouseTerminal};

use crate::point::Point;
use crate::pt;

#[derive(Debug)]
pub enum TMouseEvent {}
#[derive(Debug)]
pub enum TEvent {
  MouseDown(Point),
  MouseUp(Point),
  Drag(Point),
  Key(Key),
}

pub fn setup_stdin() -> mpsc::Receiver<TEvent> {
  use std::io::stdin;
  use termion::input::TermRead;
  let (sender, receiver) = mpsc::channel();
  let stdin = stdin();
  std::thread::spawn(move || {
    for c in stdin.events() {
      let event = c.unwrap();
      let e = match &event {
        Event::Mouse(mouse_event) => Some(match &mouse_event {
          MouseEvent::Press(_mouse_btn, x, y) => TEvent::MouseDown(pt!(x - 1, y - 1)),
          MouseEvent::Release(x, y) => TEvent::MouseUp(pt!(x - 1, y - 1)),
          MouseEvent::Hold(x, y) => TEvent::Drag(pt!(x - 1, y - 1)),
        }),
        Event::Key(key) => Some(TEvent::Key(*key)),
        _ => None,
      };
      if let Some(ev) = e {
        sender.send(ev).unwrap();
      }
    }
  });
  receiver
}

pub fn setup_stdout() -> MouseTerminal<termion::raw::RawTerminal<Stdout>> {
  MouseTerminal::from(stdout().into_raw_mode().unwrap())
}

pub fn go_to(pos: Point) {
  print!("{}", termion::cursor::Goto(pos.x + 1, pos.y + 1));
}

pub fn print_at(item: &String, pos: Point) {
  go_to(pos);
  print!("{}", item);
}

pub fn draw_lines_at(lines: &[String], pos: &Point) {
  let result = lines
    .iter()
    .enumerate()
    .map(|(index, line)| {
      format!(
        "{}{}",
        cursor::Goto(1 + pos.x, 1 + pos.y + index as u16),
        line
      )
    })
    .collect::<Vec<String>>()
    .join("");
  print!("{result}");
}

pub fn clear_all() {
  print!("{}", termion::clear::All);
}

pub fn hide() {
  print!("{}", termion::cursor::Hide);
}

pub fn show() {
  print!("{}", termion::cursor::Show);
}

#[allow(dead_code)]
pub fn fg_green(text: &str) -> String {
  format!(
    "{}{}{}",
    color::Fg(color::Green),
    text,
    color::Fg(color::Reset),
  )
}

pub fn draw_vertical_line(x: u16, y_start: u16, y_end: u16, str: String) {
  for y in y_start..y_end {
    go_to(pt!(x, y));
    print!("{str}");
  }
}

pub fn size() -> Point {
  let size = terminal_size().unwrap();
  Point::from_tuple(&size)
}
