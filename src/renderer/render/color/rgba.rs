pub struct RGBA(pub f32, pub f32, pub f32, pub f32);
pub struct RGB(pub f32, pub f32, pub f32);

impl RGBA {
   pub fn greyscale(luminance: f32) -> Self {
      Self(luminance, luminance, luminance, 1.0)
   }
   pub fn to_rgb(&self) -> RGB {
      RGB(self.0, self.1, self.2)
   }
}

impl RGB {
   pub fn greyscale(luminance: f32) -> Self {
      Self(luminance, luminance, luminance)
   }
   pub fn to_rgba(&self, alpha: f32) -> RGBA {
      RGBA(self.0, self.1, self.2, alpha)
   }
}
