use termion::color;

use crate::point::Point;
use crate::state::State;
use crate::term;

fn calculate_brush_menu_items(state: &State) -> Vec<(String, (u16, u16))> {
  let mut cur_x_pos: u16 = 0;
  let mut cur_y_pos: u16 = 0;
  let mut list = vec![];
  for brush in state.brush_list.iter() {
    if cur_x_pos + 3 > state.brush_menu_width {
      cur_x_pos = 0;
      cur_y_pos += 3;
    }
    list.push((brush.clone(), (cur_x_pos, cur_y_pos)));
    cur_x_pos += 3;
  }
  list
}

pub async fn update_brush_menu(state: &mut State) {
  if let Some(pressed_position) = state.mouse_events.left_pressed {
    let s = calculate_brush_menu_items(&state);
    let selected = s.iter().find_map(|(brush, start)| {
      let end = (start.0 + 3, start.1 + 3);
      if pressed_position.is_inbetween(*start, end) {
        return Some(brush);
      }
      return None;
    });
    if let Some(item) = selected {
      state.set_selected_item(item.to_string());
    }
  }
}

pub async fn render_brush_menu(state: &State) {
  term::draw_vertical_line(
    state.brush_menu_width,
    0,
    state.terminal_size.1,
    "|".to_string(),
  );
  calculate_brush_menu_items(&state)
    .iter()
    .for_each(|(brush, (x, y))| {
      let selected = brush == &state.selected_brush;
      render_brush_menu_item(*x, *y, brush, selected);
    });
}

pub fn render_brush_menu_item(x: u16, y: u16, brush: &String, selected: bool) {
  if selected {
    print!("{}", color::Fg(color::Green));
  }
  term::go_to(x, y);
  print!("┌─┐");
  term::go_to(x, y + 1);
  print!("|{brush}|");
  term::go_to(x, y + 2);
  print!("└─┘");
  if selected {
    print!("{}", color::Fg(color::Reset));
  }
}
