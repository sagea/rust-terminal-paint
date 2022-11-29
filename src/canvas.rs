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

  pub fn add_updates(&mut self, updates: &Vec<(Point, String)>) {
    self.updates.push(updates.to_vec());
    // self.updates = updates.to_vec();
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

pub async fn update_canvas(state: &mut State) {
  let canvas_dimensions_start = Point::new(state.brush_menu_width, 0);
  let canvas_dimensions_end = state.terminal_size;
  let updates = match &state.tools.selected {
    // todo: consolidate Tool::Brush and Tool::Erasor
    Tool::Brush => state
      .mouse_events
      .left_hover
      .iter()
      .filter(|pos| pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end))
      .map(|pos| (*pos, state.brush.selected.clone()))
      .collect(),
    Tool::Erasor => state
      .mouse_events
      .left_hover
      .iter()
      .filter(|pos| pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end))
      .map(|pos| (*pos, " ".to_string()))
      .collect(),
    _ => vec![],
  };
  state.canvas_state.add_updates(&updates);
}

pub async fn render_canvas(state: &mut State) {
  state.canvas_state.updates.iter().for_each(|single_update| {
    single_update.iter().for_each(|(pos, brush)| {
      term::print_at(brush, &pos.x, &pos.y);
    })
  });
  state.canvas_state.commit_updates();
}
