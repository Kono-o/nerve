#[derive(Copy, Clone, Debug)]
pub struct Size2D {
   pub w: u32,
   pub h: u32,
}

impl Size2D {
   pub fn empty() -> Size2D {
      Self { w: 0, h: 0 }
   }
   pub fn from(w: u32, h: u32) -> Self {
      Self { w, h }
   }
   pub(crate) fn shave(&self, n: u32) -> Size2D {
      if self.w > 0 && self.h > 0 {
         Size2D {
            w: self.w - n,
            h: self.h - n,
         }
      } else {
         *self
      }
   }
}

#[derive(Copy, Clone, Debug)]
pub struct Size3D {
   pub w: u32,
   pub h: u32,
   pub d: u32,
}

impl Size3D {
   pub fn empty() -> Size3D {
      Self { w: 0, h: 0, d: 0 }
   }

   pub fn from(w: u32, h: u32, d: u32) -> Self {
      Self { w, h, d }
   }

   pub(crate) fn shave(&self, n: u32) -> Size3D {
      if self.w > 0 && self.h > 0 && self.d > 0 {
         Size3D {
            w: self.w - n,
            h: self.h - n,
            d: self.d - n,
         }
      } else {
         *self
      }
   }
}
