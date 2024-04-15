use std::borrow::Borrow;

use gloo_console::log;
use yew::{function_component, html, Html};
use yewdux::use_selector;

use crate::{
    app_state::{AppState, ImageData},
    image_container::ImageContainer,
};

/// Renders all dropped images
#[function_component(Images)]
pub fn create() -> Html {
    let images = use_selector(|state: &AppState| state.images.clone());
    let images: &Vec<ImageData> = images.borrow();

    log!("render Images");

    images
        .clone()
        .into_iter()
        .map(|img| {
            let id = img.id.clone();
            html! {
            <ImageContainer
                key={id}
                data={img}
              />
            }
        })
        .collect::<Html>()
}
