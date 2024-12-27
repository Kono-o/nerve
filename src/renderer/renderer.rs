use crate::{
   color, NerveCamera, NerveMesh, NerveShader, NerveShaderSrc, RenderAPI, Uniform, WinSize, RGB,
};
use cgmath::Matrix4;
use core::panic;
use gl::types::{GLenum, GLuint};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

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

   //STATE
   fn set_bg_color(&self, color: RGB);
   fn resize(&self, size: WinSize);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_cull(&self, enable: bool);
   fn set_cull_face(&self, face: Cull);
   fn set_wire_width(&self, thickness: f32);

   fn bind_program(&self, id: GLuint);
   fn unbind_program(&self);
   fn bind_texture(&self, tex_id: GLuint);
   fn unbind_texture(&self);

   //SHADERS
   fn compile_shader(&self, src: &str, typ: GLenum) -> GLuint;
   fn create_program(
      &self,
      vert: &str,
      frag: &str,
      image_ids: Vec<(String, GLuint)>,
   ) -> NerveShader;
   fn set_uni(&self, id: GLuint, name: &str, uniform: Uniform);
   fn set_uni_m4f32(&self, id: GLuint, name: &str, matrix: Matrix4<f32>);

   //BUFFERS

   //DRAW
   fn clear(&self);
   fn draw(&self, mesh: &NerveMesh);
}

pub struct NerveRenderer {
   pub(crate) renderer: Box<dyn Renderer>,

   pub(crate) cam_view: Matrix4<f32>,
   pub(crate) cam_proj: Matrix4<f32>,

   pub gpu: String,
   pub api: RenderAPI,
   pub api_ver: String,
   pub glsl_ver: String,

   pub poly_mode: PolyMode,
   pub cull_face: Cull,
   pub bg_color: RGB,
   pub msaa: bool,
   pub msaa_samples: u32,
   pub culling: bool,
}
//PRIVATE
impl NerveRenderer {
   pub(crate) fn from(
      renderer: Box<dyn Renderer>,
      api: RenderAPI,
      cam_view: Matrix4<f32>,
      cam_proj: Matrix4<f32>,
   ) -> Self {
      let (gpu, api_ver, glsl_ver) = renderer.info();
      let bg_color = color::OBSIDIAN;
      renderer.set_bg_color(bg_color);
      renderer.enable_depth(true);
      renderer.set_wire_width(2.0);
      Self {
         renderer,
         cam_view,
         cam_proj,
         gpu,
         api,
         api_ver,
         glsl_ver,
         poly_mode: PolyMode::Filled,
         cull_face: Cull::AntiClock,
         bg_color,
         msaa: false,
         msaa_samples: 0,
         culling: true,
      }
   }
   pub(crate) fn set_size(&mut self, size: WinSize) {
      self.renderer.resize(size);
   }
   fn clear_bg(&self) {
      self.renderer.clear()
   }

   pub(crate) fn pre_update(&mut self, cam: &NerveCamera) {
      self.cam_view = cam.view_matrix;
      self.cam_proj = cam.proj_matrix;
      self.clear_bg()
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
   pub fn toggle_wireframe(&mut self) {
      let new_poly_mode = match self.poly_mode {
         PolyMode::WireFrame => PolyMode::Filled,
         _ => PolyMode::WireFrame,
      };
      self.set_poly_mode(new_poly_mode);
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
   pub fn set_wire_width(&mut self, width: f32) {
      self.renderer.set_wire_width(width);
   }

   pub fn compile(&self, src: NerveShaderSrc) -> NerveShader {
      let (vert_src, frag_src) = match (
         PathBuf::from_str(&src.vert_path).unwrap().exists(),
         PathBuf::from_str(&src.frag_path).unwrap().exists(),
      ) {
         (true, true) => (
            fs::read_to_string(src.vert_path).unwrap_or("".to_string()),
            fs::read_to_string(src.frag_path).unwrap_or("".to_string()),
         ),
         _ => panic!("shader src do not exist!"),
      };
      self
         .renderer
         .create_program(&vert_src, &frag_src, Vec::new())
   }

   pub fn draw(&self, mesh: &mut NerveMesh) {
      if !mesh.shader.is_compiled || !mesh.visible || !mesh.alive {
         return;
      }
      mesh.transform.calc_matrix();
      let id = mesh.shader.id;
      self.renderer.bind_program(id);
      self.renderer.set_uni_m4f32(id, "u_CamView", self.cam_view);
      self.renderer.set_uni_m4f32(id, "u_CamProj", self.cam_proj);
      self
         .renderer
         .set_uni_m4f32(id, "u_MeshTransform", mesh.transform.matrix);
      self.renderer.draw(mesh);
   }
}
