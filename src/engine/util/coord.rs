use crate::Size2D;

#[derive(Copy, Clone, Debug)]
pub struct ScreenCoord {
   pub x: i32,
   pub y: i32,
}

impl ScreenCoord {
   pub fn empty() -> ScreenCoord {
      ScreenCoord { x: 0, y: 0 }
   }
   pub fn from(x: i32, y: i32) -> Self {
      Self { x, y }
   }
   pub fn from_tup((x, y): (i32, i32)) -> Self {
      Self { x, y }
   }
   pub fn is_inside(&self, size: Size2D) -> bool {
      if (self.x >= 0 && self.y >= 0) && (self.x <= size.w as i32 && self.y <= size.h as i32) {
         true
      } else {
         false
      }
   }
}

#[derive(Copy, Clone, Debug)]
pub struct ScreenOffset {
   pub x: i32,
   pub y: i32,
}

impl ScreenOffset {
   pub fn empty() -> ScreenOffset {
      ScreenOffset { x: 0, y: 0 }
   }
   pub fn from(x: i32, y: i32) -> Self {
      Self { x, y }
   }
   pub fn from_tup((x, y): (i32, i32)) -> Self {
      Self { x, y }
   }
}
