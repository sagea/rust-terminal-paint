pub fn get_brush_list() -> Vec<String> {
  let list = vec![
    "█", "▓", "▒", "░", "▀", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "▉", "▊", "▋", "▌", "▍", "▎", "▏",
    "▐", "▔", "▕", "▖", "▗", "▘", "▙", "▚", "▛", "▜", "▝", "▞", "▟",
  ];
  list.iter().map(|item| item.to_string()).collect()
}

pub struct BrushState {
  pub selected: String,
  pub list: Vec<String>,
}

impl BrushState {
  pub fn new() -> BrushState {
    BrushState {
      ..Default::default()
    }
  }
}

impl Default for BrushState {
  fn default() -> Self {
    let brushes = get_brush_list();
    BrushState {
      selected: brushes.get(0).unwrap().clone(),
      list: brushes,
    }
  }
}
