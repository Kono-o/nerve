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
   //list of <custom attrs>
   // that are a list of <verts>
   // that are a list of <bytes>
   pub cus_attrs: Vec<CustomAttr>,
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
      }
   }
}

impl NerveMesher {
   pub fn attach_custom_attr(mut self, cus_attr: CustomAttr) -> NerveMesher {
      //todo
      self
   }

   pub fn build(&mut self) -> NerveMesh {
      self.transform.calc_matrix();

      if self.pos_attr.has_data() {
         let mut gl_verts_obj = GLVerts::new();
         let mut gl_index_obj = GLIndices::new();

         let (mut col_info, mut col_data) = (Info::new(), &Vec::new());
         let (mut uvm_info, mut uvm_data) = (Info::new(), &Vec::new());
         let (mut nrm_info, mut nrm_data) = (Info::new(), &Vec::new());
         let (mut ind_info, mut ind_data) = (Info::new(), &Vec::new());

         let mut ind_count = 0;
         let mut vert_count = 0;
         let mut stride = 0;

         let mut pos_info = self.pos_attr.info();
         let mut pos_data = self.pos_attr.data();
         stride += pos_info.elem_count * pos_info.byte_count;
         pos_info.exists = true;

         let mut col_exists = self.col_attr.has_data();
         let mut uvm_exists = self.uvm_attr.has_data();
         let mut nrm_exists = self.nrm_attr.has_data();

         if col_exists {
            col_info = self.col_attr.info();
            col_data = self.col_attr.data();
            stride += col_info.elem_count * col_info.byte_count;
         }
         if uvm_exists {
            uvm_info = self.uvm_attr.info();
            uvm_data = self.uvm_attr.data();
            stride += uvm_info.elem_count * uvm_info.byte_count;
            uvm_info.exists = true;
         }
         if nrm_exists {
            nrm_info = self.nrm_attr.info();
            nrm_data = self.nrm_attr.data();
            stride += nrm_info.elem_count * nrm_info.byte_count;
            nrm_info.exists = true;
         }

         for (i, pos) in pos_data.iter().enumerate() {
            vert_count += 1;
            gl_verts_obj.push(pos);
            if col_exists {
               gl_verts_obj.push(&col_data[i]);
            }
            if uvm_exists {
               gl_verts_obj.push(&uvm_data[i]);
            }
            if nrm_exists {
               gl_verts_obj.push(&nrm_data[i]);
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
               stride: 0,
               buffer: vec![],
            },
            index_object: GLIndices {
               ebo: 0,
               buffer: vec![],
            },
            transform: self.transform.clone(),
            ..Default::default()
         }
      }
   }
}
