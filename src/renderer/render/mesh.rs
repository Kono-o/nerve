use gl::types::*;
use crate::NerveMaterial;

pub struct NerveMesh {
   material: NerveMaterial,
   vao: GLuint,
   vbo: GLuint,
}

impl NerveMesh {
   pub fn new(data: &[f32], material: NerveMaterial) -> NerveMesh {
      let mut vbo = 0;
      let mut vao = 0;

      unsafe {
         gl::GenBuffers(1, &mut vbo);
         gl::GenVertexArrays(1, &mut vao);

         gl::BindVertexArray(vao);
         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
         gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &data[0] as *const f32 as *const std::ffi::c_void,
            gl::DYNAMIC_DRAW,
         );

         gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
         );
         gl::EnableVertexAttribArray(0);
         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
         gl::BindVertexArray(0);
         NerveMesh { material, vao, vbo }
      }
   }

   pub fn draw(&self) {
      unsafe {
         self.material.set();
         gl::BindVertexArray(self.vao);
         gl::DrawArrays(gl::TRIANGLES, 0, 3);
      }
   }
}
