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

pub struct NerveMeshSrc {
   pub shader: NerveShader,
   pub transform: Transform,
   pub pos_attr: PositionAttr,
   pub col_attr: ColorAttr,
   pub uvm_attr: UVMapAttr,
   pub nrm_attr: NormalAttr,
   pub indices: Indices,
   pub cus_attrs: Vec<CustomAttr>,
}
impl Default for NerveMeshSrc {
   fn default() -> Self {
      NerveMeshSrc {
         shader: NerveShader::empty(),
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

fn dump_into_3(data: &mut Vec<[f32; 3]>, words: &mut Vec<&str>) {
   let mut elem: [f32; 3] = [0.0; 3];
   for i in 1..=3 {
      elem[i - 1] = words[i].parse::<f32>().unwrap();
   }
   data.push(elem);
}
fn dump_into_2(data: &mut Vec<[f32; 2]>, words: &mut Vec<&str>) {
   let mut elem: [f32; 2] = [0.0; 2];
   for i in 1..=2 {
      elem[i - 1] = words[i].parse::<f32>().unwrap();
   }
   data.push([elem[0], elem[1] * -1.0]);
}

fn str_vec_to_usize(strs: Vec<&str>) -> Vec<usize> {
   let mut vec: Vec<usize> = Vec::new();
   for str in strs {
      vec.push(str.parse::<usize>().unwrap() - 1);
   }
   vec
}

impl NerveMeshSrc {
   pub fn from_obj(obj_path: &str) -> NerveMeshSrc {
      let mut pos_attr: PositionAttr = PositionAttr::empty();
      let mut col_attr: ColorAttr = ColorAttr::empty();
      let mut uvm_attr: UVMapAttr = UVMapAttr::empty();
      let mut nrm_attr: NormalAttr = NormalAttr::empty();
      let mut indices: Indices = Indices::empty();

      let mut pos_data = Vec::new();
      let mut uvm_data = Vec::new();
      let mut nrm_data = Vec::new();
      let mut verts = HashMap::new();

      let obj = match File::open(obj_path) {
         Ok(file) => file,
         Err(error) => panic!("{obj_path}: {error}"),
      };
      let obj_src = BufReader::new(obj);
      for line_res in obj_src.lines() {
         let line = line_res.unwrap_or(" ".to_string());
         let mut words = line.split(' ').collect::<Vec<&str>>();
         if words.is_empty() {
            continue;
         }
         match words[0] {
            "v" => dump_into_3(&mut pos_data, &mut words),
            "vt" => dump_into_2(&mut uvm_data, &mut words),
            "vn" => dump_into_3(&mut nrm_data, &mut words),
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
      NerveMeshSrc {
         pos_attr,
         col_attr,
         uvm_attr,
         nrm_attr,
         indices,
         ..Self::default()
      }
   }
   pub fn attach_custom_attr(mut self, cus_attr: CustomAttr) -> NerveMeshSrc {
      self.cus_attrs.push(cus_attr);
      self
   }
   pub fn starts_with_custom(&self) -> bool {
      self.pos_attr.is_empty()
         && self.col_attr.is_empty()
         && self.uvm_attr.is_empty()
         && self.nrm_attr.is_empty()
   }
   pub fn set_shader(mut self, shader: NerveShader) -> NerveMeshSrc {
      self.shader = shader;
      self
   }
   pub(crate) fn has_custom_attrs(&self) -> bool {
      !self.cus_attrs.is_empty()
   }
}
