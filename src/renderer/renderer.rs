use crate::{color, RenderAPI, WinSize, RGB};

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

pub(crate) trait Renderer {
   fn init(&self, window: &mut glfw::PWindow, glfw: &mut glfw::Glfw);
   fn info(&self) -> (String, String, String);

   //RENDERING
   fn set_bg_color(&self, color: RGB);
   fn clear_bg(&self);
   fn clear_depth(&self);

   fn resize(&self, size: WinSize);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_cull(&self, enable: bool);
   fn set_cull_face(&self, face: Cull);
   fn wire_thickness(&self, thickness: f32);

   //SHADERS

   //BUFFERS
}

pub struct NerveRenderer {
   pub(crate) renderer: Box<dyn Renderer>,

   pub gpu: String,
   pub api: RenderAPI,
   pub api_ver: String,
   pub glsl_ver: String,

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
   pub(crate) fn from(renderer: Box<dyn Renderer>, api: RenderAPI) -> Self {
      let (gpu, api_ver, glsl_ver) = renderer.info();
      let bg_color = color::CRIMSON;
      renderer.set_bg_color(bg_color);
      Self {
         renderer,
         gpu,
         api,
         api_ver,
         glsl_ver,
         poly_mode: PolyMode::Filled,
         cull_face: Cull::AntiClock,
         bg_color,
         draw_bg: true,
         depth: true,
         msaa: false,
         msaa_samples: 0,
         culling: true,
      }
   }
   pub(crate) fn set_size(&mut self, size: WinSize) {
      self.renderer.resize(size);
   }
   fn draw_bg(&self) {
      self.renderer.clear_bg();
      if self.depth {
         self.renderer.clear_depth()
      }
   }

   pub(crate) fn pre_update(&self) {
      self.draw_bg()
   }
   pub(crate) fn post_update(&self) {}
}
//PUBLIC
impl NerveRenderer {
   pub fn display_info(&self) {
      let api = match self.api {
         RenderAPI::OpenGL(_, _) => "OpenGL",
         RenderAPI::Vulkan => "Vulkan",
      };
      println!("gpu: {}", self.gpu);
      println!("api: {} {}", api, self.api_ver);
      println!("gls: {}", self.glsl_ver);
   }
   pub fn set_msaa_samples(&mut self, samples: u32) {
      self.msaa_samples = samples
   }
   pub fn set_bg_color(&mut self, color: RGB) {
      self.bg_color = color;
      self.renderer.set_bg_color(color);
   }
   pub fn set_poly_mode(&mut self, mode: PolyMode) {
      self.poly_mode = mode;
      self.renderer.poly_mode(mode);
   }
   pub fn set_msaa(&mut self, enable: bool) {
      self.msaa = enable;
      self.renderer.enable_msaa(enable);
   }
   pub fn set_culling(&mut self, enable: bool) {
      self.culling = enable;
      self.renderer.enable_cull(enable);
   }
   pub fn set_cull_face(&mut self, cull_face: Cull) {
      self.cull_face = cull_face;
      self.renderer.set_cull_face(cull_face);
   }
   pub fn set_depth(&mut self, enable: bool) {
      self.depth = enable;
      self.renderer.enable_depth(enable);
   }
   pub fn set_wire_width(&mut self, width: f32) {
      self.renderer.wire_thickness(width);
   }
}
