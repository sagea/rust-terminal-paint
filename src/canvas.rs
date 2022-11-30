use std::collections::HashSet;

use crate::{point::Point, state::State, term, tool::Tool};

pub struct CanvasState {
  full_buffer: Vec<String>,
  updates: Vec<Vec<(Point, String)>>,
}

impl CanvasState {
  pub fn new(size: Point) -> CanvasState {
    let row = (0..size.x + 1).map(|_| " ").collect::<String>();
    let full_buffer = (0..size.y).map(|_| row.clone()).collect::<Vec<String>>();
    CanvasState {
      full_buffer,
      updates: vec![],
    }
  }
  pub fn get_pixel(&self, pos: Point) -> Option<String> {
    if let Some(item) = self.full_buffer.get(pos.y as usize) {
      if let Some(pixel_info) = item.char_indices().nth(pos.x as usize) {
        return Some(pixel_info.1.to_string());
      }
    }
    return None;
  }
  pub fn add_updates(&mut self, updates: &Vec<(Point, String)>) {
    self.updates.push(updates.to_vec());
  }

  pub fn commit_updates(&mut self) {
    for single_update in &self.updates {
      for (pt, brush) in single_update {
        if let Some(row) = self.full_buffer.get_mut(pt.y as usize) {
          row.replace_range(
            row
              .char_indices()
              .nth(pt.x as usize)
              .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
              .unwrap(),
            &brush,
          );
        }
      }
    }
    self.updates.clear();
  }
}

pub fn fill_paint(state: &State, start: Point, containment: &(Point, Point)) -> Vec<Point> {
  let mut result = vec![];
  let mut stack: Vec<Point> = vec![start];
  let mut checked = HashSet::new();
  let matching_pixel = state.canvas_state.get_pixel(start).unwrap();
  if matching_pixel == state.brush.selected {
    // don't bother filling in an area that already uses the same brush.
    return vec![];
  }

  while !stack.is_empty() {
    let pos = stack.pop().unwrap();
    if !checked.contains(&pos) && pos.is_inbetween(containment.0, containment.1) {
      if let Some(pixel) = state.canvas_state.get_pixel(pos) {
        // make sure that the current pixel matches the same pixel
        if pixel == matching_pixel {
          result.push(pos);
          if pos.y > 0 {
            stack.push(Point::new(pos.x, pos.y - 1)); // top
          }
          if pos.x > 0 {
            stack.push(Point::new(pos.x - 1, pos.y)); // left
          }
          stack.push(Point::new(pos.x, pos.y + 1)); // bottom
          stack.push(Point::new(pos.x + 1, pos.y)); // right
        }
        checked.insert(pos);
      }
    }
  }
  result
}

pub fn handle_tool_erasor(
  state: &State,
  canvas_containment: &(Point, Point),
) -> Vec<(Point, String)> {
  state
    .mouse_events
    .left_hover
    .iter()
    .filter(|pos| pos.is_inbetween(canvas_containment.0, canvas_containment.1))
    .map(|pos| (*pos, " ".to_string()))
    .collect()
}

pub fn handle_tool_brush(
  state: &State,
  canvas_containment: &(Point, Point),
) -> Vec<(Point, String)> {
  state
    .mouse_events
    .left_hover
    .iter()
    .filter(|pos| pos.is_inbetween(canvas_containment.0, canvas_containment.1))
    .map(|pos| (*pos, state.brush.selected.clone()))
    .collect()
}

pub fn handle_tool_paint(
  state: &mut State,
  canvas_containment: &(Point, Point),
) -> Vec<(Point, String)> {
  let canvas_dimensions_start = Point::new(state.brush_menu_width, 0);
  let canvas_dimensions_end = state.terminal_size;
  let f = &state;
  if let Some(pos) = f.mouse_events.left_pressed {
    if pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end) {
      let mut result = vec![];
      for pt in fill_paint(&state, pos, canvas_containment) {
        result.push((pt, state.brush.selected.clone()));
      }
      return result;
    }
  }
  return vec![];
}

pub async fn update_canvas(state: &mut State) {
  let canvas_containment = (Point::new(state.brush_menu_width, 0), state.terminal_size);
  let updates = match &state.tools.selected {
    Tool::Brush => handle_tool_brush(state, &canvas_containment),
    Tool::Erasor => handle_tool_erasor(state, &canvas_containment),
    Tool::Paint => handle_tool_paint(state, &canvas_containment),
    _ => vec![],
  };
  state.canvas_state.add_updates(&updates);
}

pub async fn render_canvas(state: &mut State) {
  state.canvas_state.updates.iter().for_each(|single_update| {
    single_update.iter().for_each(|(pos, brush)| {
      term::print_at(brush, *pos);
    })
  });
  state.canvas_state.commit_updates();
}
