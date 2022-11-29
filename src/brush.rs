pub fn get_brush_list() -> Vec<String> {
  let list = vec![
    " ", "█", "▓", "▒", "░", "▀", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "▉", "▊", "▋", "▌", "▍", "▎",
    "▏", "▐", "▔", "▕", "▖", "▗", "▘", "▙", "▚", "▛", "▜", "▝", "▞", "▟",
  ];
  list.iter().map(|item| item.to_string()).collect()
}
