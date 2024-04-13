use gloo_console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, DomRect, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;
use yewdux::use_store;

use crate::app_state::{AppState, Msg};

#[derive(Clone, Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub url: String,
    pub width: i16,
    pub height: i16,
}

/// Component that renders the image
///
/// This component has 2 stages, in the first stage the image is added to the document with a class 'image' that
/// constrains the size of the image, and an onload handler.
///
/// Once the image is loaded, the onload handler dispatches the dimensions of the image to the store where the original width
/// and the height of the image are stored. This triggers a rerender and then the component enters its 2nd stage where it
/// is rendered with a fixed size and mouse handlers so that in this 2nd stage the image can be dragged around, resized and removed.
#[function_component(ScalableImage)]
pub fn scalable_image(
    ImageProps {
        id,
        url,
        width,
        height,
    }: &ImageProps,
) -> Html {
    let (_state, dispatch) = use_store::<AppState>();
    let image_ref = use_node_ref();
    let canvas_ref = use_node_ref();

    let on_load = {
        let id2 = id.to_owned();
        dispatch.apply_callback(move |e: Event| {
            let target = e.target().unwrap();
            let div = target.dyn_ref::<HtmlImageElement>().unwrap();
            let rect: DomRect = div.get_bounding_client_rect();
            let w = rect.width() as i16;
            let h = rect.height() as i16;
            Msg::ImageLoaded(id2.clone(), w, h)
        })
    };

    let on_pointer_down = {
        let id = id.clone();
        dispatch.apply_callback(move |e: MouseEvent| {
            e.prevent_default();
            let x = e.offset_x() as i16;
            let y = e.offset_y() as i16;
            Msg::SetActiveImage(id.clone(), x, y)
        })
    };

    let on_remove_image = {
        let id = id.clone();
        dispatch.apply_callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            Msg::RemoveImage(Some(id.clone()))
        })
    };

    let i_ref = image_ref.clone();
    let c_ref = canvas_ref.clone();
    let dim = (*width, *height);
    use_effect_with(dim, move |_| {
        if let Some(canvas) = c_ref.cast::<HtmlCanvasElement>() {
            match canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
            {
                Ok(ctx) => {
                    let img = i_ref.cast::<HtmlImageElement>().unwrap();
                    ctx.set_fill_style(&JsValue::from_str("green"));
                    // ctx.fill_rect(0., 0., dim.0 as f64, dim.1 as f64);

                    match ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                      &img, 
                      0.,
                      0.,
                      dim.0 as f64, 
                      dim.1 as f64,
                      0.,
                      0.,
                      dim.0 as f64, 
                      dim.1 as f64
                    ) {
                        Ok(_) => (),
                        Err(e) => {
                            log!("error draw image", e)
                        }
                    }
                }
                Err(e) => {
                    log!("error get context", e);
                }
            }
        }
    });

    // log!("--->", *width, *height);
    if *width == 0 && *height == 0 {
        html! {
          <img src={url.to_string()} class="image" onload={on_load}/>
        }
    } else {
        html! {<>
            <img
                ref={image_ref}
                class="image-hidden"
                src={url.to_string()}
                width={width.to_string()}
                height={height.to_string()}
            />
            <canvas
                ref={canvas_ref}
                width={width.to_string()}
                height={height.to_string()}
                onmousedown={on_pointer_down}
                ondblclick={on_remove_image}
            />
            </>
        }
    }
}
