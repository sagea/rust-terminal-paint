mod application;
mod border;
mod bounds;
mod brush;
mod canvas;
mod keys;
mod line_processor;
mod mouse;
mod point;
mod side_menu;
mod state;
mod term;
mod tool;

#[macro_use]
extern crate impl_ops;

use crate::application::start_application;

#[tokio::main]
async fn main() {
  start_application().await;
}
