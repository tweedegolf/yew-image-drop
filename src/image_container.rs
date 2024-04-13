use std::borrow::Borrow;

use crate::{
    absolute_style::AbsoluteStyle,
    app_state::{AppState, ImageData},
    handle_id::HandleId,
    scalable_image::ScalableImage,
};
use gloo_console::log;
use yew::prelude::*;
use yewdux::{use_selector, use_store};

#[derive(Clone, Properties, PartialEq)]
pub struct ImageContainerProps {
    pub id: String,
    pub url: String,
}

/// Renders a container for the image and the resize handles
///
/// When the user drags a resize handle the new dimensions of the container are calculated and stored in
/// the store. As soon as the store is updated the container renders again and passes on the new dimensions
/// to the ScalableImage component.
#[function_component(ImageContainer)]
pub fn image(ImageContainerProps { id, url }: &ImageContainerProps) -> Html {
    let images = use_selector(|state: &AppState| state.images.clone());
    let images: &Vec<ImageData> = images.borrow();
    let index = images.iter().position(|d| d.id == *id).unwrap();

    let width = images[index].width;
    let height = images[index].height;
    let style = AbsoluteStyle {
        x: images[index].x,
        y: images[index].y,
        width: Some(width),
        height: Some(height),
        z_index: Some(images[index].z_index),
    }
    .to_string();

    html! {
      <div
        key={id.clone()}
        class="image-container" style={style}
      >
        <ScalableImage
          id={id.clone()}
          url={url.clone()}
          width={width}
          height={height}
        />
        {HandleId::get_html(width, height, id.to_string())}
      </div>
    }
}
