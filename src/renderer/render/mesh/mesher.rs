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
   pub cus_attrs: Vec<CustomAttr>,
   pub start_with_custom: bool,
}
impl Default for NerveMesher {
   fn default() -> Self {
      NerveMesher {
         shader: NerveShader { program_id: 0 },
         transform: Default::default(),
         pos_attr: PositionAttr::empty(),
         col_attr: ColorAttr::empty(),
         uvm_attr: UVMapAttr::empty(),
         nrm_attr: NormalAttr::empty(),
         indices: Indices::empty(),
         cus_attrs: Vec::new(),
         start_with_custom: false,
      }
   }
}

impl NerveMesher {
   pub fn attach_custom_attr(mut self, cus_attr: CustomAttr) -> NerveMesher {
      self.cus_attrs.push(cus_attr);
      self
   }
   fn has_custom_attrs(&self) -> bool {
      !self.cus_attrs.is_empty()
   }
   pub fn build(&mut self) -> NerveMesh {
      let mut gl_verts_obj = GLVerts::new();
      let mut gl_index_obj = GLIndices::new();

      let (mut pos_info, mut pos_data) = (Info::new(), &Vec::new());
      let (mut col_info, mut col_data) = (Info::new(), &Vec::new());
      let (mut uvm_info, mut uvm_data) = (Info::new(), &Vec::new());
      let (mut nrm_info, mut nrm_data) = (Info::new(), &Vec::new());
      let (mut ind_info, mut ind_data) = (Info::new(), &Vec::new());
      let mut cus_infos: Vec<Info> = Vec::new();
      let mut cus_datas: Vec<&Vec<u8>> = Vec::new();

      let mut ind_count = 0;
      let mut vert_count = 0;
      let mut stride = 0;

      let mut pos_exists = self.pos_attr.has_data();
      let mut col_exists = self.col_attr.has_data();
      let mut uvm_exists = self.uvm_attr.has_data();
      let mut nrm_exists = self.nrm_attr.has_data();
      let mut cus_exists = self.has_custom_attrs();

      if pos_exists {
         pos_info = self.pos_attr.info();
         pos_data = self.pos_attr.data();
         stride += pos_info.elem_count * pos_info.byte_count;
      }
      if col_exists {
         col_info = self.col_attr.info();
         col_data = self.col_attr.data();
         stride += col_info.elem_count * col_info.byte_count;
      }
      if uvm_exists {
         uvm_info = self.uvm_attr.info();
         uvm_data = self.uvm_attr.data();
         stride += uvm_info.elem_count * uvm_info.byte_count;
      }
      if nrm_exists {
         nrm_info = self.nrm_attr.info();
         nrm_data = self.nrm_attr.data();
         stride += nrm_info.elem_count * nrm_info.byte_count;
      }
      if cus_exists {
         for cus_attr in self.cus_attrs.iter() {
            let cus_info = cus_attr.info();
            let cus_data = cus_attr.data();
            stride += cus_info.elem_count * cus_info.byte_count;
            cus_infos.push(cus_info);
            cus_datas.push(cus_data);
         }
      }
      let mut end = pos_data.len();
      if cus_exists && self.start_with_custom {
         end = cus_datas[0].len() / (cus_infos[0].byte_count * cus_infos[0].elem_count);
      }

      for i in 0..end {
         vert_count += 1;
         if pos_exists {
            gl_verts_obj.push(&pos_data[i]);
         }
         if col_exists {
            gl_verts_obj.push(&col_data[i]);
         }
         if uvm_exists {
            gl_verts_obj.push(&uvm_data[i]);
         }
         if nrm_exists {
            gl_verts_obj.push(&nrm_data[i]);
         }
         if cus_exists {
            for (j, _attr) in self.cus_attrs.iter().enumerate() {
               let cus_byte_count = cus_infos[j].byte_count * cus_infos[j].elem_count;
               let cus_data = cus_datas[j];
               let start = (i * cus_byte_count);
               let end = ((i + 1) * (cus_byte_count)) - 1;
               gl_verts_obj.push(&cus_data[start..=end]);
            }
         }
      }
      gl_verts_obj.bind();
      gl_verts_obj.stride = stride;

      let id = gl_verts_obj.layout(pos_info);
      println!("pos: {:?}", id);

      let id = gl_verts_obj.layout(col_info);
      println!("col: {:?}", id);

      let id = gl_verts_obj.layout(uvm_info);
      println!("uvm: {:?}", id);

      let id = gl_verts_obj.layout(nrm_info);
      println!("nrm: {:?}", id);

      for (i, cus_info) in cus_infos.iter().enumerate() {
         let id = gl_verts_obj.layout(*cus_info);
         println!("c{i}: {:?}", id);
      }
      gl_verts_obj.ship();

      if self.indices.has_data() {
         ind_info = self.indices.info();
         ind_data = self.indices.data();
         ind_info.exists = true;
         for index in ind_data.iter() {
            ind_count += 1;
            gl_index_obj.push(*index);
         }
         gl_index_obj.bind();
         gl_index_obj.ship();
      }
      self.transform.calc_matrix();
      NerveMesh {
         shader: self.shader,
         has_indices: ind_info.exists,
         vert_count,
         ind_count,
         is_empty: false,
         vert_object: gl_verts_obj,
         index_object: gl_index_obj,
         transform: self.transform.clone(),
         ..Default::default()
      }
   }
}

//NerveMesh {
//   shader: self.shader,
//   has_indices: false,
//   vert_count: 0,
//   ind_count: 0,
//   is_empty: true,
//   vert_object: GLVerts {
//      vao: 0,
//      vbo: 0,
//      attrib_id: 0,
//      local_offset: 0,
//      stride: 0,
//      buffer: vec![],
//   },
//   index_object: GLIndices {
//      ebo: 0,
//      buffer: vec![],
//   },
//   transform: self.transform.clone(),
//   ..Default::default()
//}
