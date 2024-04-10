use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, HtmlImageElement};
use yew::prelude::*;
use yewdux::use_store;

use crate::app_state::{AppState, Msg};

#[derive(Clone, Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub url: String,
    pub width: i16,
    pub height: i16,
}

#[function_component(ScalableImage)]
pub fn scalable_image(
    ImageProps {
        id,
        url,
        width,
        height,
    }: &ImageProps,
) -> Html {
    let (_state, dispatch) = use_store::<AppState>();

    let on_load = {
        let id2 = id.to_owned();
        dispatch.apply_callback(move |e: Event| {
            let target = e.target().unwrap();
            let div = target.dyn_ref::<HtmlImageElement>().unwrap();
            let rect: DomRect = div.get_bounding_client_rect();
            let w = rect.width() as i16;
            let h = rect.height() as i16;
            Msg::ImageLoaded(id2.clone(), w, h)
        })
    };

    let on_pointer_down = {
        let id = id.clone();
        dispatch.apply_callback(move |e: MouseEvent| {
            e.prevent_default();
            let x = e.offset_x() as i16;
            let y = e.offset_y() as i16;
            Msg::SetActiveImage(id.clone(), x, y)
        })
    };

    let on_remove_image = {
        let id = id.clone();
        dispatch.apply_callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            Msg::RemoveImage(Some(id.clone()))
        })
    };

    // log!("--->", *width, *height);
    if *width == 0 && *height == 0 {
        html! {
          <img src={url.to_string()} class="image" onload={on_load}/>
        }
    } else {
        html! {
          <img
            src={url.clone()}
            width={width.to_string()}
            height={height.to_string()}
            onmousedown={on_pointer_down}
            ondblclick={on_remove_image}
          />
        }
    }
}
