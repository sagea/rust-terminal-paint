use std::sync::{mpsc::Receiver, Arc};

use crate::canvas::Canvas;

use crate::side_menu::SideMenu;
use crate::state::State;
use crate::term as t;
use crate::term::TEvent;

use tokio::sync::Mutex;

struct Comps {
  side_menu: SideMenu,
  canvas: Canvas,
}

pub async fn start_application() {
  let stdout = t::setup_stdout();
  let stdin = t::setup_stdin();
  t::clear_all();
  t::hide();
  let state = Arc::new(Mutex::new(State::new()));
  let components = Arc::new(Mutex::new(Comps {
    side_menu: SideMenu {},
    canvas: Canvas {},
  }));

  let cloned_state = Arc::clone(&state);
  let cloned_components = Arc::clone(&components);
  let mut _is_first_render = true;
  loop {
    listen_for_events(&cloned_state, &stdin).await;
    {
      let state = cloned_state.lock().await;
      if state.was_key_pressed(&termion::event::Key::Ctrl('c')) {
        break;
      }
    }

    phase_update_ui(&cloned_state, &cloned_components).await;
    phase_render_ui(&cloned_state, &cloned_components).await;
    _is_first_render = false;
  }
  t::show();
  drop(stdout);
}

async fn listen_for_events(state: &Arc<Mutex<State>>, events: &Receiver<TEvent>) {
  let mut state = state.lock().await;
  state.reset_terminal_events();
  state.track_terminal_events(events);
}

async fn phase_update_ui(state: &Arc<Mutex<State>>, components: &Arc<Mutex<Comps>>) {
  let mut state = state.lock().await;
  let mut comps = components.lock().await;
  comps.side_menu.update(&mut state).await;
  comps.canvas.update(&mut state).await;
}

async fn phase_render_ui(state: &Arc<Mutex<State>>, components: &Arc<Mutex<Comps>>) {
  let mut state = state.lock().await;
  let mut comps = components.lock().await;
  comps.side_menu.render(&mut state).await;
  comps.canvas.render(&mut state).await;
}
