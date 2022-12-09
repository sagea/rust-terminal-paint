#[macro_export]
macro_rules! singleton {
    ($vis:vis static $idef:ident: $ty:ty = $item:expr) => {
      $vis static $idef: once_cell::sync::Lazy<tokio::sync::RwLock<$ty>> = once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new($item));
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

pub async fn get<T, RT>(
  item: &once_cell::sync::Lazy<tokio::sync::RwLock<T>>,
  getter: fn(&T) -> RT,
) -> RT {
  let state = item.read().await;
  let result = getter(&state);
  drop(state);
  result
}
