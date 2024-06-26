use std::fmt;

/// Struct that stores the bounding box of the image. The term isn't completely correct because it only stores
/// x, y, width and height.
///
/// Used to store the position and the dimensions of the images.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = "x".to_string()
            + &self.x.to_string()
            + " y:"
            + &self.y.to_string()
            + " w:"
            + &self.width.to_string()
            + " h:"
            + &self.height.to_string();
        write!(f, "{}", str)
    }
}
