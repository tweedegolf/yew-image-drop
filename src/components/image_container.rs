use crate::{
    absolute_style::AbsoluteStyle, app_state::ImageData, components::scalable_image::ScalableImage,
    handle_id::HandleId,
};
// use gloo_console::log;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ImageContainerProps {
    pub data: ImageData,
}

/// Renders a container for the image and the resize handles
///
/// When the user drags a resize handle the new dimensions of the container are calculated and stored in
/// the store. As soon as the store is updated the container renders again and passes on the new dimensions
/// to the ScalableImage component.
#[function_component(ImageContainer)]
pub fn create(ImageContainerProps { data }: &ImageContainerProps) -> Html {
    let width = data.width;
    let height = data.height;
    let style = AbsoluteStyle {
        x: data.x,
        y: data.y,
        width: Some(width),
        height: Some(height),
        z_index: Some(data.z_index),
    }
    .to_string();

    // log!("render ImageContainer", data.id.clone());

    html! {
      <div
        key={data.id.clone()}
        class="image-container" style={style}
      >
        <ScalableImage
          key={data.id.clone()}
          data={data.clone()}
        />
        {HandleId::get_html(width, height, data.id.to_string())}
      </div>
    }
}
