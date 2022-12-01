use std::sync::{mpsc::Receiver, Arc};

use crate::canvas;
use crate::side_menu;
use crate::state::State;
use crate::term as t;
use crate::term::TEvent;

use tokio::sync::Mutex;

pub async fn start_application() {
  let stdout = t::setup_stdout();
  let stdin = t::setup_stdin();
  t::clear_all();
  t::hide();
  let state = Arc::new(Mutex::new(State::new()));
  let cloned_state = Arc::clone(&state);
  let mut _is_first_render = true;
  loop {
    listen_for_events(&cloned_state, &stdin).await;
    {
      let state = cloned_state.lock().await;
      if state.was_key_pressed(&termion::event::Key::Ctrl('c')) {
        break;
      }
    }
    render_ui(&cloned_state).await;
    _is_first_render = false;
  }
  t::show();
  drop(stdout);
}

async fn listen_for_events(state: &Arc<Mutex<State>>, events: &Receiver<TEvent>) {
  let mut state = state.lock().await;
  state.reset_terminal_events();
  state.track_terminal_events(events);

  side_menu::update_side_menu(&mut state).await;
  canvas::update_canvas(&mut state).await;
}

async fn render_ui(state: &Arc<Mutex<State>>) {
  let mut state = state.lock().await;
  side_menu::render_side_menu(&state).await;
  canvas::render_canvas(&mut state).await;
}
