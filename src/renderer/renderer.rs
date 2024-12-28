use crate::{
   color, DrawMode, NerveCamera, NerveMesh, NerveShader, NerveShaderSrc, NerveTexture, RenderAPI,
   Size2D, Uniform, RGB,
};
use cgmath::Matrix4;

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

#[derive(Copy, Clone, Debug)]
pub(crate) enum ShaderType {
   Vert,
   Frag,
}

pub(crate) trait Renderer {
   fn init(&self, window: &mut glfw::PWindow);
   fn info(&self) -> (String, String, String);

   //STATE
   fn set_bg_color(&self, color: RGB);
   fn resize(&self, size: Size2D);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_cull(&self, enable: bool);
   fn set_cull_face(&self, face: Cull);
   fn set_wire_width(&self, thickness: f32);

   fn bind_program(&self, id: u32);
   fn unbind_program(&self);

   fn bind_texture_at_slot(&self, tex_id: u32, slot: u32);
   fn unbind_texture(&self);

   //SHADERS
   fn create_shader(&self, src: &str, typ: ShaderType) -> u32;
   fn delete_shader(&self, id: u32);

   fn create_program(&self, vert: &str, frag: &str) -> u32;
   fn delete_program(&self, id: u32);

   fn create_texture_at_slot(&self, tex: &NerveTexture, slot: u32) -> u32;
   fn delete_texture(&self, id: u32);
   fn get_uni_location(&self, id: u32, name: &str) -> u32;

   fn set_uni(&self, id: u32, name: &str, uniform: Uniform);
   fn set_uni_i32(&self, id: u32, name: &str, int: i32);
   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>);

   //BUFFERS

   //DRAW
   fn clear(&self);
   fn draw(&self, draw_mode: &DrawMode, index_count: u32);
   fn draw_no_index(&self, draw_mode: &DrawMode, vert_count: u32);
}

pub struct NerveRenderer {
   pub(crate) core: Box<dyn Renderer>,

   pub(crate) cam_view: Matrix4<f32>,
   pub(crate) cam_proj: Matrix4<f32>,

   pub default_shader: NerveShader,

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
      core: Box<dyn Renderer>,
      api: RenderAPI,
      cam_view: Matrix4<f32>,
      cam_proj: Matrix4<f32>,
   ) -> Self {
      let (gpu, api_ver, glsl_ver) = core.info();
      let bg_color = color::OBSIDIAN;
      core.enable_depth(true);
      let mut renderer = Self {
         core,
         cam_view,
         cam_proj,
         default_shader: NerveShader::empty(),
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
      };
      let default_shader = renderer.compile(&NerveShaderSrc::default());
      renderer.set_culling(true);
      renderer.set_wire_width(2.0);
      renderer.set_bg_color(bg_color);
      renderer.default_shader = default_shader;
      renderer
   }
   pub(crate) fn set_size(&mut self, size: Size2D) {
      self.core.resize(size);
   }
   fn clear(&self) {
      self.core.clear()
   }

   pub(crate) fn pre_update(&mut self, cam: &NerveCamera) {
      self.cam_view = cam.view_matrix;
      self.cam_proj = cam.proj_matrix;
      self.clear()
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
      self.core.set_bg_color(color);
   }
   pub fn set_poly_mode(&mut self, mode: PolyMode) {
      self.poly_mode = mode;
      self.core.poly_mode(mode);
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
      self.core.enable_msaa(enable);
   }
   pub fn set_culling(&mut self, enable: bool) {
      if self.culling != enable {
         self.toggle_culling()
      }
      self.core.enable_cull(enable);
   }
   pub fn toggle_culling(&mut self) {
      self.culling = !self.culling;
      self.core.enable_cull(self.culling);
   }
   pub fn set_cull_face(&mut self, cull_face: Cull) {
      match self.cull_face {
         cull_face => {}
         _ => self.flip_cull_face(),
      }
   }
   pub fn flip_cull_face(&mut self) {
      self.cull_face = match self.cull_face {
         Cull::Clock => Cull::AntiClock,
         Cull::AntiClock => Cull::Clock,
      };
      self.core.set_cull_face(self.cull_face);
   }
   pub fn set_wire_width(&mut self, width: f32) {
      self.core.set_wire_width(width);
   }
   pub fn default_shader(&self) -> NerveShader {
      self.default_shader.clone()
   }
   pub fn compile(&self, src: &NerveShaderSrc) -> NerveShader {
      let p_id = self.core.create_program(&src.vert_src, &src.frag_src);
      self.core.bind_program(p_id);

      let mut image_ids = Vec::new();
      for (i, texture) in src.textures.iter().enumerate() {
         if texture.exists {
            let name = format!("tDif{}", i + 1);
            let t_id = self.core.create_texture_at_slot(texture, i as u32);
            self.core.set_uni_i32(p_id, &name, i as i32);
            image_ids.push(t_id);
         }
      }
      NerveShader {
         id: p_id,
         image_ids,
         exists_on_gpu: true,
      }
   }

   pub fn delete_shader(&self, shader: NerveShader) {
      self.core.delete_shader(shader.id)
   }

   pub fn draw(&self, mesh: &mut NerveMesh) {
      if !mesh.shader.exists_on_gpu || !mesh.visible || !mesh.alive {
         return;
      }
      mesh.update();
      let s = mesh.shader.id;
      self.core.bind_program(s);
      self.core.set_uni_m4f32(s, "uCamView", self.cam_view);
      self.core.set_uni_m4f32(s, "uCamProj", self.cam_proj);
      self.core.set_uni_m4f32(s, "uMeshTfm", mesh.matrix());

      for (i, t) in mesh.shader.image_ids.iter().enumerate() {
         self.core.bind_texture_at_slot(*t, i as u32);
      }

      mesh.vert_object.bind();
      if mesh.has_indices {
         mesh.index_object.bind();
         self.core.draw(&mesh.draw_mode, mesh.ind_count);
      } else {
         self.core.draw_no_index(&mesh.draw_mode, mesh.vert_count)
      }
   }
}
