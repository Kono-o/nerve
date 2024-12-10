use std::mem::size_of;
use gl::types::{GLfloat, GLsizei};
use crate::*;
use crate::renderer::render::mesh::glbuffers::{GLIndices, GLVerts};

pub struct PositionAttr(pub AttrData<Float32x3>);
pub struct ColorAttr(pub AttrData<Float32x3>);
pub struct UVMapAttr(pub AttrData<Float32x2>);
pub struct Indices(pub AttrData<Int32>);

pub struct CustomAttr<T: DataFormat>(pub AttrData<T>);

pub struct NerveMesher {
   pub shader: NerveShader,
   pub transform: Transform,
   pub pos_attr: PositionAttr, //Float32x3
   pub color_attr: ColorAttr,  //Float32x3
   pub uv_attr: UVMapAttr,     //Int32
   pub custom_attr_i32: CustomAttr<Int32>,
   pub custom_attr_i8x2: CustomAttr<Int8x2>,
   pub indices: Indices,
}
impl Default for NerveMesher {
   fn default() -> Self {
      NerveMesher {
         shader: NerveShader { program_id: 0 },
         transform: Default::default(),
         pos_attr: PositionAttr(AttrData::Empty),
         color_attr: ColorAttr(AttrData::Empty),
         uv_attr: UVMapAttr(AttrData::Empty),
         custom_attr_i32: CustomAttr(AttrData::Empty),
         custom_attr_i8x2: CustomAttr(AttrData::Empty),
         indices: Indices(AttrData::Empty),
      }
   }
}

impl NerveMesher {
   pub fn build(&mut self) -> NerveMesh {
      let mut gl_vert_obj = GLVerts::new();
      let mut gl_indices_obj = GLIndices::new();

      let mut pos_data = self.pos_attr.0.got_data();
      let mut col_data = self.color_attr.0.got_data();
      let mut uvm_data = self.uv_attr.0.got_data();
      let mut ind_data = self.indices.0.got_data();

      let mut pcu_vec = Vec::new();

      let mut vert_count: u32 = 0;
      let mut ind_count = 0;
      let mut has_indices = false;
      let mut stride = 0;

      if pos_data.is_some() {
         let pos_data = pos_data.unwrap();
         stride += 3;
         let (mut col_exists, mut uvm_exists) = (false, false);
         let mut col_vec = &Vec::new();
         let mut uvm_vec = &Vec::new();
         if col_data.is_some() {
            col_exists = true;
            col_vec = col_data.unwrap();
            stride += 3;
         }
         if uvm_data.is_some() {
            uvm_exists = true;
            uvm_vec = uvm_data.unwrap();
            stride += 2;
         }
         for (i, pos) in pos_data.iter().enumerate() {
            vert_count += 1;
            pcu_vec.push(pos.0);
            pcu_vec.push(pos.1);
            pcu_vec.push(pos.2);
            if col_exists {
               pcu_vec.push(col_vec[i].0);
               pcu_vec.push(col_vec[i].1);
               pcu_vec.push(col_vec[i].2);
            }
            if uvm_exists {
               pcu_vec.push(uvm_vec[i].0);
               pcu_vec.push(uvm_vec[i].1);
            }
         }
         gl_vert_obj.bind();
         gl_vert_obj.fill(&pcu_vec);
         stride = stride * size_of::<GLfloat>() as GLsizei;
         //POS
         gl_vert_obj.gen_ptr(3, gl::FLOAT, stride);
         //COL
         if col_exists {
            gl_vert_obj.gen_ptr(3, gl::FLOAT, stride);
         }
         //UVM
         if uvm_exists {
            gl_vert_obj.gen_ptr(2, gl::FLOAT, stride);
         }
         gl_vert_obj.unbind();

         if ind_data.is_some() {
            has_indices = true;
            let ind_data = ind_data.unwrap();

            let mut ind_vec = Vec::new();
            for (i, index) in ind_data.iter().enumerate() {
               ind_count += 1;
               ind_vec.push(*index);
            }
            gl_indices_obj.bind();
            gl_indices_obj.fill(&ind_vec);
            gl_indices_obj.unbind();
         }
      }
      self.transform.calc_matrix();
      NerveMesh {
         shader: self.shader,
         has_indices,
         vert_count,
         ind_count,
         vert_object: gl_vert_obj,
         index_object: gl_indices_obj,
         transform: self.transform.clone(),
         ..Default::default()
      }
   }
}
