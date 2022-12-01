use std::collections::HashSet;

use crate::{point::Point, state::State, term, tool::Tool};

pub struct SingleCanvasUpdate {
  pub brush: String,
  pub pixels: Vec<Point>,
}

impl SingleCanvasUpdate {
  pub fn new(brush: String, pixels: Vec<Point>) -> SingleCanvasUpdate {
    SingleCanvasUpdate { brush, pixels }
  }
  pub fn empty() -> SingleCanvasUpdate {
    SingleCanvasUpdate {
      brush: "".to_string(),
      pixels: vec![],
    }
  }
}

pub struct CanvasState {
  size: Point,
  full_buffer: Vec<String>,
  updates: Vec<SingleCanvasUpdate>,
}

impl CanvasState {
  pub fn new(size: Point) -> CanvasState {
    let row = (0..size.x + 1).map(|_| " ").collect::<String>();
    let full_buffer = (0..size.y).map(|_| row.clone()).collect::<Vec<String>>();
    CanvasState {
      size,
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
    None
  }

  pub fn clear_all(&mut self) {
    let mut pixels = vec![];
    for x in 0..self.size.x {
      for y in 0..self.size.y {
        pixels.push(Point::new(x, y));
      }
    }
    self.add_updates(SingleCanvasUpdate::new(" ".to_string(), pixels));
  }

  pub fn add_updates(&mut self, updates: SingleCanvasUpdate) {
    self.updates.push(updates);
  }

  pub fn commit_updates(&mut self) {
    for single_update in &self.updates {
      for pt in &single_update.pixels {
        if let Some(row) = self.full_buffer.get_mut(pt.y as usize) {
          row.replace_range(
            row
              .char_indices()
              .nth(pt.x as usize)
              .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
              .unwrap(),
            &single_update.brush,
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
) -> SingleCanvasUpdate {
  SingleCanvasUpdate::new(
    " ".to_string(),
    state
      .mouse_events
      .left_hover
      .iter()
      .filter(|pos| pos.is_inbetween(canvas_containment.0, canvas_containment.1))
      .copied()
      .collect::<Vec<Point>>(),
  )
}

pub fn handle_tool_brush(state: &State, canvas_containment: &(Point, Point)) -> SingleCanvasUpdate {
  SingleCanvasUpdate::new(
    state.brush.selected.clone(),
    state
      .mouse_events
      .left_hover
      .iter()
      .filter(|pos| pos.is_inbetween(canvas_containment.0, canvas_containment.1))
      .copied()
      .collect::<Vec<Point>>(),
  )
}

pub fn handle_tool_paint(
  state: &mut State,
  canvas_containment: &(Point, Point),
) -> SingleCanvasUpdate {
  let canvas_dimensions_start = Point::new(state.brush_menu_width, 0);
  let canvas_dimensions_end = state.terminal_size;
  let f = &state;
  if let Some(pos) = f.mouse_events.left_pressed {
    if pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end) {
      return SingleCanvasUpdate::new(
        state.brush.selected.clone(),
        fill_paint(state, pos, canvas_containment),
      );
    }
  }
  SingleCanvasUpdate::empty()
}

pub async fn update_canvas(state: &mut State) {
  let canvas_containment = (Point::new(state.brush_menu_width, 0), state.terminal_size);
  let updates = match &state.tools.selected {
    Tool::Brush => handle_tool_brush(state, &canvas_containment),
    Tool::Erasor => handle_tool_erasor(state, &canvas_containment),
    Tool::Paint => handle_tool_paint(state, &canvas_containment),
  };
  state.canvas_state.add_updates(updates);
}

pub async fn render_canvas(state: &mut State) {
  state.canvas_state.updates.iter().for_each(|single_update| {
    single_update.pixels.iter().for_each(|pos| {
      term::print_at(&single_update.brush, *pos);
    })
  });
  state.canvas_state.commit_updates();
}

pub struct Canvas {}
impl Canvas {
  pub async fn update(&mut self, state: &mut State) {
    update_canvas(state).await;
  }
  pub async fn render(&mut self, state: &mut State) {
    render_canvas(state).await;
  }
}
