use std::ffi::c_void;
use std::mem::size_of;
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr};
use crate::{NerveMesh, NerveShader};

pub enum DataType {
   Uint8x2(u8, u8),
   Uint8x4(u8, u8, u8, u8),

   Int8x2(i8, i8),
   Int8x4(i8, i8, i8, i8),
   //---------------------
   Uint32(u32),
   Uint32x2(u32, u32),
   Uint32x3(u32, u32, u32),
   Uint32x4(u32, u32, u32, u32),

   Int32(i32),
   Int32x2(i32, i32),
   Int32x3(i32, i32, i32),
   Int32x4(i32, i32, i32, i32),
   //---------------------
   Float32(f32),
   Float32x2(f32, f32),
   Float32x3(f32, f32, f32),
   Float32x4(f32, f32, f32, f32),

   Float64(f64),
   Float64x2(f64, f64),
   Float64x3(f64, f64, f64),
   Float64x4(f64, f64, f64, f64),
}

pub enum Data<T> {
   Empty,
   Vec(Vec<T>),
}

pub struct PositionAttr(pub Data<(f32, f32, f32)>);
pub struct ColorAttr(pub Data<(f32, f32, f32)>);
pub struct UVMapAttr(pub Data<(f32, f32)>);
pub struct Indices(pub Data<i32>);

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

         match &self.indices {
            Indices(Data::Vec(vec)) => unsafe {
               has_indices = true;
               let mut indices: [i32; 500] = [0; 500];

               let mut count = 0;

               for index in vec {
                  indices[count.clone()] = index.clone();
                  count += 1;
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
         gl::BindVertexArray(vao);
         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      }
      let mut pos_vec = &Vec::new();
      let mut col_vec = &Vec::new();
      let mut uvm_vec = &Vec::new();

      let mut pos_exists = false;
      let mut col_exists = false;
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

      let mut buffer_vec: [f32; 500] = [0.0; 500];

      let mut vert_count: u32 = 0;
      let mut count = 0;
      for (i, pos) in pos_vec.iter().enumerate() {
         vert_count += 1;
         buffer_vec[count] = pos.0;
         count += 1;
         buffer_vec[count] = pos.1;
         count += 1;
         buffer_vec[count] = pos.2;
         count += 1;
         if col_exists {
            buffer_vec[count] = col_vec[i].0;
            count += 1;
            buffer_vec[count] = col_vec[i].1;
            count += 1;
            buffer_vec[count] = col_vec[i].2;
            count += 1;
         }
         if uvm_exists {
            buffer_vec[count] = uvm_vec[i].0;
            count += 1;
            buffer_vec[count] = uvm_vec[i].1;
            count += 1;
         }
      }

      unsafe {
         gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer_vec.len() * size_of::<GLfloat>()) as GLsizeiptr,
            &buffer_vec[0] as *const f32 as *const c_void,
            gl::DYNAMIC_DRAW,
         )
      }

      let stride = stride * size_of::<GLfloat>() as GLsizei;
      unsafe {
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
            println!("id: {}", attr_id);
            println!("off: {}", attr_offset);
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
            println!("id: {}", attr_id);
            println!("off: {}", attr_offset);
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
            println!("id: {}", attr_id);
            println!("off: {}", attr_offset);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
         }
      }
      println!("stride: {}", stride);

      NerveMesh {
         shader: self.shader,
         has_indices,
         vert_count,
         vao_id: vao,
         vbo_id: vbo,
      }
   }
}
