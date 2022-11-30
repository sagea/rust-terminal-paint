use crate::point::Point;

static BTL: &'static str = "┌";
static BTR: &'static str = "┐";
static BBL: &'static str = "└";
static BBR: &'static str = "┘";
static BV: &'static str = "|";
static BH: &'static str = "─";

pub struct BorderedText {
  pub size: Point,
  pub lines: Vec<String>,
}

pub fn draw_boder(text: String) -> BorderedText {
  let text_len = text.chars().count();
  let horz = (0..text_len)
    .map(|_| BH.to_string())
    .collect::<Vec<String>>()
    .join("");
  let lines = vec![
    format!("{BTL}{horz}{BTR}"),
    format!("{BV}{text}{BV}"),
    format!("{BBL}{horz}{BBR}"),
  ];
  BorderedText {
    size: calculate_boder_size(&text),
    lines,
  }
}

pub fn calculate_boder_size(text: &String) -> Point {
  let text_len = text.chars().count() as u16;
  Point::new(text_len + 2, 3)
}

// pub struct BText {
//   pub text: String,
//   pub size: Point,
// }

// impl BText {
//   pub fn new(text: String) -> Self {
//     BText { text: text }
//   }
// }
