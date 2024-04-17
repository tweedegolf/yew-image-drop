use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Event, Html};
use yewdux::use_dispatch;

use crate::app_state::Msg;

/// Renders a button that triggers a file dialog
#[function_component(FileDialog)]
pub fn create() -> Html {
    let dispatch = use_dispatch();
    let input_ref = use_node_ref();

    let on_change = dispatch.apply_callback(move |e: Event| {
        let target = e.target().unwrap();
        let input = target.dyn_ref::<HtmlInputElement>().unwrap();
        log!(input.files());
        Msg::None
        // Msg::ImageLoaded(
        //     id2.clone(),
        //     w,
        //     h,
        //     img.natural_width() as i16,
        //     img.natural_height() as i16,
        // )
    });

    let on_click = {
        let i_ref = input_ref.clone();
        Callback::from(move |_| {
            let input_element = i_ref.cast::<HtmlInputElement>().unwrap();
            input_element.click();
        })
    };

    log!("render FileDialog");

    html! {
      <>
        <input ref={input_ref} class="file_input" type="file" onchange={on_change} accept="image/png, image/jpeg"/>
        <div class="file-dialog-button" onclick={on_click}>{"open file dialog"}</div>
      </>
    }
}
