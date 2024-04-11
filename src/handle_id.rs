use std::fmt;
use yew::{html, virtual_dom::VNode, Html};

use crate::{bounding_box::BoundingBox, handle::Handle, position::Position};

const HANDLE_SIZE: i16 = 10;

/// This enum contains all resize handles for all 8 directions.
///
/// Implements the following functions:
/// - `to_string`&rarr; prints out a snake case id that can be used for the `id` attribute of the handle div
/// - `get_position`&rarr; returns the position where the handle should be rendered on the image
/// - `get_cursor`&rarr; returns the matching css style cursor type, based on the resize direction
/// - `get_bounding_box`&rarr; returns the x- and y-coordinate and the width and the height of the image container based on the new position of the handle
/// - `into_iter`&rarr; turns the enum into an iterable
/// - `get_html`&rarr; returns a html fragment that contains all 8 resize handles at their proper positions
#[derive(Clone, PartialEq)]
pub enum HandleId {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    MidRight,
    MidLeft,
    MidBottom,
    MidTop,
}

/// prints out a snake case id that can be used for the `id` attribute of the handle div
impl fmt::Display for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match &self {
            Self::TopLeft => "top_left",
            Self::TopRight => "top_right",
            Self::BottomLeft => "bottom_left",
            Self::BottomRight => "bottom_right",
            Self::MidRight => "mid_right",
            Self::MidLeft => "mid_left",
            Self::MidTop => "mid_top",
            Self::MidBottom => "mid_bottom",
        }
        .to_string();
        write!(f, "{}", id)
    }
}

impl HandleId {
    /// returns the position where the handle should be rendered on the image
    pub fn get_position(&self, width: i16, height: i16) -> (String, i16, i16) {
        match &self {
            Self::TopLeft => ("top_left".to_string(), 0, 0),
            Self::TopRight => ("top_right".to_string(), width - HANDLE_SIZE, 0),
            Self::BottomLeft => ("bottom_left".to_string(), 0, height - HANDLE_SIZE),
            Self::BottomRight => (
                "bottom_right".to_string(),
                width - HANDLE_SIZE,
                height - HANDLE_SIZE,
            ),
            Self::MidRight => (
                "mid_right".to_string(),
                width - HANDLE_SIZE,
                (height - HANDLE_SIZE) / 2,
            ),
            Self::MidLeft => ("mid_left".to_string(), 0, (height - HANDLE_SIZE) / 2),
            Self::MidTop => ("mid_top".to_string(), (width - HANDLE_SIZE) / 2, 0),
            Self::MidBottom => (
                "mid_bottom".to_string(),
                (width - HANDLE_SIZE) / 2,
                height - HANDLE_SIZE,
            ),
        }
    }

    /// returns the matching css style cursor type, based on the resize direction
    pub fn get_cursor(&self) -> String {
        match &self {
            Self::TopLeft => "nw-resize",
            Self::BottomLeft => "sw-resize",
            Self::TopRight => "ne-resize",
            Self::BottomRight => "se-resize",
            Self::MidRight => "e-resize",
            Self::MidLeft => "w-resize",
            Self::MidTop => "n-resize",
            Self::MidBottom => "s-resize",
        }
        .to_string()
    }

    /// returns the x- and y-coordinate and the width and the height of the image container
    /// based on the new position of the handle
    pub fn calculate_bounding_box(
        &self,
        img: BoundingBox,
        anchor: Position,
        mouse: Position,
        ratio: f32,
        keep_ratio: bool,
    ) -> BoundingBox {
        // log!("mouse", mouse.to_string());
        // log!("anchor", anchor.to_string());
        match &self {
            Self::TopLeft => {
                if keep_ratio {
                    let width = img.width + (img.x - mouse.x + anchor.x);
                    let height = (1.0 / ratio * width as f32) as i16;
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: img.y - (height - img.height),
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: mouse.y - anchor.y,
                        width: img.width + (img.x - mouse.x),
                        height: img.height + (img.y - mouse.y),
                    }
                }
            }

            Self::BottomLeft => {
                if keep_ratio {
                    let width = img.width + (img.x - mouse.x + anchor.x);
                    let height = (1.0 / ratio * width as f32) as i16;
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: img.y,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: img.y,
                        width: img.width + (img.x - mouse.x + anchor.x),
                        height: mouse.y - img.y + anchor.y,
                    }
                }
            }

            Self::TopRight => {
                if keep_ratio {
                    let width = mouse.x - img.x + anchor.x;
                    let height = (1.0 / ratio * width as f32) as i16;
                    BoundingBox {
                        x: img.x,
                        y: img.y - (height - img.height),
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: img.x,
                        y: mouse.y - anchor.y,
                        width: mouse.x - img.x + anchor.x,
                        height: img.height + (img.y - mouse.y),
                    }
                }
            }

            Self::BottomRight => {
                if keep_ratio {
                    let width = mouse.x - img.x + anchor.x;
                    let height = (1.0 / ratio * width as f32) as i16;
                    BoundingBox {
                        x: img.x,
                        y: img.y,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: img.x,
                        y: img.y,
                        width: mouse.x - img.x + anchor.x,
                        height: mouse.y - img.y + anchor.y,
                    }
                }
            }

            Self::MidRight => {
                if keep_ratio {
                    let width = mouse.x - img.x + anchor.x;
                    let height = (1.0 / ratio * width as f32) as i16;
                    let diff = (height - img.height) / 2;
                    BoundingBox {
                        x: img.x,
                        y: img.y - diff,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: img.x,
                        y: img.y,
                        width: mouse.x - img.x + anchor.x,
                        height: img.height,
                    }
                }
            }
            Self::MidLeft => {
                if keep_ratio {
                    let width = img.width + (img.x - mouse.x + anchor.x);
                    let height = (1.0 / ratio * width as f32) as i16;
                    let diff = (height - img.height) / 2;
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: img.y - diff,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: mouse.x - anchor.x,
                        y: img.y,
                        width: img.width + (img.x - mouse.x),
                        height: img.height,
                    }
                }
            }
            Self::MidTop => {
                if keep_ratio {
                    let height = img.height + (img.y - mouse.y);
                    let width = (ratio * height as f32) as i16;
                    let diff = (width - img.width) / 2;
                    BoundingBox {
                        x: img.x - diff,
                        y: mouse.y - anchor.y,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: img.x,
                        y: mouse.y - anchor.y,
                        width: img.width,
                        height: img.height + (img.y - mouse.y),
                    }
                }
            }
            Self::MidBottom => {
                if keep_ratio {
                    let height = img.height + (mouse.y - img.y - img.height);
                    let width = (ratio * height as f32) as i16;
                    let diff = (width - img.width) / 2;
                    BoundingBox {
                        x: img.x - diff,
                        y: img.y,
                        width,
                        height,
                    }
                } else {
                    BoundingBox {
                        x: img.x,
                        y: img.y,
                        width: img.width,
                        height: img.height + (mouse.y - img.y - img.height),
                    }
                }
            }
        }
    }

    /// Turns the enum into an iterable
    pub fn into_iter() -> core::array::IntoIter<HandleId, 8> {
        [
            HandleId::TopLeft,
            HandleId::TopRight,
            HandleId::BottomLeft,
            HandleId::BottomRight,
            HandleId::MidRight,
            HandleId::MidLeft,
            HandleId::MidTop,
            HandleId::MidBottom,
        ]
        .into_iter()
    }

    /// Returns a html fragment that contains all 8 resize handles at their proper positions
    pub fn get_html(width: i16, height: i16, image_id: String) -> VNode {
        html! {
          <>
            {
                HandleId::into_iter().map(|handle| {
                    let val = handle.get_position(width, height);
                    html! {
                      <Handle
                        key={val.0.clone()}
                        id={handle}
                        image_id={image_id.clone()}
                        x={val.1} y={val.2}
                      />
                    }
                }).collect::<Html>()
            }
          </>
        }
    }
}
