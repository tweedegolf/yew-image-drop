use std::fmt;

///
/// Struct that stores the absolute position and optionally the dimensions of a html element.
///
/// The `to_string` function prints out a css style string that can be set as the `style` attribute of a html element
///
#[derive(Default)]
pub struct AbsoluteStyle {
    pub x: i16,
    pub y: i16,
    pub width: Option<i16>,
    pub height: Option<i16>,
}

impl fmt::Display for AbsoluteStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut style =
            "left:".to_string() + &self.x.to_string() + "px;top:" + &self.y.to_string() + "px;";

        if let Some(width) = &self.width {
            style = style + "width:" + &width.to_string() + "px;"
        }
        if let Some(height) = &self.height {
            style = style + "height:" + &height.to_string() + "px;"
        }
        write!(f, "{}", style)
    }
}
