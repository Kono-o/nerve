use std::ffi::c_void;
use std::mem::size_of;
use gl::types::*;
use crate::NerveShader;

pub struct NerveMesh {
   pub(crate) shader: NerveShader,
   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) indices_count: u32,
   pub(crate) vao_id: GLuint,
   pub(crate) vbo_id: GLuint,
}

impl NerveMesh {
   pub fn draw(&self) {
      unsafe {
         self.shader.set();
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

   pub fn kill(&self) {
      self.shader.kill();
      unsafe {
         gl::DeleteVertexArrays(1, self.vao_id as *const GLuint);
         gl::DeleteBuffers(1, self.vbo_id as *const GLuint);
      }
   }
}
