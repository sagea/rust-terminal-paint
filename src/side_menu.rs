use termion::color;

use crate::border::calculate_boder_size;
use crate::point::Point;
use crate::state::{
  BRUSH_MENU_WIDTH, BRUSH_STATE, CANVAS_STATE, MOUSE_EVENTS, TERMINAL_SIZE, TOOL_STATE,
};
use crate::tool::Tool;
use crate::{border, pt, read, term, writ};

pub async fn update_side_menu() {
  ToolMenu::update(pt!(0)).await;
  ClearButton::update(pt!(0, 10)).await;
  BrushMenu::update(pt!(0, 20)).await;
}

pub async fn render_side_menu() {
  term::draw_vertical_line(BRUSH_MENU_WIDTH, 0, read!(TERMINAL_SIZE).y, "|".to_string());
  ToolMenu::render(pt!(0)).await;
  ClearButton::render(pt!(0, 10));
  BrushMenu::render(pt!(0) + pt!(0, 20)).await;
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
  pub async fn update(at: Point) {
    if let Some(pressed_position) = read!(MOUSE_EVENTS).left_pressed {
      let end = at + calculate_boder_size("Clear All");
      if pressed_position.is_inbetween(at, end) {
        let mut canvas_state = CANVAS_STATE.write().await;
        canvas_state.clear_all();
      }
    }
  }
  pub fn render(at: Point) {
    render_brush_menu_item(&at, &"Clear All".to_string(), false);
  }
}

struct ToolMenu {}
impl ToolMenu {
  pub async fn calculate_menu_items(at: &Point) -> Vec<(Tool, Point)> {
    let mut cur_pos = *at;
    let mut list = vec![];
    for tool in read!(TOOL_STATE).list.iter() {
      list.push((*tool, cur_pos));
      cur_pos.y += 3;
    }
    list
  }
  pub async fn update(at: Point) {
    if let Some(pressed_position) = read!(MOUSE_EVENTS).left_pressed {
      let s = ToolMenu::calculate_menu_items(&at).await;
      let selected = s.iter().find_map(|(tool, start)| {
        let end = start + calculate_boder_size(&tool.to_string());
        if pressed_position.is_inbetween(*start, end) {
          return Some(tool);
        }
        None
      });
      if let Some(item) = selected {
        writ!(TOOL_STATE).selected = *item;
        // state.tools.selected = **item;
      }
    }
  }
  pub async fn render(at: Point) {
    term::go_to(at);
    let selected = read!(TOOL_STATE).selected;
    ToolMenu::calculate_menu_items(&(at + (0, 1)))
      .await
      .iter()
      .for_each(|(tool_r, pos)| {
        let selected = *tool_r == selected;
        render_brush_menu_item(pos, &tool_r.to_string(), selected);
      });
  }
}

struct BrushMenu {}
impl BrushMenu {
  pub async fn calculate_menu_items(at: &Point) -> Vec<(String, Point)> {
    let mut cur_pos = *at;
    let mut list = vec![];
    for brush in read!(BRUSH_STATE).list.iter() {
      if cur_pos.x + 3 > BRUSH_MENU_WIDTH {
        cur_pos = pt!(0, cur_pos.y + 3);
      }
      list.push((brush.clone(), cur_pos));
      cur_pos.x += 3;
    }
    list
  }
  pub async fn update(at: Point) {
    if let Some(pressed_position) = read!(MOUSE_EVENTS).left_pressed {
      let s = BrushMenu::calculate_menu_items(&at).await;
      let selected = s.iter().find_map(|(brush, start)| {
        let end = start + Point::from(3);
        if pressed_position.is_inbetween(*start, end) {
          return Some(brush);
        }
        None
      });
      if let Some(item) = selected {
        let mut brush_state = BRUSH_STATE.write().await;
        brush_state.selected = item.to_string();
      }
    }
  }
  pub async fn render(at: Point) {
    term::go_to(at);
    print!("Brushes:");
    let selected = read!(BRUSH_STATE).selected.clone();
    BrushMenu::calculate_menu_items(&(at + (0, 1)))
      .await
      .iter()
      .for_each(|(brush, pos)| {
        let selected = brush == &selected;
        render_brush_menu_item(pos, brush, selected);
      });
  }
}

pub struct SideMenu {}
impl SideMenu {
  pub async fn update(&self) {
    update_side_menu().await;
  }
  pub async fn render(&self) {
    render_side_menu().await;
  }
}
