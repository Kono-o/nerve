use crate::Size2D;

#[derive(Copy, Clone, Debug)]
pub struct Coord2D {
   pub x: f64,
   pub y: f64,
}

impl Coord2D {
   pub fn empty() -> Coord2D {
      Coord2D { x: 0.0, y: 0.0 }
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
pub struct CoordOffset {
   pub x: f64,
   pub y: f64,
}

impl CoordOffset {
   pub fn is_zero(&self) -> bool {
      self.x == 0.0 && self.y == 0.0
   }
}

impl CoordOffset {
   pub fn empty() -> CoordOffset {
      CoordOffset { x: 0.0, y: 0.0 }
   }
   pub fn from(x: f64, y: f64) -> Self {
      Self { x, y }
   }
   pub fn from_tup((x, y): (f64, f64)) -> Self {
      Self { x, y }
   }
}
