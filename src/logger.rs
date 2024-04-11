use yew::prelude::*;
use yewdux::use_store;

use crate::app_state::AppState;

/// Just prints the mouse position, handy for debugging
#[function_component(Logger)]
pub fn image() -> Html {
    let (state, _dispatch) = use_store::<AppState>();

    html! {
      <div class="logger">{state.mouse.to_string()}</div>
    }
}
