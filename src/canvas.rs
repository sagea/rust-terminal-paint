use crate::{point::Point, state::State, term};

pub struct CanvasState {
  full_buffer: Vec<String>,
  updates: Vec<(u16, u16, String)>,
}

impl CanvasState {
  pub fn new(size: (u16, u16)) -> CanvasState {
    let row = (0..size.0 + 1).map(|_| " ").collect::<String>();
    let full_buffer = (0..size.1).map(|_| row.clone()).collect::<Vec<String>>();
    CanvasState {
      full_buffer,
      updates: vec![],
    }
  }

  pub fn add_updates(&mut self, updates: &Vec<(u16, u16, String)>) {
    self.updates = updates.to_vec();
  }

  pub fn commit_updates(&mut self) {
    for (x, y, brush) in &self.updates {
      if let Some(row) = self.full_buffer.get_mut(*y as usize) {
        row.replace_range(
          row
            .char_indices()
            .nth(*x as usize)
            .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
            .unwrap(),
          &brush,
        );
      }
    }
  }
}

pub async fn update_canvas(state: &mut State) {
  let canvas_dimensions_start = (state.brush_menu_width, 0);
  let canvas_dimensions_end = state.terminal_size;
  if let Some(pos) = state.mouse_events.left_pressed {
    if pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end) {
      term::go_to(pos.0, pos.1);
      print!("{}", state.selected_brush);
    }
  }

  let updates: Vec<(u16, u16, String)> = state
    .mouse_events
    .left_hover
    .iter()
    .filter(|pos| pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end))
    .map(|pos| (pos.0, pos.1, state.selected_brush.clone()))
    .collect();
  state.canvas_state.add_updates(&updates);
}

pub async fn render_canvas(state: &mut State) {
  // term::print_at(&format!("length = {}", state.canvas_state.updates.len()), &10, &30);
  state.canvas_state.updates.iter().for_each(|(x, y, brush)| {
    term::print_at(brush, x, y);
  });
  state.canvas_state.commit_updates();
}
