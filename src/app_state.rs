use std::rc::Rc;

// use gloo_console::log;
use yewdux::{Reducer, Store};

use crate::{bounding_box::BoundingBox, handle_id::HandleId, position::Position};

/// - `anchor`&rarr; The position of the mouse down event relative to the image. In other words the offset of the mouse position.
/// - `lock`&rarr; When an image is being dragged around, the original position is stored.
/// The original position is used to calculate the offset to the new position
#[derive(Default, Clone, PartialEq, Store)]
pub struct ImageData {
    pub id: String,
    pub url: String,
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub natural_width: i16,
    pub natural_height: i16,
    pub pattern_width: i16,
    pub pattern_height: i16,
    pub ratio_wh: f32,
    pub z_index: i16,
    pub use_pattern: bool,
}
/// - `active_handle`&rarr; Is set as soon as the user clicks on a resize handle
/// - `active_image_index`&rarr; Is set as soon as the user clicks on an image or when the user clicks on a resize handle of that image
#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub images: Vec<ImageData>,
    pub mouse: Position,
    pub anchor: Position,
    pub lock: BoundingBox,
    pub active_handle: Option<HandleId>,
    pub active_image_index: Option<usize>,
    pub ctrl_key_down: bool,
    pub shift_key_down: bool,
    pub next_z_index: i16,
}

#[derive(Clone, Default)]
pub enum Msg {
    AddImages(Vec<String>),
    ImageLoaded(String, i16, i16, i16, i16),
    SetActiveHandle(HandleId, String, i16, i16),
    SetActiveImage(String, i16, i16),
    RemoveImage(Option<String>),
    MouseMove(i16, i16),
    MouseUp,
    CtrlKeyDown(bool),
    ShiftKeyDown(bool),
    ImageToFront,
    ImageToBack,
    #[default]
    None,
}

impl Reducer<AppState> for Msg {
    fn apply(self, mut app_state: Rc<AppState>) -> Rc<AppState> {
        let state = Rc::make_mut(&mut app_state);
        match self {
            Msg::MouseUp => {
                state.active_handle = None;
                state.active_image_index = None;
            }
            Msg::MouseMove(x, y) => {
                state.mouse.x = x;
                state.mouse.y = y;
                if let Some(index) = state.active_image_index {
                    let handle = state.active_handle.clone();
                    if let Some(handle) = handle {
                        let img_data = &mut state.images[index];
                        let bb = handle.calculate_bounding_box(
                            state.lock,
                            state.anchor,
                            state.mouse,
                            img_data.ratio_wh,
                            state.ctrl_key_down,
                        );
                        img_data.x = bb.x;
                        img_data.y = bb.y;
                        img_data.width = bb.width;
                        img_data.height = bb.height;
                        if !state.shift_key_down {
                            img_data.pattern_width = bb.width;
                            img_data.pattern_height = bb.height;
                        }
                    } else {
                        let img_data = &mut state.images[index];
                        img_data.x = x - state.anchor.x;
                        img_data.y = y - state.anchor.y;
                    }
                    // log!("Msg::MouseMove position", img_data.x, img_data.y);
                    // log!("Msg::MouseMove bounding box", bb.to_string());
                }
            }
            Msg::SetActiveImage(image_id, anchor_x, anchor_y) => {
                let index = state.images.iter().position(|d| d.id == image_id);
                if let Some(i) = index {
                    state.active_image_index = Some(i);
                    state.anchor.x = anchor_x;
                    state.anchor.y = anchor_y;
                    // log!("Msg::SetActiveImage", state.active_image_index);
                }
            }
            Msg::SetActiveHandle(handle_id, image_id, anchor_x, anchor_y) => {
                let index = state.images.iter().position(|d| d.id == image_id);
                if let Some(i) = index {
                    state.active_handle = Some(handle_id.clone());
                    state.active_image_index = Some(i);
                    state.anchor.x = anchor_x;
                    state.anchor.y = anchor_y;
                    let img_data = &mut state.images[i];
                    state.lock.x = img_data.x;
                    state.lock.y = img_data.y;
                    state.lock.width = img_data.width;
                    state.lock.height = img_data.height;
                    img_data.use_pattern = state.shift_key_down;
                }
            }
            Msg::AddImages(urls) => {
                let mut x = 50;
                let mut y = 50;
                let mut index = state.images.len();

                for url in urls.clone() {
                    let z_index = state.next_z_index + 1;
                    state.next_z_index = z_index;
                    let new_image = ImageData {
                        id: index.to_string(),
                        url: url.clone(),
                        x,
                        y,
                        width: 0,
                        height: 0,
                        natural_width: 0,
                        natural_height: 0,
                        pattern_width: 0,
                        pattern_height: 0,
                        ratio_wh: 0.0,
                        z_index,
                        use_pattern: false,
                    };
                    state.images.push(new_image);
                    x += 30;
                    y += 30;
                    index += 1;
                }
                // let length = state.images.len();
                // log!("Msg::AddImage", url.clone(), length);
            }
            Msg::ImageLoaded(id, width, height, natural_width, natural_height) => {
                let index = state.images.iter().position(|d| d.id == id);
                if let Some(i) = index {
                    let img_data = &mut state.images[i];
                    let r: f32 = width as f32 / height as f32;
                    img_data.ratio_wh = r;
                    img_data.width = 300;
                    img_data.height = (1.0 / r * 300.0) as i16;
                    img_data.natural_width = natural_width;
                    img_data.natural_height = natural_height;
                    img_data.pattern_width = img_data.width;
                    img_data.pattern_height = img_data.height;
                    // log!("Msg::ImageLoaded", width, height, r);
                }
            }
            Msg::RemoveImage(id) => {
                if let Some(id) = id {
                    let index = state.images.iter().position(|d| d.id == id);
                    if let Some(i) = index {
                        state.images.remove(i);
                        state.active_image_index = None;
                        // log!("Msg::RemoveImage", i);
                    }
                } else if let Some(i) = state.active_image_index {
                    state.images.remove(i);
                    state.active_image_index = None;
                    // log!("Msg::RemoveImage", i);
                }
            }
            Msg::ImageToFront => {
                if let Some(i) = state.active_image_index {
                    let z_index = state.images[i].z_index + 1;
                    state.images[i].z_index = if z_index > state.next_z_index {
                        state.next_z_index = z_index;
                        z_index
                    } else {
                        z_index
                    };
                    // log!("Msg::ImageToFront", z_index);
                }
            }
            Msg::ImageToBack => {
                if let Some(i) = state.active_image_index {
                    let z_index = state.images[i].z_index - 1;
                    state.images[i].z_index = if z_index < 0 { 0 } else { z_index }
                    // log!("Msg::ImageToBack", z_index);
                }
            }
            Msg::CtrlKeyDown(flag) => {
                state.ctrl_key_down = flag;
                // log!("Msg::CtrlKeyDown", flag);
            }
            Msg::ShiftKeyDown(flag) => {
                state.shift_key_down = flag;
                // log!("Msg::ShiftKeyDown", flag);
            }
            Msg::None => (),
        };

        app_state
    }
}
