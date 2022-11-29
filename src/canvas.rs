use crate::{point::Point, state::State, term};

pub async fn render_canvas(state: &State) {
  let canvas_dimensions_start = (state.brush_menu_width, 0);
  let canvas_dimensions_end = state.terminal_size;
  if let Some(pos) = state.mouse_events.left_pressed {
    if pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end) {
      term::go_to(pos.0, pos.1);
      print!("{}", state.selected_brush);
    }
  }
  let results = state
    .mouse_events
    .left_hover
    .iter()
    .filter(|pos| pos.is_inbetween(canvas_dimensions_start, canvas_dimensions_end));

  results.for_each(|pos| {
    term::go_to(pos.0, pos.1);
    print!("{}", state.selected_brush);
  });
}
