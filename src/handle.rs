use std::borrow::Borrow;

// use gloo_console::log;
use yew::prelude::*;
use yewdux::{use_dispatch, use_selector};

use crate::{
    absolute_style::AbsoluteStyle,
    app_state::{AppState, Msg},
    handle_id::HandleId,
};

#[derive(Clone, Properties, PartialEq)]
pub struct HandleProps {
    pub id: HandleId,
    pub image_id: String,
    pub x: i16,
    pub y: i16,
}
/// This component renders a resize handle. Every image has 8 resize handles.
///
/// The component has a pointer down event listener that stores the active handle in the store.
///
/// Once the store is updated, the handle renders again to set the cursor in the style attribute to `inherit`. This is done
/// because once a resize handle is active, the cursor changes to match the resize direction on document level.
#[function_component(Handle)]
pub fn create(HandleProps { image_id, id, x, y }: &HandleProps) -> Html {
    let dispatch = use_dispatch();
    let active_handle = use_selector(|state: &AppState| state.active_handle.clone());

    let on_pointer_down = {
        let handle_id = id.to_owned();
        let image_id = image_id.to_owned();
        dispatch.apply_callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            let x = e.offset_x() as i16;
            let y = e.offset_y() as i16;
            Msg::SetActiveHandle(handle_id.clone(), image_id.clone(), x, y)
        })
    };

    let mut style = AbsoluteStyle {
        x: *x,
        y: *y,
        ..Default::default()
    }
    .to_string();

    if let Some(handle_id) = active_handle.borrow() {
        if handle_id == id {
            style += "cursor: inherit"
        }
    }

    let class = if let Some(handle_id) = active_handle.borrow() {
        if handle_id == id {
            "handle handle-active"
        } else {
            "handle"
        }
    } else {
        "handle"
    };

    // log!("render handle", id.to_string());

    html! {
    <div
      onmousedown={on_pointer_down}
      class={class}
      style={style}
    />
    }
}
