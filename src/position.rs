use std::fmt;

/// Struct that stores a position. Used to store the mouse position and the position of the
/// images that are dropped onto the page
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = "x".to_string() + &self.x.to_string() + " y:" + &self.y.to_string();
        write!(f, "{}", str)
    }
}
