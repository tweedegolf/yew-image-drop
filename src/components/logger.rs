// use gloo_console::log;
use yew::prelude::*;
use yewdux::use_selector;

use crate::app_state::AppState;

/// Just prints the mouse position, handy for debugging
#[function_component(Logger)]
pub fn create() -> Html {
    let mouse = use_selector(|state: &AppState| state.mouse);

    // log!("render Logger");
    html! {
      <div class="logger">{mouse.to_string()}</div>
    }
}
