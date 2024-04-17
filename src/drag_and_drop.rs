use std::borrow::Borrow;

// use gloo_console::log;
use web_sys::Url;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::{use_dispatch, use_selector};

use crate::app_state::{AppState, Msg};

#[derive(Properties, PartialEq)]
pub struct DropProps {
    #[prop_or_default]
    pub children: Html,
}

/// Drag and drop component. The whole document is a drop area.
///
/// For more documentation see [yew_hooks](https://docs.rs/yew-hooks/latest/yew_hooks/struct.UseDropHandle.html)
#[function_component(UseDrop)]
pub fn create(DropProps { children }: &DropProps) -> Html {
    let node: NodeRef = use_node_ref();
    let drop_state = use_drop(node.clone());
    let dispatch = use_dispatch();
    let handle_id = use_selector(|state: &AppState| state.active_handle.clone());

    // If the user drags a resize handle show the cursor that matches the resize direction
    let style = if let Some(handle) = handle_id.borrow() {
        let cursor = handle.get_cursor();
        "cursor:".to_string() + &cursor + ";"
    } else {
        "".to_string()
    };

    let class_name = if *drop_state.over {
        "drop-area-over"
    } else {
        "drop-area"
    };

    let _ = use_drop_with_options(
        node.clone(),
        UseDropOptions {
            onfiles: Some(Box::new(move |files, _data_transfer| {
                // Process files or data_transfer
                let file = &files[0];
                let file_type = file.type_();
                if file_type.contains("image") {
                    let url = Url::create_object_url_with_blob(file).unwrap();
                    let call = dispatch.apply_callback(move |_: ()| Msg::AddImage(url.clone()));
                    call.emit(());
                }
            })),
            ..Default::default()
        },
    );

    // log!("render Drop");

    html! {
    <div
        ref={node}
        style={style}
        class={class_name}
      >
        {children.clone()}
      </div>
    }
}
