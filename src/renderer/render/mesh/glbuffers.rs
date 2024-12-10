use std::ffi::c_void;
use gl::types::{GLenum, GLfloat, GLint, GLsizeiptr, GLuint};

pub(crate) struct GLVerts {
   pub(crate) vao: GLuint,
   pub(crate) vbo: GLuint,
   pub(crate) attrib_id: u32,
   pub(crate) local_offset: usize,
}

impl GLVerts {
   pub(crate) fn new() -> GLVerts {
      let (mut vao, mut vbo) = (0, 0);
      unsafe {
         gl::GenVertexArrays(1, &mut vao);
         gl::GenBuffers(1, &mut vbo);
      }
      GLVerts {
         vao,
         vbo,
         attrib_id: 0,
         local_offset: 0,
      }
   }
   pub(crate) fn bind(&self) {
      unsafe {
         gl::BindVertexArray(self.vao);
         gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
      }
   }
   pub(crate) fn unbind(&self) {
      unsafe {
         gl::BindVertexArray(0);
         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      }
   }
   pub(crate) fn fill(&mut self, data: &Vec<f32>) {
      self.bind();
      let data_len = data.len();
      if data_len > 0 {
         unsafe {
            gl::BufferData(
               gl::ARRAY_BUFFER,
               (data_len * size_of::<GLfloat>()) as GLsizeiptr,
               &data[0] as *const f32 as *const c_void,
               gl::DYNAMIC_DRAW,
            );
         }
      }
   }
   pub(crate) fn gen_ptr(&mut self, size: usize, data_type: GLenum, stride: i32) {
      unsafe {
         gl::VertexAttribPointer(
            self.attrib_id,
            size as GLint,
            data_type,
            gl::FALSE,
            stride,
            match self.local_offset {
               0 => std::ptr::null(),
               _ => self.local_offset as *const c_void,
            },
         );

         self.local_offset = self.local_offset + (size * size_of::<GLfloat>());
         gl::EnableVertexAttribArray(self.attrib_id);
         self.attrib_id += 1;
      }
   }
   pub(crate) fn delete(&mut self) {
      unsafe {
         gl::DeleteVertexArrays(1, &self.vao);
         gl::DeleteBuffers(1, &self.vbo);
      }
   }
}

pub(crate) struct GLIndices {
   pub(crate) ebo: GLuint,
}

impl GLIndices {
   pub(crate) fn new() -> GLIndices {
      let mut ebo = 0;
      unsafe {
         gl::GenBuffers(1, &mut ebo);
      }
      GLIndices { ebo }
   }

   pub(crate) fn bind(&self) {
      unsafe {
         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
      }
   }
   pub(crate) fn unbind(&self) {
      unsafe {
         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
         gl::BindVertexArray(0);
      }
   }

   pub(crate) fn fill(&mut self, data: &Vec<i32>) {
      let data_len = data.len();
      if data_len > 0 {
         unsafe {
            gl::BufferData(
               gl::ELEMENT_ARRAY_BUFFER,
               (data_len * size_of::<GLint>()) as GLsizeiptr,
               &data[0] as *const i32 as *const c_void,
               gl::DYNAMIC_DRAW,
            );
         }
      }
   }
   pub(crate) fn delete(&mut self) {
      unsafe {
         gl::DeleteBuffers(1, &self.ebo);
      }
   }
}
