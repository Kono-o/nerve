use crate::render::AttrInfo;
use crate::DataFormat;
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use std::ffi::c_void;

pub(crate) struct GLVerts {
   pub(crate) vao: GLuint,
   pub(crate) vbo: GLuint,
   pub(crate) attrib_id: u32,
   pub(crate) local_offset: usize,
   pub(crate) stride: usize,
   pub(crate) buffer: Vec<u8>,
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
         stride: 0,
         buffer: vec![],
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

   pub(crate) fn push<T: DataFormat>(&mut self, attr: &[T]) {
      for elem in attr.iter() {
         let bytes = elem.u8ify();
         for byte in bytes.iter() {
            self.buffer.push(*byte)
         }
      }
   }

   pub(crate) fn ship(&mut self) {
      let buffer_len = self.buffer.len();
      if buffer_len > 0 {
         unsafe {
            gl::BufferData(
               gl::ARRAY_BUFFER,
               (buffer_len * 4) as GLsizeiptr,
               &self.buffer[0] as *const u8 as *const c_void,
               gl::DYNAMIC_DRAW,
            );
            self.unbind()
         }
      }
   }
   pub(crate) fn layout(&mut self, info: AttrInfo) -> Option<u32> {
      if info.exists {
         unsafe {
            gl::VertexAttribPointer(
               self.attrib_id,
               info.elem_count as GLint,
               info.typ,
               gl::FALSE,
               self.stride as GLsizei,
               match self.local_offset {
                  0 => std::ptr::null(),
                  _ => self.local_offset as *const c_void,
               },
            );

            self.local_offset = self.local_offset + (info.elem_count * size_of::<GLfloat>());
            gl::EnableVertexAttribArray(self.attrib_id);
         }
         self.attrib_id += 1;
         Some(self.attrib_id - 1)
      } else {
         None
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
   pub(crate) buffer: Vec<u32>,
}

impl GLIndices {
   pub(crate) fn new() -> GLIndices {
      let mut ebo = 0;
      unsafe {
         gl::GenBuffers(1, &mut ebo);
      }
      GLIndices {
         ebo,
         buffer: vec![],
      }
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
   pub(crate) fn push(&mut self, index: u32) {
      self.buffer.push(index)
   }
   pub(crate) fn ship(&mut self) {
      let buffer_len = self.buffer.len();
      if buffer_len > 0 {
         unsafe {
            gl::BufferData(
               gl::ELEMENT_ARRAY_BUFFER,
               (buffer_len * size_of::<GLint>()) as GLsizeiptr,
               &self.buffer[0] as *const u32 as *const c_void,
               gl::DYNAMIC_DRAW,
            );
            self.unbind()
         }
      }
   }
   pub(crate) fn delete(&mut self) {
      unsafe {
         gl::DeleteBuffers(1, &self.ebo);
      }
   }
}
