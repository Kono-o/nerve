use crate::Size2D;

#[derive(Copy, Clone, Debug)]
pub struct ScreenCoord {
   pub x: f64,
   pub y: f64,
}

impl ScreenCoord {
   pub fn empty() -> ScreenCoord {
      ScreenCoord { x: 0.0, y: 0.0 }
   }
   pub fn from(x: f64, y: f64) -> Self {
      Self { x, y }
   }
   pub fn from_tup((x, y): (f64, f64)) -> Self {
      Self { x, y }
   }
   pub fn is_inside(&self, size: Size2D) -> bool {
      if (self.x >= 0.0 && self.y >= 0.0) && (self.x <= size.w as f64 && self.y <= size.h as f64) {
         true
      } else {
         false
      }
   }
}

#[derive(Copy, Clone, Debug)]
pub struct ScreenOffset {
   pub x: f64,
   pub y: f64,
}

impl ScreenOffset {
   pub fn is_zero(&self) -> bool {
      self.x == 0.0 && self.y == 0.0
   }
}

impl ScreenOffset {
   pub fn empty() -> ScreenOffset {
      ScreenOffset { x: 0.0, y: 0.0 }
   }
   pub fn from(x: f64, y: f64) -> Self {
      Self { x, y }
   }
   pub fn from_tup((x, y): (f64, f64)) -> Self {
      Self { x, y }
   }
}
