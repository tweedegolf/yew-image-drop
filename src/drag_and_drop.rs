use web_sys::Url;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::use_store;

use crate::app_state::{AppState, Msg};

/// Drag and drop component. The whole document is a drop area.
///
/// For more documentation see [yew_hooks](https://docs.rs/yew-hooks/latest/yew_hooks/struct.UseDropHandle.html)
#[function_component(UseDrop)]
pub fn drop() -> Html {
    let node: NodeRef = use_node_ref();
    let state = use_drop(node.clone());
    let (_state, dispatch) = use_store::<AppState>();

    // Demo #2, use callback options.
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

    html! {
    <div>
        <div
          ref={node}
          class={
            if *state.over {
              "drop-area-over"
            } else {
              "drop-area"
            }
          }
        />
      </div>
    }
}
