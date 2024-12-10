use gl::types::*;
use crate::{NerveCanvas, NerveShader};
use crate::renderer::render::mesh::glbuffers::{GLIndices, GLVerts};
use crate::renderer::Transform;

pub struct NerveMesh {
   pub visible: bool,
   pub transform: Transform,

   pub(crate) alive: bool,
   pub(crate) shader: NerveShader,
   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) ind_count: u32,
   pub(crate) vert_object: GLVerts,
   pub(crate) index_object: GLIndices,
}

impl Default for NerveMesh {
   fn default() -> Self {
      Self {
         visible: true,
         alive: true,
         shader: NerveShader { program_id: 0 },
         transform: Transform::default(),
         has_indices: false,
         vert_count: 0,
         ind_count: 0,
         vert_object: GLVerts::new(),
         index_object: GLIndices::new(),
      }
   }
}
impl NerveMesh {
   pub fn draw_to(&mut self, canvas: &NerveCanvas) {
      if !self.visible || !self.alive {
         return;
      }
      self.transform.calc_matrix();
      self.shader.set();

      self
         .shader
         .set_mat4("u_MeshTransform", self.transform.matrix);
      self.shader.set_mat4("u_CamView", canvas.cam.view_matrix);
      self.shader.set_mat4("u_CamProj", canvas.cam.proj_matrix);

      self.vert_object.bind();
      self.index_object.bind();
      unsafe {
         if self.has_indices {
            gl::DrawElements(
               gl::TRIANGLES,
               self.ind_count as GLsizei,
               gl::UNSIGNED_INT,
               std::ptr::null(),
            );
         } else {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vert_count as GLsizei);
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
         alive: self.alive,
         shader: self.shader,
         has_indices: self.has_indices,
         vert_count: self.vert_count,
         ind_count: self.ind_count,
         vert_object: GLVerts {
            vao: self.vert_object.vao,
            vbo: self.vert_object.vbo,
            attrib_id: self.vert_object.attrib_id,
            local_offset: self.vert_object.local_offset,
         },
         index_object: GLIndices {
            ebo: self.index_object.ebo,
         },
      }
   }

   pub fn kill(&mut self) {
      self.alive = false;
      unsafe {
         self.vert_object.delete();
         self.index_object.delete();
      }
   }
}
