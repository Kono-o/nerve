use crate::asset::ATTRInfo;
use crate::renderer::handles::{DrawMode, NEMesh3D, NEShader};
use crate::renderer::MeshHandle;
use crate::{
   ansi, color, log_info, ClipDist, DataType, NECamera, NEError, NEMeshAsset, NEResult,
   NEShaderAsset, NETexAsset, NETexture, RenderAPI, Size2D, TexFilter, TexWrap, Transform3D, RGB,
};
use cgmath::{Matrix4, Vector2};
use std::ops::Deref;

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
   fn log_info(&self);

   //STATE
   fn set_clear(&self, color: RGB);
   fn resize(&self, size: Size2D);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_alpha(&self, enable: bool);
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
   fn create_spv_shader(&self, spv: &Vec<u8>, typ: ShaderType) -> NEResult<u32>;
   fn create_src_shader(&self, src: &str, typ: ShaderType) -> NEResult<u32>;
   fn delete_shader(&self, id: u32);

   fn create_spv_program(&self, nshdr: &NEShaderAsset) -> NEResult<u32>;
   fn create_src_program(&self, vert: &str, frag: &str) -> NEResult<u32>;
   fn delete_program(&self, id: u32);

   fn create_texture(&self, tex: &NETexAsset) -> u32;
   fn delete_texture(&self, id: u32);
   fn get_uni_location(&self, id: u32, name: &str) -> u32;

   fn set_uni_i32(&self, id: u32, name: &str, int: i32);
   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>);
   fn set_uni_vec2f32(&self, id: u32, name: &str, vec2: Vector2<f32>);

   //BUFFERS
   fn create_buffer(&self) -> (u32, u32);
   fn set_attr(&self, info: &ATTRInfo, attr_id: u32, stride: usize, local_offset: usize);
   fn fill_buffer(&self, v_id: u32, b_id: u32, buffer: &Vec<u8>);
   fn fill_index_buffer(&self, id: u32, buffer: &Vec<u32>);
   fn delete_buffer(&self, v_id: u32, b_id: u32);
   fn create_index_buffer(&self) -> u32;
   fn delete_index_buffer(&self, id: u32);

   //DRAW
   fn clear(&self);
   fn draw_indexed(&self, draw_mode: &DrawMode, index_count: u32);
   fn draw_array(&self, draw_mode: &DrawMode, vert_count: u32);
}

pub enum NERendererErrKing {
   NoGLSLValidator,
   GLSLCompileFailed,
   CreateShaderFailed,
   CreateProgramFailed,
}

pub struct NERenderer {
   pub(crate) core: Box<dyn Renderer>,

   pub(crate) size: Size2D,
   pub(crate) clip: ClipDist,

   pub(crate) cam_view: Matrix4<f32>,
   pub(crate) cam_proj: Matrix4<f32>,

   pub(crate) fallback_shader: NEShader,
   pub(crate) fallback_texture: NETexture,

   pub(crate) api: RenderAPI,
   pub(crate) poly_mode: PolyMode,
   pub(crate) cull_face: Cull,
   pub(crate) bg_color: RGB,
   pub(crate) msaa: bool,
   pub(crate) msaa_samples: u32,
   pub(crate) culling: bool,
}

//PRIVATE
impl NERenderer {
   pub(crate) fn from(core: Box<dyn Renderer>, api: RenderAPI, cam: &NECamera) -> NERenderer {
      let bg_color = color::OBSIDIAN;
      core.enable_depth(true);

      let mut renderer = NERenderer {
         api,
         core,
         bg_color,
         msaa: true,
         culling: true,
         msaa_samples: 4,
         size: cam.transform.size,
         clip: cam.transform.clip,
         cull_face: Cull::AntiClock,
         poly_mode: PolyMode::Filled,
         cam_view: cam.transform.view_matrix(),
         cam_proj: cam.transform.proj_matrix(),
         fallback_shader: NEShader::temporary(),
         fallback_texture: NETexture::temporary(),
      };
      let fallback_shader_asset = NEShaderAsset::fallback().unpack();
      let mut fallback_tex_asset = NETexAsset::fallback().unpack();
      fallback_tex_asset.set_filter(TexFilter::Closest);
      fallback_tex_asset.set_wrap(TexWrap::Repeat);

      renderer.fallback_texture = renderer.add_texture(fallback_tex_asset);
      renderer.fallback_shader = renderer.add_shader(fallback_shader_asset).unpack();

      renderer
         .fallback_shader
         .attach_tex(&renderer.fallback_texture);

      renderer.set_msaa(true);
      renderer.set_culling(true);
      renderer.set_wire_width(2.0);
      renderer.set_bg_color(bg_color);
      renderer.core.enable_alpha(true);
      renderer
   }
   pub(crate) fn set_size(&mut self, size: Size2D) {
      self.size = size;
      self.core.resize(self.size);
   }
   fn clear(&self) {
      self.core.clear()
   }

   pub(crate) fn pre_update(&mut self, cam: &NECamera) {
      self.cam_view = cam.transform.view_matrix();
      self.cam_proj = cam.transform.proj_matrix();
      self.clip = cam.clip();
      self.clear()
   }
   pub(crate) fn post_update(&self) {}
}
//PUBLIC
impl NERenderer {
   pub fn log_backend_info(&self) {
      self.core.log_info();
   }
   pub fn log_info(&self) {
      log_info!("RENDERER");
      log_info!(
         "> mode: {}",
         match self.poly_mode {
            PolyMode::Points => "POINTS",
            PolyMode::WireFrame => "WIREFRAME",
            PolyMode::Filled => "RASTERIZE",
         }
      );
      log_info!(
         "> cull: {}",
         if self.culling {
            let cull_face = if matches!(self.cull_face, Cull::Clock) {
               "clockwise"
            } else {
               "anti-clock"
            };
            format!("ON [{}]", cull_face)
         } else {
            "OFF".to_string()
         }
      );
      log_info!(
         "> msaa: {}\n",
         if self.msaa {
            format!("ON [{} samples]", self.msaa_samples)
         } else {
            "OFF".to_string()
         }
      );
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
      self.cull_face = cull_face;
      self.core.set_cull_face(self.cull_face)
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
   pub fn fallback_shader(&self) -> NEShader {
      self.fallback_shader.clone()
   }

   pub fn add_shader(&self, nshdr: NEShaderAsset) -> NEResult<NEShader> {
      let id = match self.core.create_spv_program(&nshdr) {
         NEResult::OK(id) => id,
         NEResult::ER(mut e) => {
            return match e {
               NEError::Renderer { kind, msg, .. } => NEError::Renderer {
                  kind,
                  path: nshdr.path,
                  msg,
               }
               .pack(),
               _ => e.pack(),
            }
         }
      };
      let mut shader = NEShader::temporary();
      shader.id = id;
      NEResult::OK(shader)
   }
   pub fn remove_shader(&self, shader: NEShader) {
      self.core.delete_shader(shader.id)
   }

   pub fn add_texture(&self, ntxtr: NETexAsset) -> NETexture {
      let id = self.core.create_texture(&ntxtr);

      NETexture {
         id,
         size: ntxtr.size,
         fmt: ntxtr.fmt,
         filter: ntxtr.filter,
         wrap: ntxtr.wrap,
      }
   }
   pub fn remove_texture(&self, tex: NETexture) {
      self.core.delete_texture(tex.id)
   }

   pub(crate) fn create_mesh_handle(&self, nmesh: &NEMeshAsset) -> MeshHandle {
      let (vao_id, buf_id) = self.core.create_buffer();
      let ind_id = self.core.create_index_buffer();

      let (mut pos_info, mut pos_data) = (&ATTRInfo::empty(), &Vec::new());
      let (mut col_info, mut col_data) = (&ATTRInfo::empty(), &Vec::new());
      let (mut uvm_info, mut uvm_data) = (&ATTRInfo::empty(), &Vec::new());
      let (mut nrm_info, mut nrm_data) = (&ATTRInfo::empty(), &Vec::new());
      let (mut ind_info, mut ind_data) = (&ATTRInfo::empty(), &Vec::new());

      let mut cus_infos: Vec<&ATTRInfo> = Vec::new();
      let mut cus_datas: Vec<&Vec<u8>> = Vec::new();

      let (mut ind_count, mut vert_count, mut stride) = (0, 0, 0);

      let mut pos_exists = !nmesh.pos_attr.is_empty();
      let mut col_exists = !nmesh.col_attr.is_empty();
      let mut uvm_exists = !nmesh.uvm_attr.is_empty();
      let mut nrm_exists = !nmesh.nrm_attr.is_empty();

      if pos_exists {
         pos_info = &nmesh.pos_attr.info;
         pos_data = &nmesh.pos_attr.data;
         stride += pos_info.elem_count * pos_info.byte_count;
      }
      if col_exists {
         col_info = &nmesh.col_attr.info;
         col_data = &nmesh.col_attr.data;
         stride += col_info.elem_count * col_info.byte_count;
      }
      if uvm_exists {
         uvm_info = &nmesh.uvm_attr.info;
         uvm_data = &nmesh.uvm_attr.data;
         stride += uvm_info.elem_count * uvm_info.byte_count;
      }
      if nrm_exists {
         nrm_info = &nmesh.nrm_attr.info;
         nrm_data = &nmesh.nrm_attr.data;
         stride += nrm_info.elem_count * nrm_info.byte_count;
      }
      for cus_attr in nmesh.cus_attrs.iter() {
         let cus_info = &cus_attr.info;
         let cus_data = &cus_attr.data;
         stride += cus_info.elem_count * cus_info.byte_count;
         cus_infos.push(cus_info);
         cus_datas.push(cus_data);
      }
      let mut end = pos_data.len();
      if nmesh.starts_with_custom() {
         end = cus_datas[0].len() / (cus_infos[0].byte_count * cus_infos[0].elem_count);
      }

      let mut buffer: Vec<u8> = Vec::new();
      for i in 0..end {
         vert_count += 1;
         if pos_exists {
            buffer.push_attr(&pos_data[i]);
         }
         if col_exists {
            buffer.push_attr(&col_data[i]);
         }
         if uvm_exists {
            buffer.push_attr(&uvm_data[i]);
         }
         if nrm_exists {
            buffer.push_attr(&nrm_data[i]);
         }

         for (j, _attr) in nmesh.cus_attrs.iter().enumerate() {
            let cus_byte_count = cus_infos[j].byte_count * cus_infos[j].elem_count;
            let cus_data = cus_datas[j];
            let start = i * cus_byte_count;
            let end = ((i + 1) * (cus_byte_count)) - 1;
            buffer.push_attr(&cus_data[start..=end]);
         }
      }

      let mut attr_id = 0;
      let mut local_offset = 0;
      self.core.bind_buffer(vao_id, buf_id);
      let mut layouts: Vec<(ATTRInfo, u32)> = Vec::new();
      if pos_exists {
         self.core.set_attr(&pos_info, attr_id, stride, local_offset);
         local_offset += pos_info.elem_count * pos_info.byte_count;
         layouts.push((pos_info.clone(), attr_id));
         attr_id += 1;
      }
      if col_exists {
         self.core.set_attr(&col_info, attr_id, stride, local_offset);
         local_offset += col_info.elem_count * col_info.byte_count;
         layouts.push((col_info.clone(), attr_id));
         attr_id += 1;
      }
      if uvm_exists {
         self.core.set_attr(&uvm_info, attr_id, stride, local_offset);
         local_offset += uvm_info.elem_count * uvm_info.byte_count;
         layouts.push((uvm_info.clone(), attr_id));
         attr_id += 1;
      }
      if nrm_exists {
         self.core.set_attr(&nrm_info, attr_id, stride, local_offset);
         local_offset += nrm_info.elem_count * nrm_info.byte_count;
         layouts.push((nrm_info.clone(), attr_id));
         attr_id += 1;
      }

      for cus_info in cus_infos.iter() {
         self.core.set_attr(cus_info, attr_id, stride, local_offset);
         local_offset += cus_info.elem_count * cus_info.byte_count;
         layouts.push((cus_info.deref().clone(), attr_id));
         attr_id += 1;
      }

      if buffer.len() > 0 {
         self.core.fill_buffer(vao_id, buf_id, &buffer);
      }
      self.core.unbind_buffer();

      let mut has_indices = false;
      let mut index_buffer: Vec<u32> = Vec::new();

      if !nmesh.ind_attr.is_empty() {
         ind_info = &nmesh.ind_attr.info;
         ind_data = &nmesh.ind_attr.data;
         has_indices = true;
         for index in ind_data.iter() {
            ind_count += 1;
            index_buffer.push(*index);
         }
         self.core.bind_index_buffer(ind_id);
         self.core.fill_index_buffer(ind_id, &index_buffer);
         self.core.unbind_index_buffer();
      }

      MeshHandle {
         layouts,
         has_indices,
         vert_count,
         ind_count,
         vao_id,
         buf_id,
         ind_id,
      }
   }

   pub fn create_mesh3d(&self, nmesh: NEMeshAsset) -> NEMesh3D {
      let handle = self.create_mesh_handle(&nmesh);
      NEMesh3D {
         handle,
         visible: true,
         shader: self.fallback_shader(),
         transform: Transform3D::default(),
         draw_mode: DrawMode::default(),
      }
   }
   pub fn remove_mesh3d(&self, mesh: NEMesh3D) {}

   pub fn render3d(&self, mesh: &mut NEMesh3D) {
      if !mesh.is_renderable() {
         return;
      }
      let s = mesh.shader.id;
      let handle = &mesh.handle;
      self.core.bind_program(s);
      self.core.set_uni_m4f32(s, "uCamView", self.cam_view);
      self.core.set_uni_m4f32(s, "uCamProj", self.cam_proj);
      self.core.set_uni_m4f32(s, "uMeshTfm", mesh.matrix());

      for (slot, tex_id) in mesh.shader.tex_ids.iter().enumerate() {
         match tex_id {
            None => {}
            Some(id) => {
               self.core.bind_texture_at(*id, slot as u32);
            }
         }
      }
      self.core.bind_buffer(handle.vao_id, handle.buf_id);
      match handle.has_indices {
         false => self.core.draw_array(&mesh.draw_mode, handle.vert_count),
         true => {
            self.core.bind_index_buffer(handle.ind_id);
            self.core.draw_indexed(&mesh.draw_mode, handle.ind_count);
            self.core.unbind_index_buffer()
         }
      }
      self.core.unbind_buffer();
   }
   pub fn render2d(&self, mesh: &mut NEMesh3D) {
      if !mesh.is_renderable() {
         return;
      }
      let s = mesh.shader.id;
      let handle = &mesh.handle;
      self.core.bind_program(s);

      let aspect = self.size.aspect_ratio();

      //self.core.set_uni_vec2f32(s, "uAspect", vec2(aspect, 1.0));
      self.core.set_uni_m4f32(s, "uTfm", mesh.matrix());

      for (slot, tex_id) in mesh.shader.tex_ids.iter().enumerate() {
         match tex_id {
            None => {}
            Some(id) => {
               self.core.bind_texture_at(*id, slot as u32);
            }
         }
      }
      self.core.bind_buffer(handle.vao_id, handle.buf_id);
      match handle.has_indices {
         false => self.core.draw_array(&mesh.draw_mode, handle.vert_count),
         true => {
            self.core.bind_index_buffer(handle.ind_id);
            self.core.draw_indexed(&mesh.draw_mode, handle.ind_count);
         }
      }
   }
}

trait Buffer {
   fn push_attr<T: DataType>(&mut self, attr: &[T]);
}

impl Buffer for Vec<u8> {
   fn push_attr<T: DataType>(&mut self, attr: &[T]) {
      for elem in attr.iter() {
         let bytes = elem.u8ify();
         for byte in bytes.iter() {
            self.push(*byte);
         }
      }
   }
}
