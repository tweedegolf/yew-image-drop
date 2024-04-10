use crate::{
    absolute_style::AbsoluteStyle, app_state::AppState, handle_id::HandleId,
    scalable_image::ScalableImage,
};
use yew::prelude::*;
use yewdux::use_store;

#[derive(Clone, Properties, PartialEq)]
pub struct ImageContainerProps {
    pub id: String,
    pub url: Option<String>,
}

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

    if let Some(u) = url {
        html! {
          <div
            key={id.clone()}
            class="image-container" style={style}
          >
            <ScalableImage
              id={id.clone()}
              url={u.clone()}
              width={width}
              height={height}
            />
            {HandleId::get_handles(width, height, id.to_string())}
          </div>
        }
    } else {
        html! {
          <div key={id.clone()} class="dropped-image-container" style={style}></div>
        }
    }
}
