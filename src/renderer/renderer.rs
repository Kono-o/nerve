use crate::renderer::ATTRInfo;
use crate::{
   color, DataType, DrawMode, NECamera, NEMesh, NEMeshSrc, NEShader, NEShaderSrc, NETexture,
   RenderAPI, Size2D, Uniform, RGB,
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
   fn set_clear(&self, color: RGB);
   fn resize(&self, size: Size2D);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_cull(&self, enable: bool);
   fn set_cull_face(&self, face: Cull);
   fn set_wire_width(&self, thickness: f32);

   fn bind_program(&self, id: u32);
   fn unbind_program(&self);

   fn bind_texture_at(&self, tex_id: u32, slot: u32);
   fn unbind_texture(&self);
   fn bind_buffer(&self, v_id: u32, b_id: u32);
   fn unbind_buffer(&self);
   fn bind_index_buffer(&self, id: u32);
   fn unbind_index_buffer(&self);

   //SHADERS
   fn create_shader(&self, src: &str, typ: ShaderType) -> u32;
   fn delete_shader(&self, id: u32);

   fn create_program(&self, vert: &str, frag: &str) -> u32;
   fn delete_program(&self, id: u32);

   fn create_texture(&self, tex: &NETexture) -> u32;
   fn delete_texture(&self, id: u32);
   fn get_uni_location(&self, id: u32, name: &str) -> u32;

   fn set_uni(&self, id: u32, name: &str, uniform: Uniform);
   fn set_uni_i32(&self, id: u32, name: &str, int: i32);
   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>); //BUFFERS

   //BUFFERS
   fn create_buffer(&self) -> (u32, u32);
   fn set_attr_layout(&self, info: &ATTRInfo, attr_id: u32, stride: usize, local_offset: usize);
   fn fill_buffer(&self, v_id: u32, b_id: u32, buffer: &Vec<u8>);
   fn fill_index_buffer(&self, id: u32, buffer: &Vec<u32>);
   fn delete_buffer(&self, v_id: u32, b_id: u32);
   fn create_index_buffer(&self) -> u32;
   fn delete_index_buffer(&self, id: u32);

   //DRAW
   fn clear(&self);
   fn draw(&self, draw_mode: &DrawMode, index_count: u32);
   fn draw_no_index(&self, draw_mode: &DrawMode, vert_count: u32);
}

pub struct NERenderer {
   pub(crate) core: Box<dyn Renderer>,

   pub(crate) cam_view: Matrix4<f32>,
   pub(crate) cam_proj: Matrix4<f32>,

   pub default_shader: NEShader,

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
impl NERenderer {
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
         default_shader: NEShader::empty(),
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
      let default_shader = renderer.compile(NEShaderSrc::default());
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

   pub(crate) fn pre_update(&mut self, cam: &NECamera) {
      self.cam_view = cam.view_matrix;
      self.cam_proj = cam.proj_matrix;
      self.clear()
   }
   pub(crate) fn post_update(&self) {}
}
//PUBLIC
impl NERenderer {
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
      self.core.set_clear(color);
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
   pub fn toggle_msaa(&mut self) {
      self.msaa = !self.msaa;
      self.core.enable_msaa(self.msaa)
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
   pub fn default_shader(&self) -> NEShader {
      self.default_shader.clone()
   }
   pub fn compile(&self, src: NEShaderSrc) -> NEShader {
      let p_id = self.core.create_program(&src.vert_src, &src.frag_src);
      self.core.bind_program(p_id);

      let mut image_ids = Vec::new();
      for (i, texture) in src.textures.iter().enumerate() {
         if texture.exists {
            let name = format!("tDif{}", i + 1);
            let t_id = self.core.create_texture(texture);
            self.core.set_uni_i32(p_id, &name, i as i32);
            image_ids.push(t_id);
         }
      }
      NEShader {
         id: p_id,
         image_ids,
         exists_on_gpu: true,
      }
   }

   pub fn delete_shader(&self, shader: NEShader) {
      self.core.delete_shader(shader.id)
   }

   pub fn mesh(&self, mut src: NEMeshSrc) -> NEMesh {
      let (v_id, b_id) = self.core.create_buffer();
      let i_id = self.core.create_index_buffer();

      let (mut pos_info, mut pos_data) = (ATTRInfo::empty(), &Vec::new());
      let (mut col_info, mut col_data) = (ATTRInfo::empty(), &Vec::new());
      let (mut uvm_info, mut uvm_data) = (ATTRInfo::empty(), &Vec::new());
      let (mut nrm_info, mut nrm_data) = (ATTRInfo::empty(), &Vec::new());
      let (mut ind_info, mut ind_data) = (ATTRInfo::empty(), &Vec::new());

      let mut cus_infos: Vec<ATTRInfo> = Vec::new();
      let mut cus_datas: Vec<&Vec<u8>> = Vec::new();

      let mut ind_count = 0;
      let mut vert_count = 0;
      let mut stride = 0;

      let mut pos_exists = src.pos_attr.has_data();
      let mut col_exists = src.col_attr.has_data();
      let mut uvm_exists = src.uvm_attr.has_data();
      let mut nrm_exists = src.nrm_attr.has_data();
      let mut cus_exists = src.has_custom_attrs();

      if pos_exists {
         pos_info = src.pos_attr.info();
         pos_data = src.pos_attr.data();
         stride += pos_info.elem_count * pos_info.byte_count;
      }
      if col_exists {
         col_info = src.col_attr.info();
         col_data = src.col_attr.data();
         stride += col_info.elem_count * col_info.byte_count;
      }
      if uvm_exists {
         uvm_info = src.uvm_attr.info();
         uvm_data = src.uvm_attr.data();
         stride += uvm_info.elem_count * uvm_info.byte_count;
      }
      if nrm_exists {
         nrm_info = src.nrm_attr.info();
         nrm_data = src.nrm_attr.data();
         stride += nrm_info.elem_count * nrm_info.byte_count;
      }
      if cus_exists {
         for cus_attr in src.cus_attrs.iter() {
            let cus_info = cus_attr.info();
            let cus_data = cus_attr.data();
            stride += cus_info.elem_count * cus_info.byte_count;
            cus_infos.push(cus_info);
            cus_datas.push(cus_data);
         }
      }
      let mut end = pos_data.len();
      if cus_exists && src.starts_with_custom() {
         end = cus_datas[0].len() / (cus_infos[0].byte_count * cus_infos[0].elem_count);
      }

      let mut buffer: Vec<u8> = Vec::new();
      for i in 0..end {
         vert_count += 1;
         if pos_exists {
            push_into_buffer(&mut buffer, &pos_data[i]);
         }
         if col_exists {
            push_into_buffer(&mut buffer, &col_data[i]);
         }
         if uvm_exists {
            push_into_buffer(&mut buffer, &uvm_data[i]);
         }
         if nrm_exists {
            push_into_buffer(&mut buffer, &nrm_data[i]);
         }
         if cus_exists {
            for (j, _attr) in src.cus_attrs.iter().enumerate() {
               let cus_byte_count = cus_infos[j].byte_count * cus_infos[j].elem_count;
               let cus_data = cus_datas[j];
               let start = (i * cus_byte_count);
               let end = ((i + 1) * (cus_byte_count)) - 1;
               push_into_buffer(&mut buffer, &cus_data[start..=end]);
            }
         }
      }

      let mut attr_id = 0;
      let mut local_offset = 0;
      self.core.bind_buffer(v_id, b_id);
      let mut layouts: Vec<String> = Vec::new();
      if pos_info.exists {
         self
            .core
            .set_attr_layout(&pos_info, attr_id, stride, local_offset);
         local_offset += (pos_info.elem_count * pos_info.byte_count);
         layouts.push(format!("position(f32x3): {:?}", attr_id));
         attr_id += 1;
      }
      if col_info.exists {
         self
            .core
            .set_attr_layout(&col_info, attr_id, stride, local_offset);
         local_offset += (col_info.elem_count * col_info.byte_count);
         layouts.push(format!("color(f32x3): {:?}", attr_id));
         attr_id += 1;
      }
      if uvm_info.exists {
         self
            .core
            .set_attr_layout(&uvm_info, attr_id, stride, local_offset);
         local_offset += (uvm_info.elem_count * uvm_info.byte_count);
         layouts.push(format!("uvmap(f32x2): {:?}", attr_id));
         attr_id += 1;
      }
      if nrm_info.exists {
         self
            .core
            .set_attr_layout(&nrm_info, attr_id, stride, local_offset);
         local_offset += (nrm_info.elem_count * nrm_info.byte_count);
         layouts.push(format!("normal(f32x3): {:?}", attr_id));
         attr_id += 1;
      }

      for (i, cus_info) in cus_infos.iter().enumerate() {
         if cus_info.exists {
            self
               .core
               .set_attr_layout(&cus_info, attr_id, stride, local_offset);
            local_offset += (nrm_info.elem_count * nrm_info.byte_count);
            let format = cus_info.typ_str.clone();
            layouts.push(format!("custom-{i}({format}): {:?}", attr_id));
            attr_id += 1;
         }
      }
      if buffer.len() > 0 {
         self.core.fill_buffer(v_id, b_id, &buffer);
      }
      self.core.unbind_buffer();

      let mut index_buffer: Vec<u32> = Vec::new();
      if src.indices.has_data() {
         ind_info = src.indices.info();
         ind_data = src.indices.data();
         ind_info.exists = true;
         for index in ind_data.iter() {
            ind_count += 1;
            index_buffer.push(*index);
         }
         self.core.bind_index_buffer(i_id);
         self.core.fill_index_buffer(i_id, &index_buffer);
         self.core.unbind_index_buffer();
      }
      src.transform.calc_matrix();
      NEMesh {
         shader: src.shader.clone(),
         has_indices: ind_info.exists,
         vert_count,
         ind_count,
         is_empty: false,
         layouts,
         buf_id: (v_id, b_id),
         index_buf_id: i_id,
         transform: src.transform.clone(),
         ..Default::default()
      }
   }

   pub fn draw(&self, mesh: &mut NEMesh) {
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
         self.core.bind_texture_at(*t, i as u32);
      }

      self.core.bind_buffer(mesh.buf_id.0, mesh.buf_id.1);
      if mesh.has_indices {
         self.core.bind_index_buffer(mesh.index_buf_id);
         self.core.draw(&mesh.draw_mode, mesh.ind_count);
      } else {
         self.core.draw_no_index(&mesh.draw_mode, mesh.vert_count)
      }
   }
}

fn push_into_buffer<T: DataType>(buffer: &mut Vec<u8>, attr: &[T]) {
   for elem in attr.iter() {
      let bytes = elem.u8ify();
      for byte in bytes.iter() {
         buffer.push(*byte);
      }
   }
}
