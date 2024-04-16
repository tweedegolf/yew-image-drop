use std::borrow::Borrow;

use gloo_console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, DomRect, HtmlCanvasElement, HtmlImageElement, SvgMatrix,
    SvgsvgElement,
};
use yew::prelude::*;
use yewdux::{use_dispatch, use_selector};

use crate::app_state::{AppState, ImageData, Msg};

#[derive(Clone, Properties, PartialEq)]
pub struct ImageProps {
    pub data: ImageData,
}

struct Data {
    pub width: f64,
    pub height: f64,
    pub natural_width: f64,
    pub natural_height: f64,
    pub pattern_width: f64,
    pub pattern_height: f64,
    pub shift_key_down: bool,
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
pub fn create(ImageProps { data }: &ImageProps) -> Html {
    let ImageData {
        id,
        url,
        width,
        height,
        natural_width,
        natural_height,
        pattern_width,
        pattern_height,
        use_pattern,
        ..
    } = data;

    let dispatch = use_dispatch();
    let svg_ref = use_node_ref();
    let image_ref = use_node_ref();
    let canvas_ref = use_node_ref();
    let shift_key_down = use_selector(|state: &AppState| state.shift_key_down);
    let shift_key_down: bool = *shift_key_down.borrow();
    let use_pattern = *use_pattern;

    let on_load = {
        let id2 = id.to_owned();
        dispatch.apply_callback(move |e: Event| {
            let target = e.target().unwrap();
            let img = target.dyn_ref::<HtmlImageElement>().unwrap();
            let rect: DomRect = img.get_bounding_client_rect();
            let w = rect.width() as i16;
            let h = rect.height() as i16;
            Msg::ImageLoaded(
                id2.clone(),
                w,
                h,
                img.natural_width() as i16,
                img.natural_height() as i16,
            )
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

    let s_ref = svg_ref.clone();
    let i_ref = image_ref.clone();
    let c_ref = canvas_ref.clone();
    let data = Data {
        width: *width as f64,
        height: *height as f64,
        natural_width: *natural_width as f64,
        natural_height: *natural_height as f64,
        pattern_width: *pattern_width as f64,
        pattern_height: *pattern_height as f64,
        shift_key_down,
    };

    let create_canvas = move || {
        if let Some(canvas) = c_ref.cast::<HtmlCanvasElement>() {
            match canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
            {
                Ok(ctx) => {
                    let svg_element = s_ref.cast::<SvgsvgElement>().unwrap();
                    let img_element = i_ref.cast::<HtmlImageElement>().unwrap();
                    ctx.set_fill_style(&JsValue::from_str("green"));
                    ctx.fill_rect(0., 0., data.width, data.height);

                    let sw = data.width / data.natural_width;
                    let sh = data.height / data.natural_height;
                    if data.shift_key_down || use_pattern {
                        match ctx.draw_image_with_html_image_element_and_dw_and_dh(
                            &img_element,
                            0.,
                            0.,
                            sw,
                            sh,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                log!("error drawImage", e);
                            }
                        }

                        let pattern_option = match ctx
                            .create_pattern_with_html_image_element(&img_element, "repeat")
                        {
                            Ok(pattern) => pattern,
                            Err(e) => {
                                log!("error draw pattern", e);
                                None
                            }
                        };
                        if let Some(pattern) = pattern_option {
                            let sw = data.pattern_width / data.natural_width;
                            let sh = data.pattern_height / data.natural_height;

                            let matrix: SvgMatrix = svg_element.create_svg_matrix();
                            let matrix = matrix.scale_non_uniform(sw as f32, sh as f32);

                            pattern.set_transform(&matrix);
                            ctx.set_fill_style(&pattern);
                            ctx.fill_rect(0., 0., data.width, data.height);
                        }
                    } else {
                        match ctx.draw_image_with_html_image_element_and_dw_and_dh(
                            &img_element,
                            0.,
                            0.,
                            sw * data.natural_width,
                            sh * data.natural_height,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                log!("error draw image", e)
                            }
                        }
                    }
                }
                Err(e) => {
                    log!("error get context", e);
                }
            }
        }
    };

    let data = (*width, *height, shift_key_down);
    let cc = create_canvas.clone();
    use_effect_with(data, move |_| {
        cc.borrow()();
    });

    log!("render ScalableImage");

    if *width == 0 && *height == 0 {
        html! {
            <img src={url.to_string()} class="image" onload={on_load}/>
        }
    } else {
        html! { <>
            <svg
                ref={svg_ref}
                xmlns="http://www.w3.org/2000/svg"
                width="0"
                height="0"
            />
            <img
                ref={image_ref}
                class="image-hidden"
                src={url.to_string()}
                onload={move |_e| create_canvas.borrow()()}
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
