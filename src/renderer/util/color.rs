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

pub const RED: RGB = RGB(1.0, 0.0, 0.0);
pub const CRIMSON: RGB = RGB(0.86, 0.08, 0.24);
pub const PINK: RGB = RGB(1.0, 0.75, 0.8);
pub const BLUSH: RGB = RGB(1.0, 0.4, 0.5);
pub const CORAL: RGB = RGB(1.0, 0.5, 0.31);
pub const ORANGE: RGB = RGB(1.0, 0.65, 0.0);
pub const AMBER: RGB = RGB(1.0, 0.75, 0.0);
pub const GOLD: RGB = RGB(1.0, 0.84, 0.0);
pub const YELLOW: RGB = RGB(1.0, 1.0, 0.0);
pub const LIME: RGB = RGB(0.75, 1.0, 0.0);
pub const SPRING: RGB = RGB(0.0, 1.0, 0.5);
pub const SEA: RGB = RGB(0.18, 0.55, 0.34);
pub const FOREST: RGB = RGB(0.13, 0.55, 0.13);
pub const GREEN: RGB = RGB(0.0, 1.0, 0.0);
pub const TEAL: RGB = RGB(0.0, 0.5, 0.5);
pub const AQUA: RGB = RGB(0.25, 0.88, 0.82);
pub const SKY: RGB = RGB(0.53, 0.81, 0.92);
pub const CYAN: RGB = RGB(0.0, 1.0, 1.0);
pub const BLUE: RGB = RGB(0.0, 0.0, 1.0);
pub const MIDNIGHT: RGB = RGB(0.1, 0.1, 0.44);
pub const INDIGO: RGB = RGB(0.29, 0.0, 0.51);
pub const PURPLE: RGB = RGB(0.5, 0.0, 0.5);
pub const PLUM: RGB = RGB(0.87, 0.63, 0.87);
pub const DUSK: RGB = RGB(0.25, 0.22, 0.45);
pub const MAGENTA: RGB = RGB(1.0, 0.0, 1.0);
pub const FERN: RGB = RGB(0.42, 0.74, 0.42);
pub const SALMON: RGB = RGB(0.98, 0.5, 0.45);
pub const BROWN: RGB = RGB(0.65, 0.16, 0.16);
pub const GRAY: RGB = RGB(0.5, 0.5, 0.5);
pub const SILVER: RGB = RGB(0.75, 0.75, 0.75);
pub const WHITE: RGB = RGB(1.0, 1.0, 1.0);
pub const BLACK: RGB = RGB(0.0, 0.0, 0.0);
pub const OBSIDIAN: RGB = RGB(0.05, 0.05, 0.08);
