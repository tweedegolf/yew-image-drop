use gloo_console::log;
use yew::prelude::*;
use yew_hooks::use_event_with_window;
use yewdux::use_store;

use crate::app_state::{AppState, Msg};

/// Just prints the mouse position, handy for debugging
#[function_component(Logger)]
pub fn image() -> Html {
    let (_, dispatch) = use_store::<AppState>();

    log!("render logger");
    html! {
      // <div class="logger">{state.mouse.to_string()}</div>
      <div class="logger"></div>
    }
}
