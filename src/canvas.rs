use std::collections::HashSet;

use crate::{
  bounds::Bounds,
  point::Point,
  pt, read,
  state::{get, BRUSH_STATE, CANVAS_STATE, MOUSE_EVENTS, TOOL_STATE},
  term,
  tool::Tool,
  writ,
};

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
  pub full_buffer: Vec<String>,
  pub updates: Vec<SingleCanvasUpdate>,
  pub bounds: Bounds,
}

impl CanvasState {
  pub fn new(bounds: Bounds) -> CanvasState {
    let row = (0..bounds.size.x + 1).map(|_| " ").collect::<String>();
    let full_buffer = (0..bounds.size.y)
      .map(|_| row.clone())
      .collect::<Vec<String>>();
    CanvasState {
      bounds,
      full_buffer,
      updates: vec![],
    }
  }
  pub fn get_pixel(&self, pos: &Point) -> Option<String> {
    if let Some(item) = self.full_buffer.get(pos.y as usize) {
      if let Some(pixel_info) = item.char_indices().nth(pos.x as usize) {
        return Some(pixel_info.1.to_string());
      }
    }
    None
  }

  pub fn clear_all(&mut self) {
    let mut pixels = vec![];
    for x in 0..self.bounds.size.x {
      for y in 0..self.bounds.size.y {
        pixels.push(pt!(x, y));
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

pub async fn fill_paint(
  canvas_state: &CanvasState,
  selected_brush: &String,
  start: &Point,
  containment: &(Point, Point),
) -> Vec<Point> {
  let mut result = vec![];
  let mut stack: Vec<Point> = vec![*start];
  let mut checked = HashSet::new();
  let matching_pixel = canvas_state.get_pixel(start).unwrap();
  if &matching_pixel == selected_brush {
    // don't bother filling in an area that already uses the same brush.
    return vec![];
  }

  while !stack.is_empty() {
    let pos = stack.pop().unwrap();
    if !checked.contains(&pos) && pos.is_inbetween(containment.0, containment.1) {
      if let Some(pixel) = canvas_state.get_pixel(&pos) {
        // make sure that the current pixel matches the same pixel
        if pixel == matching_pixel {
          result.push(pos);
          if pos.y > 0 {
            stack.push(pt!(pos.x, pos.y - 1)); // top
          }
          if pos.x > 0 {
            stack.push(pt!(pos.x - 1, pos.y)); // left
          }
          stack.push(pt!(pos.x, pos.y + 1)); // bottom
          stack.push(pt!(pos.x + 1, pos.y)); // right
        }
        checked.insert(pos);
      }
    }
  }
  result
}

pub async fn handle_tool_erasor(canvas_bounds: &Bounds) -> SingleCanvasUpdate {
  SingleCanvasUpdate::new(
    " ".to_string(),
    read!(MOUSE_EVENTS)
      .left_hover
      .iter()
      .filter(|pos| canvas_bounds.contains_point(pos))
      .copied()
      .collect::<Vec<Point>>(),
  )
}

pub async fn handle_tool_brush(canvas_bounds: &Bounds) -> SingleCanvasUpdate {
  SingleCanvasUpdate::new(
    read!(BRUSH_STATE).selected.clone(),
    read!(MOUSE_EVENTS)
      .left_hover
      .iter()
      .filter(|pos| canvas_bounds.contains_point(pos))
      .copied()
      .collect::<Vec<Point>>(),
  )
}

pub async fn handle_tool_paint(canvas_bounds: &Bounds) -> SingleCanvasUpdate {
  let left_pressed = read!(MOUSE_EVENTS).left_pressed;
  let selected_brush_state = read!(BRUSH_STATE).selected.clone();

  if let Some(pos) = left_pressed {
    let canvas_state = read!(CANVAS_STATE);
    if canvas_bounds.contains_point(&pos) {
      let result = fill_paint(
        &canvas_state,
        &selected_brush_state,
        &pos,
        &(
          canvas_bounds.offset,
          canvas_bounds.offset + canvas_bounds.size,
        ),
      )
      .await;
      return SingleCanvasUpdate::new(selected_brush_state, result);
    }
  }
  SingleCanvasUpdate::empty()
}

pub async fn update_canvas() {
  let bounds = read!(CANVAS_STATE).bounds;
  let updates = match read!(TOOL_STATE).selected {
    Tool::Brush => handle_tool_brush(&bounds).await,
    Tool::Erasor => handle_tool_erasor(&bounds).await,
    Tool::Paint => handle_tool_paint(&bounds).await,
  };

  writ!(
    CANVAS_STATE | s | {
      s.add_updates(updates);
    }
  );
}

pub async fn render_canvas() {
  read!(
    CANVAS_STATE | s | {
      s.updates.iter().for_each(|single_update| {
        single_update.pixels.iter().for_each(|pos| {
          term::print_at(&single_update.brush, *pos);
        })
      });
    }
  );
  writ!(CANVAS_STATE | s | { s.commit_updates() });
}

pub struct Canvas {}
impl Canvas {
  pub async fn update(&mut self) {
    update_canvas().await;
  }
  pub async fn render(&mut self) {
    render_canvas().await;
  }
}
