use crate::app_state::Msg;
use crate::components::drag_and_drop::UseDrop;
use crate::components::header::Header;
use crate::components::images::Images;
use yew::prelude::*;
use yew_hooks::use_event_with_window;
use yewdux::use_dispatch;

/// 1. Registers user input event listener that need to be handled on document level (mouseup, mousemove, keydown, keyup)
/// 2. Renders container div that holds the Yew app
#[function_component(App)]
pub fn create() -> Html {
    let dispatch = use_dispatch();

    {
        let dis = dispatch.clone();
        use_event_with_window("mousemove", move |e: MouseEvent| {
            e.prevent_default();
            let x = e.client_x() as i16;
            let y = e.client_y() as i16;
            dis.apply(Msg::MouseMove(x, y));
        });
    }

    {
        let dis = dispatch.clone();
        use_event_with_window("mouseup", move |_e: MouseEvent| {
            dis.apply(Msg::MouseUp);
        });
    }

    {
        let dis = dispatch.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            let msg = if KeyboardEvent::ctrl_key(&e) {
                Msg::CtrlKeyDown(true)
            } else if KeyboardEvent::shift_key(&e) {
                Msg::ShiftKeyDown(true)
            } else {
                Default::default()
            };
            dis.apply(msg);
        });
    }

    {
        let dis = dispatch.clone();
        use_event_with_window("keypress", move |e: KeyboardEvent| {
            let msg = if KeyboardEvent::key(&e) == "Delete" {
                Msg::RemoveImage(None)
            } else if KeyboardEvent::key(&e) == "+" {
                Msg::ImageToFront
            } else if KeyboardEvent::key(&e) == "-" {
                Msg::ImageToBack
            } else {
                Default::default()
            };
            dis.apply(msg);
        });
    }

    {
        let dis = dispatch.clone();
        use_event_with_window("keyup", move |e: KeyboardEvent| {
            let msg = if KeyboardEvent::key(&e) == "Control" {
                Msg::CtrlKeyDown(false)
            } else if KeyboardEvent::key(&e) == "Shift" {
                Msg::ShiftKeyDown(false)
            } else {
                Default::default()
            };
            dis.apply(msg);
        });
    }

    // log!("render App");

    html! {
      <UseDrop>
          <Header />
          <Images />
      </UseDrop>
    }
}
