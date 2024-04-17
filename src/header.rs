use gloo_console::log;
use yew::{function_component, html, Html};

use crate::logger::Logger;

/// Renders the header and the mouse position logger
#[function_component(Header)]
pub fn create() -> Html {
    log!("render Header");

    html! {
      <header>
        <div class="header-container">
          <h3>{ "drop an image below" }</h3>
          <Logger />
        </div>
      </header>
    }
}
