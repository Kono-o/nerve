use crate::renderer::render::mesh::attr::AttrData;
use crate::renderer::render::mesh::glbuffers::{GLIndices, GLVerts};
use crate::*;
use std::fmt::Debug;

pub struct NerveMesher {
   pub shader: NerveShader,
   pub transform: Transform,
   pub pos_attr: PositionAttr,
   pub col_attr: ColorAttr,
   pub uvm_attr: UVMapAttr,
   pub nrm_attr: NormalAttr,
   pub indices: Indices,
   pub cus_attrs: Vec<Vec<u8>>,
}
impl Default for NerveMesher {
   fn default() -> Self {
      NerveMesher {
         shader: NerveShader { program_id: 0 },
         transform: Default::default(),
         pos_attr: PositionAttr(AttrData::Empty),
         col_attr: ColorAttr(AttrData::Empty),
         uvm_attr: UVMapAttr(AttrData::Empty),
         nrm_attr: NormalAttr(AttrData::Empty),
         indices: Indices(AttrData::Empty),
         cus_attrs: vec![],
      }
   }
}

fn push_array(vec: &mut Vec<u8>, array: &[u8]) {
   for elem in array.iter() {
      vec.push(*elem);
   }
}

fn shove_f32x3(mut vec: &mut Vec<u8>, elem: (f32, f32, f32), size: usize) {
   let elem1 = elem.0.to_ne_bytes();
   let elem2 = elem.1.to_ne_bytes();
   let elem3 = elem.2.to_ne_bytes();
   push_array(&mut vec, &elem1[0..=size]);
   push_array(&mut vec, &elem2[0..=size]);
   push_array(&mut vec, &elem3[0..=size]);
}

fn shove_f32x2(mut vec: &mut Vec<u8>, elem: (f32, f32), size: usize) {
   let elem1 = elem.0.to_ne_bytes();
   let elem2 = elem.1.to_ne_bytes();
   push_array(&mut vec, &elem1[0..=size]);
   push_array(&mut vec, &elem2[0..=size]);
}

impl NerveMesher {
   pub fn build<T: DataFormat + 'static + Debug>(&mut self) -> NerveMesh {
      if self.pos_attr.has_data() {
         let mut gl_verts_obj = GLVerts::new();
         let mut gl_index_obj = GLIndices::new();
         self.transform.calc_matrix();
         let mut stride = 0;

         let mut col_data = &Vec::new();
         let mut col_type = gl::FLOAT;
         let mut col_exists = false;
         let mut col_bytes = 0;
         let mut col_elems = 0;

         let mut uvm_data = &Vec::new();
         let mut uvm_type = gl::FLOAT;
         let mut uvm_exists = false;
         let mut uvm_bytes = 0;
         let mut uvm_elems = 0;

         let mut nrm_data = &Vec::new();
         let mut nrm_type = gl::FLOAT;
         let mut nrm_exists = false;
         let mut nrm_bytes = 0;
         let mut nrm_elems = 0;

         let mut ind_data = &Vec::new();
         let mut ind_exists = true;
         let mut ind_count = 0;
         let mut vert_count = 0;

         let (pos_data, pos_type, pos_bytes, pos_elems) = self.pos_attr.data();
         stride += pos_elems * pos_bytes;

         if self.col_attr.has_data() {
            (col_data, col_type, col_bytes, col_elems) = self.col_attr.data();
            stride += col_elems * col_bytes;
            col_exists = true;
         }
         if self.uvm_attr.has_data() {
            (uvm_data, uvm_type, uvm_bytes, uvm_elems) = self.uvm_attr.data();
            stride += uvm_elems * uvm_bytes;
            uvm_exists = true;
         }
         if self.nrm_attr.has_data() {
            (nrm_data, nrm_type, nrm_bytes, nrm_elems) = self.nrm_attr.data();
            stride += nrm_elems * nrm_bytes;
            nrm_exists = true;
         }

         let mut buffer_data: Vec<u8> = Vec::new();
         for (i, _pos) in pos_data.iter().enumerate() {
            vert_count += 1;
            shove_f32x3(&mut buffer_data, pos_data[i], pos_elems);
            if col_exists {
               shove_f32x3(&mut buffer_data, col_data[i], col_elems);
            }
            if uvm_exists {
               shove_f32x2(&mut buffer_data, uvm_data[i], uvm_elems);
            }
            if nrm_exists {
               shove_f32x3(&mut buffer_data, nrm_data[i], nrm_elems);
            }
         }
         gl_verts_obj.bind();
         gl_verts_obj.fill(&buffer_data);

         gl_verts_obj.gen_ptr(pos_elems, pos_type, stride);
         if col_exists {
            gl_verts_obj.gen_ptr(col_elems, col_type, stride);
         }
         if uvm_exists {
            gl_verts_obj.gen_ptr(uvm_elems, uvm_type, stride);
         }
         if nrm_exists {
            gl_verts_obj.gen_ptr(nrm_elems, nrm_type, stride);
         }
         gl_verts_obj.unbind();

         let mut ind_buffer_data = Vec::new();
         if self.indices.has_data() {
            (ind_data, _, _, _) = self.indices.data();
            ind_exists = true;
            for index in ind_data.iter() {
               ind_count += 1;
               ind_buffer_data.push(*index);
            }
            gl_index_obj.bind();
            gl_index_obj.fill(&ind_buffer_data);
            gl_index_obj.unbind();
         }
         NerveMesh {
            shader: self.shader,
            has_indices: ind_exists,
            vert_count,
            ind_count,
            is_empty: false,
            vert_object: gl_verts_obj,
            index_object: gl_index_obj,
            transform: self.transform.clone(),
            ..Default::default()
         }
      } else {
         NerveMesh {
            shader: self.shader,
            has_indices: false,
            vert_count: 0,
            ind_count: 0,
            is_empty: true,
            vert_object: GLVerts {
               vao: 0,
               vbo: 0,
               attrib_id: 0,
               local_offset: 0,
            },
            index_object: GLIndices { ebo: 0 },
            transform: self.transform.clone(),
            ..Default::default()
         }
      }
   }
}
