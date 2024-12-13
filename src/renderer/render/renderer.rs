use crate::RGB;
use gl::types::GLfloat;

pub enum PolyMode {
   Points,
   WireFrame,
   Filled,
}

pub enum Cull {
   Clock,
   AntiClock,
}

pub struct NerveRenderer {
   pub bg_color: RGB,
   pub draw_bg: bool,
   pub depth: bool,
   pub msaa: bool,
   pub msaa_samples: u32,
   pub culling: bool,
   pub cull_face: Cull,
   pub poly_mode: PolyMode,
}

impl Default for NerveRenderer {
   fn default() -> Self {
      Self {
         bg_color: RGB::greyscale(0.05),
         draw_bg: true,
         depth: true,
         msaa: false,
         msaa_samples: 4,
         culling: true,
         cull_face: Cull::AntiClock,
         poly_mode: PolyMode::Filled,
      }
   }
}
impl NerveRenderer {
   pub fn set_bg_color(&mut self, color: RGB) {
      self.bg_color = color;
      unsafe { gl::ClearColor(self.bg_color.0, self.bg_color.1, self.bg_color.2, 1.0) }
   }
   pub fn set_msaa_samples(&mut self, samples: u32) {
      self.msaa_samples = samples
   }
   pub fn set_cull_face(&mut self, cull_face: Cull) {
      self.cull_face = cull_face
   }
   pub fn set_poly_mode(&mut self, mode: PolyMode) {
      self.poly_mode = mode;
      unsafe {
         match self.poly_mode {
            PolyMode::WireFrame => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
            PolyMode::Filled => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
            PolyMode::Points => {
               gl::PointSize(10.0);
               gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT)
            }
         }
      }
   }

   pub fn set_wireframe_width(&self, width: u32) {
      unsafe { gl::LineWidth(width as GLfloat) }
   }

   pub fn enable_depth(&mut self, enable: bool) {
      self.depth = enable;
      unsafe {
         match self.depth {
            true => gl::Enable(gl::DEPTH_TEST),
            false => gl::Disable(gl::DEPTH_TEST),
         }
      }
   }
   pub fn enable_msaa(&mut self, enable: bool) {
      self.msaa = enable;
      unsafe {
         match self.msaa {
            true => gl::Enable(gl::MULTISAMPLE),
            false => gl::Disable(gl::MULTISAMPLE),
         }
      }
   }
   pub fn enable_culling(&mut self, enable: bool) {
      self.culling = enable;

      unsafe {
         match self.culling {
            true => {
               gl::Enable(gl::CULL_FACE);
               gl::CullFace(gl::BACK);
               match self.cull_face {
                  Cull::Clock => gl::FrontFace(gl::CW),
                  Cull::AntiClock => gl::FrontFace(gl::CCW),
               }
            }
            false => gl::Disable(gl::CULL_FACE),
         }
      }
   }

   pub(crate) fn init(&mut self) {
      self.enable_depth(true);
      self.enable_msaa(true);
      self.enable_culling(true);
   }
   pub(crate) fn resize(&self, width: i32, height: i32) {
      unsafe { gl::Viewport(0, 0, width, height) }
   }
   pub(crate) fn draw_bg(&self) {
      unsafe {
         if self.draw_bg {
            match self.depth {
               true => gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT),
               false => gl::Clear(gl::COLOR_BUFFER_BIT),
            }
         }
      }
   }
}
