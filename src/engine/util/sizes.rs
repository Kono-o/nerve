#[derive(Copy, Clone, Debug)]
pub struct WinSize {
   pub w: u32,
   pub h: u32,
}

impl WinSize {
   pub fn empty() -> WinSize {
      Self { w: 0, h: 0 }
   }
   pub fn from(w: u32, h: u32) -> Self {
      Self { w, h }
   }
   pub(crate) fn shave(&self, n: u32) -> WinSize {
      if self.w > 0 && self.h > 0 {
         WinSize {
            w: self.w - n,
            h: self.h - n,
         }
      } else {
         *self
      }
   }
}
