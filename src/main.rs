mod application;
mod brush;
mod brush_menu;
mod canvas;
mod line_processor;
mod point;
mod state;
mod term;
mod tool;

use crate::application::start_application;

#[tokio::main]
async fn main() {
  start_application().await;
}
