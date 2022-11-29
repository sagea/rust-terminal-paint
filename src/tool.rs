#[derive(strum_macros::Display, PartialEq, Eq)]
pub enum Tool {
  Brush,
  Paint,
  Erasor,
}

pub fn get_tool_list() -> Vec<Tool> {
  vec![Tool::Brush, Tool::Paint, Tool::Erasor]
}

pub struct ToolState {
  pub selected: Tool,
  pub list: Vec<Tool>,
}

impl ToolState {
  pub fn new() -> Self {
    ToolState {
      ..Default::default()
    }
  }
}

impl Default for ToolState {
  fn default() -> Self {
    let list = get_tool_list();
    ToolState {
      selected: Tool::Brush,
      list,
    }
  }
}
