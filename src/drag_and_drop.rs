// use gloo_console::log;
use web_sys::Url;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::use_dispatch;

use crate::app_state::Msg;

/// Drag and drop component. The whole document is a drop area.
///
/// For more documentation see [yew_hooks](https://docs.rs/yew-hooks/latest/yew_hooks/struct.UseDropHandle.html)
#[function_component(UseDrop)]
pub fn create() -> Html {
    let node: NodeRef = use_node_ref();
    let drop_state = use_drop(node.clone());
    let dispatch = use_dispatch();

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
    <div>
        <div
          ref={node}
          class={
            if *drop_state.over {
              "drop-area-over"
            } else {
              "drop-area"
            }
          }
        />
      </div>
    }
}
