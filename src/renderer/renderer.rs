use crate::core::Renderer;
use crate::{color, WinSize, RGB};

#[derive(Copy, Clone)]
pub enum PolyMode {
   Points,
   WireFrame,
   Filled,
}
#[derive(Copy, Clone)]
pub enum Cull {
   Clock,
   AntiClock,
}

pub struct NerveRenderer {
   pub(crate) renderer: Box<dyn Renderer>,
   pub poly_mode: PolyMode,
   pub cull_face: Cull,
   pub bg_color: RGB,
   pub draw_bg: bool,
   pub depth: bool,
   pub msaa: bool,
   pub msaa_samples: u32,
   pub culling: bool,
}
//PRIVATE
impl NerveRenderer {
   pub(crate) fn new(context: Box<dyn Renderer>) -> Self {
      Self {
         renderer: context,
         poly_mode: PolyMode::Filled,
         cull_face: Cull::AntiClock,
         bg_color: color::BLACK,
         draw_bg: true,
         depth: true,
         msaa: false,
         msaa_samples: 0,
         culling: true,
      }
   }
   pub(crate) fn resize(&mut self, size: WinSize) {
      self.renderer.resize(size);
   }
   pub(crate) fn draw_bg(&self) {
      self.renderer.clear_bg();
      if self.depth {
         self.renderer.clear_depth()
      }
   }
}
//PUBLIC
impl NerveRenderer {
   pub fn set_msaa_samples(&mut self, samples: u32) {
      self.msaa_samples = samples
   }
   pub fn set_bg_color(&mut self, color: RGB) {
      self.bg_color = color;
      self.renderer.set_bg(color);
   }
   pub fn set_poly_mode(&mut self, mode: PolyMode) {
      self.poly_mode = mode;
      self.renderer.poly_mode(mode);
   }
   pub fn enable_msaa(&mut self, enable: bool) {
      self.msaa = enable;
      self.renderer.enable_msaa(enable);
   }
   pub fn enable_culling(&mut self, enable: bool) {
      self.culling = enable;
      self.renderer.enable_cull(enable);
   }
   pub fn set_cull_face(&mut self, cull_face: Cull) {
      self.cull_face = cull_face;
      self.renderer.set_cull_face(cull_face);
   }
   pub fn enable_depth(&mut self, enable: bool) {
      self.depth = enable;
      self.renderer.enable_depth(enable);
   }
   pub fn set_wire_width(&mut self, width: f32) {
      self.renderer.wire_thickness(width);
   }
}
