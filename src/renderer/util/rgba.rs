#[derive(Copy, Clone)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);
#[derive(Copy, Clone)]
pub struct RGB(pub f32, pub f32, pub f32);
impl RGBA {
   pub fn grey(lum: f32) -> Self {
      Self(lum, lum, lum, 1.0)
   }
   pub fn from_rgb(rgb: RGB, alpha: f32) -> Self {
      Self(rgb.0, rgb.1, rgb.2, alpha)
   }
   pub fn to_rgb(&self) -> RGB {
      RGB(self.0, self.1, self.2)
   }
}
impl RGB {
   pub fn grey(lum: f32) -> Self {
      Self(lum, lum, lum)
   }
   pub fn from_rgba(rgba: RGBA) -> Self {
      Self(rgba.0, rgba.1, rgba.2)
   }
   pub fn to_rgba(&self, alpha: f32) -> RGBA {
      RGBA(self.0, self.1, self.2, alpha)
   }
}
