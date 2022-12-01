use termion::color;

use crate::border::calculate_boder_size;
use crate::point::Point;
use crate::state::State;
use crate::tool::Tool;
use crate::{border, term};

pub async fn update_side_menu(state: &mut State) {
  ToolMenu::update(state, Point::zero());
  ClearButton::update(state, Point::new(0, 10));
  BrushMenu::update(state, Point::new(0, 20));
}

pub async fn render_side_menu(state: &State) {
  term::draw_vertical_line(
    state.brush_menu_width,
    0,
    state.terminal_size.y,
    "|".to_string(),
  );
  ToolMenu::render(&state, Point::zero());
  ClearButton::render(&state, Point::new(0, 10));
  BrushMenu::render(&state, Point::zero() + (0, 20));
}

pub fn render_brush_menu_item(pos: &Point, brush: &String, selected: bool) {
  if selected {
    print!("{}", color::Fg(color::Green));
  }
  let btn = border::draw_boder(brush.to_owned());
  term::draw_lines_at(&btn.lines, pos);
  if selected {
    print!("{}", color::Fg(color::Reset));
  }
}

struct ClearButton {}
impl ClearButton {
  pub fn update(state: &mut State, at: Point) {
    if let Some(pressed_position) = state.mouse_events.left_pressed {
      let end = at + calculate_boder_size(&"Clear All".to_string());
      if pressed_position.is_inbetween(at, end) {
        state.canvas_state.clear_all();
      }
    }
  }
  pub fn render(state: &State, at: Point) {
    render_brush_menu_item(&at, &"Clear All".to_string(), false);
  }
}

struct ToolMenu {}
impl ToolMenu {
  pub fn calculate_menu_items<'a>(state: &'a State, at: &Point) -> Vec<(&'a Tool, Point)> {
    let mut cur_pos = at.clone();
    let mut list = vec![];
    for tool in state.tools.list.iter() {
      list.push((tool, cur_pos));
      cur_pos.y += 3;
    }
    list
  }
  pub fn update(state: &mut State, at: Point) {
    if let Some(pressed_position) = state.mouse_events.left_pressed {
      let s = ToolMenu::calculate_menu_items(&state, &at);
      let selected = s.iter().find_map(|(tool, start)| {
        let end = start + calculate_boder_size(&tool.to_string());
        if pressed_position.is_inbetween(*start, end) {
          return Some(tool);
        }
        return None;
      });
      if let Some(item) = selected {
        state.tools.selected = *item.clone();
      }
    }
  }
  pub fn render(state: &State, at: Point) {
    term::go_to(at);
    print!("Tools:");
    ToolMenu::calculate_menu_items(&state, &(at + (0, 1)))
      .iter()
      .for_each(|(tool_r, pos)| {
        let selected = **tool_r == state.tools.selected;
        render_brush_menu_item(pos, &tool_r.to_string(), selected);
      });
  }
}

struct BrushMenu {}
impl BrushMenu {
  pub fn calculate_menu_items(state: &State, at: &Point) -> Vec<(String, Point)> {
    let mut cur_pos = at.clone();
    let mut list = vec![];
    for brush in state.brush.list.iter() {
      if cur_pos.x + 3 > state.brush_menu_width {
        cur_pos = Point::new(0, cur_pos.y + 3);
      }
      list.push((brush.clone(), cur_pos));
      cur_pos.x += 3;
    }
    list
  }
  pub fn update(state: &mut State, at: Point) {
    if let Some(pressed_position) = state.mouse_events.left_pressed {
      let s = BrushMenu::calculate_menu_items(&state, &at);
      let selected = s.iter().find_map(|(brush, start)| {
        let end = start + Point::from(3);
        if pressed_position.is_inbetween(*start, end) {
          return Some(brush);
        }
        return None;
      });
      if let Some(item) = selected {
        state.brush.selected = item.to_string();
      }
    }
  }
  pub fn render(state: &State, at: Point) {
    term::go_to(at);
    print!("Brushes:");
    // let next = at + (0, 1);
    BrushMenu::calculate_menu_items(&state, &(at + (0, 1)))
      .iter()
      .for_each(|(brush, pos)| {
        let selected = brush == &state.brush.selected;
        render_brush_menu_item(pos, brush, selected);
      });
  }
}
