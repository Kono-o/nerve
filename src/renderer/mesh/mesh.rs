use crate::render::mesh::glbuffers::{GLIndices, GLVerts};
use crate::render::Transform;
use crate::{NerveGame, NerveShader, Uniform};
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
         shader: NerveShader::default(),
         draw_mode: DrawMode::Triangles,
         layouts: vec![],
      }
   }
}
impl NerveMesh {
   pub fn draw_to(&mut self, game: &NerveGame) {
      if !self.visible || !self.alive {
         return;
      }
      self.transform.calc_matrix();

      self.shader.bind();
      self
         .shader
         .set_uniform("u_MeshTransform", Uniform::Matrix4(self.transform.matrix));
      self
         .shader
         .set_uniform("u_CamView", Uniform::Matrix4(game.cam.view_matrix));
      self
         .shader
         .set_uniform("u_CamProj", Uniform::Matrix4(game.cam.proj_matrix));

      if !self.is_empty {
         self.vert_object.bind();
         self.index_object.bind();
         unsafe {
            if self.has_indices {
               gl::DrawElements(
                  self.draw_mode.gl_enum(),
                  self.ind_count as GLsizei,
                  gl::UNSIGNED_INT,
                  std::ptr::null(),
               );
            } else {
               gl::DrawArrays(self.draw_mode.gl_enum(), 0, self.vert_count as GLsizei);
            }
         }
      }
   }
   pub fn set_shader(&mut self, shader: NerveShader) {
      self.shader = shader
   }

   pub fn mimic(&mut self) -> NerveMesh {
      NerveMesh {
         visible: self.visible,
         transform: self.transform.clone(),
         draw_mode: DrawMode::Triangles,
         alive: self.alive,
         shader: self.shader.clone(),
         has_indices: self.has_indices,
         is_empty: self.is_empty,
         vert_count: self.vert_count,
         ind_count: self.ind_count,
         vert_object: GLVerts {
            vao: self.vert_object.vao,
            vbo: self.vert_object.vbo,
            attrib_id: self.vert_object.attrib_id,
            local_offset: self.vert_object.local_offset,
            stride: self.vert_object.stride,
            buffer: self.vert_object.buffer.clone(),
         },
         index_object: GLIndices {
            ebo: self.index_object.ebo,
            buffer: vec![],
         },
         layouts: self.layouts.clone(),
      }
   }

   pub fn show_layouts(&self) {
      for attr in self.layouts.clone() {
         println!("{}", attr);
      }
      for texture in self.shader.image_ids.clone() {
         println!("{}(tex): {}", texture.0, texture.1);
      }
   }
   pub fn kill(&mut self) {
      self.alive = false;
      self.vert_object.delete();
      self.index_object.delete();
   }
}
