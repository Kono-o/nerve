use std::ffi::c_void;
use std::mem::size_of;
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr};
use crate::{Data, DataFormat, Float32x2, Float32x3, Int32, NerveMesh, NerveShader};

pub struct PositionAttr(pub Data<Float32x3>);
pub struct ColorAttr(pub Data<Float32x3>);
pub struct UVMapAttr(pub Data<Float32x2>);
pub struct Indices(pub Data<Int32>);

pub struct CustomAttr<T: DataFormat>(pub Data<T>);

pub struct NerveMesher {
   pub shader: NerveShader,
   pub pos_attr: PositionAttr,
   pub color_attr: ColorAttr,
   pub uv_attr: UVMapAttr,
   pub indices: Indices,
}

impl NerveMesher {
   pub fn build(&self) -> NerveMesh {
      let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
      let mut has_indices = false;

      unsafe {
         gl::GenVertexArrays(1, &mut vao);
         gl::GenBuffers(1, &mut vbo);

         gl::BindVertexArray(vao);
         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      }

      let mut pos_vec = &Vec::new();
      let mut pos_exists = false;
      let mut col_vec = &Vec::new();
      let mut col_exists = false;
      let mut uvm_vec = &Vec::new();
      let mut uvm_exists = false;

      let mut stride = 0;
      match &self.pos_attr {
         PositionAttr(Data::Vec(v)) => {
            pos_exists = true;
            stride += 3;
            pos_vec = v
         }
         _ => {}
      }
      match &self.color_attr {
         ColorAttr(Data::Vec(v)) => {
            col_exists = true;
            stride += 3;
            col_vec = v
         }
         _ => {}
      }
      match &self.uv_attr {
         UVMapAttr(Data::Vec(v)) => {
            uvm_exists = true;
            stride += 3;
            uvm_vec = v
         }
         _ => {}
      }

      let mut buffer_vec = Vec::new();
      let mut vert_count: u32 = 0;
      let mut indices_count = 0;
      for (i, pos) in pos_vec.iter().enumerate() {
         vert_count += 1;
         buffer_vec.push(pos.0);
         buffer_vec.push(pos.1);
         buffer_vec.push(pos.2);
         if col_exists {
            buffer_vec.push(col_vec[i].0);
            buffer_vec.push(col_vec[i].1);
            buffer_vec.push(col_vec[i].2);
         }
         if uvm_exists {
            buffer_vec.push(uvm_vec[i].0);
            buffer_vec.push(uvm_vec[i].1);
         }
      }

      unsafe {
         gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer_vec.len() * size_of::<GLfloat>()) as GLsizeiptr,
            &buffer_vec[0] as *const f32 as *const c_void,
            gl::DYNAMIC_DRAW,
         );

         let stride = stride * size_of::<GLfloat>() as GLsizei;
         let mut attr_id = 0;
         let mut attr_offset = 0;
         let mut nullptr = std::ptr::null();

         if pos_exists {
            gl::VertexAttribPointer(
               attr_id,
               3,
               gl::FLOAT,
               gl::FALSE,
               stride,
               match attr_offset {
                  0 => nullptr,
                  _ => attr_offset as *const c_void,
               },
            );
            gl::EnableVertexAttribArray(attr_id);
            attr_id += 1;
            attr_offset = 3 * size_of::<GLfloat>();
         }
         if col_exists {
            gl::VertexAttribPointer(
               attr_id,
               3,
               gl::FLOAT,
               gl::FALSE,
               stride,
               match attr_offset {
                  0 => nullptr,
                  _ => attr_offset as *const c_void,
               },
            );
            gl::EnableVertexAttribArray(attr_id);
            attr_id += 1;
            attr_offset = attr_offset + (3 * size_of::<GLfloat>());
         }

         if uvm_exists {
            gl::VertexAttribPointer(
               attr_id,
               2,
               gl::FLOAT,
               gl::FALSE,
               stride,
               match attr_offset {
                  0 => nullptr,
                  _ => attr_offset as *const c_void,
               },
            );
            gl::EnableVertexAttribArray(attr_id);
            attr_id += 1;
            attr_offset = attr_offset + (2 * size_of::<GLfloat>());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
         }

         match &self.indices {
            Indices(Data::Vec(vec)) => unsafe {
               has_indices = true;

               let mut indices: [i32; 500] = [0; 500];

               for (i, index) in vec.iter().enumerate() {
                  indices[i] = *index;
                  indices_count += 1;
               }

               gl::GenBuffers(1, &mut ebo);
               gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
               gl::BufferData(
                  gl::ELEMENT_ARRAY_BUFFER,
                  (indices.len() * size_of::<GLint>()) as GLsizeiptr,
                  &indices[0] as *const i32 as *const c_void,
                  gl::DYNAMIC_DRAW,
               );
            },
            _ => {}
         }
         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
         gl::BindVertexArray(0);
      }

      NerveMesh {
         shader: self.shader,
         has_indices,
         vert_count,
         indices_count,
         vao_id: vao,
         vbo_id: vbo,
      }
   }
}
