use tokio::sync::RwLock;

use crate::{
  bounds::Bounds,
  brush::BrushState,
  canvas::CanvasState,
  keys::KeyState,
  mouse::MouseEventTracker,
  point::Point,
  pt,
  term::{self},
  tool::ToolState,
};

use once_cell::sync::Lazy;

#[macro_export]
macro_rules! singleton {
    ($vis:vis static $idef:ident: $ty:ty = $item:expr) => {
        $vis static $idef: Lazy<RwLock<$ty>> = Lazy::new(|| RwLock::new($item));
    };
}

#[macro_export]
macro_rules! read {
  ($state:ident) => {
    $state.read().await
  };
  ($state:ident |$ident:ident| $block:block) => {
    get(&$state, |$ident| $block).await
  };
}
#[macro_export]
macro_rules! writ {
  ($ident:ident) => {
    $ident.write().await
  };
  ($ident: ident |$foo:ident| $block: block) => {
    async {
      let mut $foo = $ident.write().await;
      $block;
    }
    .await
  };
}

pub async fn get<T, RT>(item: &Lazy<RwLock<T>>, getter: fn(&T) -> RT) -> RT {
  let state = item.read().await;
  let result = getter(&state);
  drop(state);
  result
}

pub static BRUSH_MENU_WIDTH: u16 = 20;

singleton!(pub static TERMINAL_SIZE: Point = term::size());
singleton!(pub static MOUSE_EVENTS: MouseEventTracker = MouseEventTracker::new());
singleton!(pub static KEY_STATE: KeyState = KeyState::new());
singleton!(pub static TOOL_STATE: ToolState = ToolState::new());
singleton!(pub static BRUSH_STATE: BrushState = BrushState::new());
singleton!(pub static CANVAS_STATE: CanvasState = CanvasState::new(Bounds {
  size: term::size() - pt!(BRUSH_MENU_WIDTH, 0),
  offset: pt!(BRUSH_MENU_WIDTH, 0),
}));
