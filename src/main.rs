//! # Yew Image Drop
//!
//! Simple app that allows you to drag and drop images onto the html page and position and scale them.

use crate::components::app::App;

mod absolute_style;
pub mod components {
    pub mod app;
    mod drag_and_drop;
    mod file_dialog;
    pub mod handle;
    mod header;
    mod image_container;
    mod images;
    mod logger;
    mod scalable_image;
}
mod app_state;
mod bounding_box;
mod handle_id;
mod position;

fn main() {
    yew::Renderer::<App>::new().render();
}
