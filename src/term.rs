use std::io::Stdout;
use std::{io::stdout, sync::mpsc};
use termion::color;
use termion::raw::IntoRawMode;
use termion::{event::Event, input::MouseTerminal};

pub fn setup_stdin() -> mpsc::Receiver<Event> {
  use std::io::stdin;
  use termion::input::TermRead;
  let (sender, receiver) = mpsc::channel();
  let stdin = stdin();
  std::thread::spawn(move || {
    for c in stdin.events() {
      sender.send(c.unwrap()).unwrap();
    }
  });
  return receiver;
}

pub fn setup_stdout() -> MouseTerminal<termion::raw::RawTerminal<Stdout>> {
  MouseTerminal::from(stdout().into_raw_mode().unwrap())
}

pub fn go_to(x: u16, y: u16) {
  print!("{}", termion::cursor::Goto(x + 1, y + 1));
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
    go_to(x, y);
    print!("{str}");
  }
}
