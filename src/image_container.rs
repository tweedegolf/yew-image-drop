use crate::{
    absolute_style::AbsoluteStyle, app_state::AppState, handle_id::HandleId,
    scalable_image::ScalableImage,
};
use gloo_console::log;
use yew::prelude::*;
use yewdux::use_store;

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
    let (state, _dispatch) = use_store::<AppState>();
    let index = state.images.iter().position(|d| d.id == *id).unwrap();

    let width = state.images[index].width;
    let height = state.images[index].height;
    let style = AbsoluteStyle {
        x: state.images[index].x,
        y: state.images[index].y,
        width: Some(width),
        height: Some(height),
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
