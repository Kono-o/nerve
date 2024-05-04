use gl::types::*;
use crate::{NerveCanvas, NerveShader};
use crate::renderer::Transform;

#[derive(Clone)]
pub struct NerveMesh {
   pub visible: bool,
   pub(crate) alive: bool,
   pub(crate) shader: NerveShader,
   pub transform: Transform,

   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) indices_count: u32,
   pub(crate) vao_id: GLuint,
   pub(crate) vbo_id: GLuint,
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
         indices_count: 0,
         vao_id: 0,
         vbo_id: 0,
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

      unsafe {
         gl::BindVertexArray(self.vao_id);
         if self.has_indices {
            gl::DrawElements(
               gl::TRIANGLES,
               self.indices_count as GLsizei,
               gl::UNSIGNED_INT,
               std::ptr::null(),
            );
         } else {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vert_count as GLsizei);
         }
      }
   }

   pub fn kill(&mut self) {
      self.alive = false;
      unsafe {
         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
         gl::BindVertexArray(0);
         gl::DeleteVertexArrays(1, &self.vao_id);
         gl::DeleteBuffers(1, &self.vbo_id);
      }
   }
}
