pub struct RGBA(pub f32, pub f32, pub f32, pub f32);

impl RGBA {
   pub fn greyscale(grey: f32) -> Self {
      Self(grey, grey, grey, 1.0)
   }
}
