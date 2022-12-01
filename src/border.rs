use crate::point::Point;

static BTL: &str = "┌";
static BTR: &str = "┐";
static BBL: &str = "└";
static BBR: &str = "┘";
static BV: &str = "|";
static BH: &str = "─";

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

pub fn calculate_boder_size(text: &str) -> Point {
  let text_len = text.chars().count() as u16;
  Point::new(text_len + 2, 3)
}
