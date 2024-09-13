use std::sync::{atomic::Ordering, Arc};

use specta::specta;
use tauri::{command, State, WebviewWindow};

use crate::{
  util::{AppState, ErrToString},
  view::util::ctrl_to_window_and_data,
};

#[command]
#[specta]
pub fn toggle_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pointer_ignore);
  let condition = atomic.load(Ordering::Acquire);

  set_ignore_cursor_events(ctrl, state, !condition)?;

  Ok(!condition)
}

#[command]
#[specta]
pub fn set_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
  value: bool,
) -> Result<(), String> {
  let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pointer_ignore);

  // TODO:winapiを使って判定をなくす
  window.set_ignore_cursor_events(value).err_to_string()?;
  atomic.store(value, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pointer_ignore);

  Ok(atomic.load(Ordering::Acquire))
}
