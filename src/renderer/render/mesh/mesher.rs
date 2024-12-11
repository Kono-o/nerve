use crate::renderer::render::mesh::attr::{get_format, AttrData};
use crate::renderer::render::mesh::glbuffers::{GLIndices, GLVerts};
use crate::*;

pub struct NerveMesher {
   pub shader: NerveShader,
   pub transform: Transform,
   pub pos_attr: PositionAttr,
   pub col_attr: ColorAttr,
   pub uvm_attr: UVMapAttr,
   pub indices: Indices,
}
impl Default for NerveMesher {
   fn default() -> Self {
      NerveMesher {
         shader: NerveShader { program_id: 0 },
         transform: Default::default(),
         pos_attr: PositionAttr(AttrData::Empty),
         col_attr: ColorAttr(AttrData::Empty),
         uvm_attr: UVMapAttr(AttrData::Empty),
         indices: Indices(AttrData::Empty),
      }
   }
}

impl NerveMesher {
   pub fn build(&mut self) -> NerveMesh {
      let mut gl_vert_obj = GLVerts::new();
      let mut gl_indices_obj = GLIndices::new();

      let pos_data = self.pos_attr.0.get();
      let col_data = self.col_attr.0.get();
      let uvm_data = self.uvm_attr.0.get();
      let ind_data = self.indices.0.get();

      let pos_bytes;
      let col_bytes;
      let uvm_bytes;
      let pos_elems;
      let mut col_elems = 0;
      let mut uvm_elems = 0;
      let (mut col_exists, mut uvm_exists) = (false, false);

      let pos_vec;
      let mut col_vec = &Vec::new();
      let mut uvm_vec = &Vec::new();
      let mut pcu_vec = Vec::new();

      let mut vert_count: u32 = 0;
      let mut has_indices = false;
      let mut ind_count = 0;
      let mut stride = 0;

      if pos_data.is_some() {
         pos_vec = pos_data.unwrap();
         (pos_bytes, pos_elems) = get_format(&pos_vec[0]);
         stride += pos_elems * pos_bytes;

         if col_data.is_some() {
            col_vec = col_data.unwrap();
            (col_bytes, col_elems) = get_format(&col_vec[0]);
            stride += col_elems * col_bytes;
            col_exists = true;
         }
         if uvm_data.is_some() {
            uvm_vec = uvm_data.unwrap();
            (uvm_bytes, uvm_elems) = get_format(&uvm_vec[0]);
            stride += uvm_elems * uvm_bytes;
            uvm_exists = true;
         }
         for (i, pos) in pos_vec.iter().enumerate() {
            vert_count += 1;
            pcu_vec.push(pos.0.to_bits());
            pcu_vec.push(pos.1.to_bits());
            pcu_vec.push(pos.2.to_bits());
            if col_exists {
               pcu_vec.push(col_vec[i].0.to_bits());
               pcu_vec.push(col_vec[i].1.to_bits());
               pcu_vec.push(col_vec[i].2.to_bits());
            }
            if uvm_exists {
               pcu_vec.push(uvm_vec[i].0.to_bits());
               pcu_vec.push(uvm_vec[i].1.to_bits());
            }
         }
         gl_vert_obj.bind();
         gl_vert_obj.fill(&pcu_vec);
         //POS
         gl_vert_obj.gen_ptr(pos_elems, gl::FLOAT, stride);
         match (col_exists, uvm_exists) {
            //COL UVM
            (true, _) => gl_vert_obj.gen_ptr(col_elems, gl::FLOAT, stride),
            (_, true) => gl_vert_obj.gen_ptr(uvm_elems, gl::FLOAT, stride),
            _ => {}
         }
         gl_vert_obj.unbind();

         if ind_data.is_some() {
            let ind_data = ind_data.unwrap();
            has_indices = true;

            let mut ind_vec = Vec::new();
            for index in ind_data.iter() {
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
