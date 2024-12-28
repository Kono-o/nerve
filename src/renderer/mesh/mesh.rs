use crate::renderer::mesh::glbuffers::{GLIndices, GLVerts};
use crate::{NerveShader, Transform};
use cgmath::Matrix4;
use gl::types::*;

pub enum DrawMode {
   Points,
   Lines,
   Triangles,
   Strip,
}

impl DrawMode {
   pub(crate) fn gl_enum(&self) -> GLenum {
      match self {
         DrawMode::Points => gl::POINTS,
         DrawMode::Lines => gl::LINES,
         DrawMode::Triangles => gl::TRIANGLES,
         DrawMode::Strip => gl::TRIANGLE_STRIP,
      }
   }
}

pub struct NerveMesh {
   pub visible: bool,
   pub transform: Transform,
   pub draw_mode: DrawMode,

   pub(crate) alive: bool,
   pub(crate) has_indices: bool,
   pub(crate) is_empty: bool,
   pub(crate) vert_count: u32,
   pub(crate) ind_count: u32,
   pub(crate) vert_object: GLVerts,
   pub(crate) index_object: GLIndices,
   pub(crate) shader: NerveShader,
   pub(crate) layouts: Vec<String>,
}

impl Default for NerveMesh {
   fn default() -> Self {
      Self {
         visible: true,
         alive: true,
         transform: Transform::default(),
         has_indices: false,
         is_empty: true,
         vert_count: 0,
         ind_count: 0,
         vert_object: GLVerts {
            vao: 0,
            vbo: 0,
            attrib_id: 0,
            local_offset: 0,
            stride: 0,
            buffer: vec![],
         },
         index_object: GLIndices {
            ebo: 0,
            buffer: vec![],
         },
         shader: NerveShader::empty(),
         draw_mode: DrawMode::Triangles,
         layouts: vec![],
      }
   }
}
impl NerveMesh {
   pub fn set_shader(&mut self, shader: NerveShader) {
      self.shader = shader
   }
   pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
      self.draw_mode = draw_mode
   }
   pub fn set_visibility(&mut self, enable: bool) {
      self.visible = enable;
   }
   pub fn toggle_visibility(&mut self) {
      self.visible = !self.visible;
   }
   pub(crate) fn matrix(&self) -> Matrix4<f32> {
      self.transform.matrix
   }
   pub(crate) fn update(&mut self) {
      self.transform.calc_matrix()
   }
   pub fn display_layouts(&self) {
      for attr in self.layouts.clone() {
         println!("{}", attr);
      }
   }
   pub fn kill(&mut self) {
      self.alive = false;
      self.vert_object.delete();
      self.index_object.delete();
   }
}
