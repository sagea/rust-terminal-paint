use std::sync::mpsc::Receiver;

use termion::event::Key;
use tokio::sync::Mutex;

use crate::canvas::Canvas;

use crate::side_menu::SideMenu;
use crate::state::{KEY_STATE, MOUSE_EVENTS};
use crate::term::TEvent;
use crate::{read, term as t, writ};

pub struct Comps {
  pub side_menu: SideMenu,
  pub canvas: Canvas,
}

use once_cell::sync::Lazy;

pub static COMPS: Lazy<Mutex<Comps>> = Lazy::new(|| {
  Mutex::new(Comps {
    side_menu: SideMenu {},
    canvas: Canvas {},
  })
});

pub async fn start_application() {
  let stdout = t::setup_stdout();
  let stdin = t::setup_stdin();
  t::clear_all();
  t::hide();
  let mut _is_first_render = true;
  loop {
    listen_for_events(&stdin).await;
    if should_stop_application().await {
      break;
    }
    phase_update_ui().await;
    phase_render_ui().await;
    _is_first_render = false;
  }
  t::show();
  drop(stdout);
}

async fn should_stop_application() -> bool {
  read!(KEY_STATE).was_key_pressed(&Key::Ctrl('c'))
}
async fn listen_for_events(events: &Receiver<TEvent>) {
  writ!(MOUSE_EVENTS | e | { e.reset() });
  writ!(KEY_STATE | e | { e.reset() });

  if let Ok(item) = events.try_recv() {
    writ!(MOUSE_EVENTS | e | { e.handle_terminal_event(&item) });
    writ!(KEY_STATE | e | { e.handle_terminal_event(&item) });
  }
}

async fn phase_update_ui() {
  let mut comps = COMPS.lock().await;
  comps.side_menu.update().await;
  comps.canvas.update().await;
}

async fn phase_render_ui() {
  let mut comps = COMPS.lock().await;
  comps.side_menu.render().await;
  comps.canvas.render().await;
}
