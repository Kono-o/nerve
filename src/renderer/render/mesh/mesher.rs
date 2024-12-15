use crate::renderer::render::mesh::glbuffers::{GLIndices, GLVerts};
use crate::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! str {
   ($t:expr) => {
      format!("{}", $t)
   };
}

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
         shader: NerveShader::default(),
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

fn dump_into(data: &mut Vec<[f32; 3]>, words: &mut Vec<&str>) {
   let mut elem: [f32; 3] = [0.0; 3];
   for i in 1..=3 {
      elem[i - 1] = words[i].parse::<f32>().unwrap();
   }
   data.push(elem);
}
fn dump_into_uvm(data: &mut Vec<[f32; 2]>, words: &mut Vec<&str>) {
   let mut elem: [f32; 2] = [0.0; 2];
   for i in 1..=2 {
      elem[i - 1] = words[i].parse::<f32>().unwrap();
   }
   data.push([elem[0], elem[1] * -1.0]); //flip y for opengl
}

fn str_vec_to_usize(strs: Vec<&str>) -> Vec<usize> {
   let mut vec: Vec<usize> = Vec::new();
   for str in strs {
      vec.push(str.parse::<usize>().unwrap() - 1);
   }
   vec
}
impl NerveMesher {
   pub fn obj(obj_path: &str) -> NerveMesher {
      let mut pos_attr: PositionAttr = PositionAttr::empty();
      let mut col_attr: ColorAttr = ColorAttr::empty();
      let mut uvm_attr: UVMapAttr = UVMapAttr::empty();
      let mut nrm_attr: NormalAttr = NormalAttr::empty();
      let mut indices: Indices = Indices::empty();

      let mut pos_data = Vec::new();
      let mut uvm_data = Vec::new();
      let mut nrm_data = Vec::new();
      let mut verts = HashMap::new();

      //OPEN FILE
      let obj = match File::open(obj_path) {
         Ok(file) => file,
         Err(error) => panic!("{obj_path}: {error}"),
      };
      //READ FILE
      let obj_src = BufReader::new(obj);
      for line_res in obj_src.lines() {
         let line = line_res.unwrap_or(" ".to_string());
         let mut words = line.split_whitespace().collect::<Vec<&str>>();
         if words.is_empty() {
            continue;
         }
         match words[0] {
            "v" => dump_into(&mut pos_data, &mut words),
            "vt" => dump_into_uvm(&mut uvm_data, &mut words),
            "vn" => dump_into(&mut nrm_data, &mut words),
            "f" => {
               if words.len() > 4 {
                  panic!("non-triangulated meshes not supported!");
               }
               for word in &words[1..] {
                  let vert = str_vec_to_usize(word.split('/').collect::<Vec<&str>>());
                  let index = vert[0] as u32;
                  let pos_index = vert[0];
                  let uvm_index = vert[1];
                  let nrm_index = vert[2];

                  let key = (pos_index, uvm_index, nrm_index);
                  if verts.contains_key(&key) {
                     let idx = verts[&key] as u32;
                     indices.shove(idx);
                  } else {
                     let new = pos_attr.data.len();
                     verts.insert(key, new);
                     pos_attr.shove(pos_data[pos_index]);
                     uvm_attr.shove(uvm_data[uvm_index]);
                     nrm_attr.shove(nrm_data[nrm_index]);
                     col_attr.shove([1.0, 1.0, 1.0]); //default
                     indices.shove(new as u32);
                  }
               }
            }
            _ => {}
         }
      }
      pos_attr.calc_info();
      col_attr.calc_info();
      uvm_attr.calc_info();
      nrm_attr.calc_info();
      indices.calc_info();
      NerveMesher {
         pos_attr,
         col_attr,
         uvm_attr,
         nrm_attr,
         indices,
         ..Self::default()
      }
   }
   pub fn attach_custom_attr(mut self, cus_attr: CustomAttr) -> NerveMesher {
      self.cus_attrs.push(cus_attr);
      self
   }
   pub fn set_shader(mut self, shader: NerveShader) -> NerveMesher {
      self.shader = shader;
      self
   }
   fn has_custom_attrs(&self) -> bool {
      !self.cus_attrs.is_empty()
   }

   pub fn mesh(&mut self) -> NerveMesh {
      let mut gl_verts_obj = GLVerts::new();
      let mut gl_index_obj = GLIndices::new();

      let (mut pos_info, mut pos_data) = (AttrInfo::empty(), &Vec::new());
      let (mut col_info, mut col_data) = (AttrInfo::empty(), &Vec::new());
      let (mut uvm_info, mut uvm_data) = (AttrInfo::empty(), &Vec::new());
      let (mut nrm_info, mut nrm_data) = (AttrInfo::empty(), &Vec::new());
      let (mut ind_info, mut ind_data) = (AttrInfo::empty(), &Vec::new());
      let mut cus_infos: Vec<AttrInfo> = Vec::new();
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

      let mut layouts: Vec<String> = Vec::new();

      let id = gl_verts_obj.layout(pos_info);
      layouts.push(format!("POSITION(f32x3): {:?}", id));

      let id = gl_verts_obj.layout(col_info);
      layouts.push(format!("COLOR(f32x3): {:?}", id));

      let id = gl_verts_obj.layout(uvm_info);
      layouts.push(format!("UVMAP(f32x2): {:?}", id));

      let id = gl_verts_obj.layout(nrm_info);
      layouts.push(format!("NORMAL(f32x3): {:?}", id));

      for (i, cus_info) in cus_infos.iter().enumerate() {
         let id = gl_verts_obj.layout(cus_info.clone());
         let format = cus_info.typ_str.clone();
         layouts.push(format!("CUSTOM{i}({format}): {:?}", id));
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
         shader: self.shader.clone(),
         has_indices: ind_info.exists,
         vert_count,
         ind_count,
         is_empty: false,
         layouts,
         vert_object: gl_verts_obj,
         index_object: gl_index_obj,
         transform: self.transform.clone(),
         ..Default::default()
      }
   }
}
