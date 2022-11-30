mod application;
mod border;
mod brush;
mod canvas;
mod line_processor;
mod point;
mod side_menu;
mod state;
mod term;
mod tool;

use crate::application::start_application;

#[tokio::main]
async fn main() {
  start_application().await;
}
