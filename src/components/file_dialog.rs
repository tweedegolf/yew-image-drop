// use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Url};
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
        let mut urls = Vec::new();

        if let Some(file_list) = input.files() {
            let len = file_list.length();
            for i in 0..len {
                let file = file_list.item(i).unwrap();
                let url = Url::create_object_url_with_blob(&file).unwrap();
                urls.push(url.clone());
            }
            Msg::AddImages(urls)
        } else {
            Msg::None
        }
    });

    let on_click = {
        let i_ref = input_ref.clone();
        Callback::from(move |_| {
            let input_element = i_ref.cast::<HtmlInputElement>().unwrap();
            input_element.click();
        })
    };

    // log!("render FileDialog");

    html! {
      <>
        <input
            ref={input_ref}
            class="file_input" type="file"
            accept="image/png, image/jpeg"
            multiple={true}
            onchange={on_change}
        />
        <div class="file-dialog-button" onclick={on_click}>{"open file dialog"}</div>
      </>
    }
}
