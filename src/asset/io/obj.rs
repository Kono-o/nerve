use crate::asset::io::file;
use crate::util::{NEError, NEResult};
use crate::{ColATTR, Indices, NrmATTR, PosATTR, UVMATTR};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

trait ParseWords {
   fn parse_2_to_f32(&self) -> [f32; 2];
   fn parse_3_to_f32(&self) -> [f32; 3];
   fn parse_to_usize(&self) -> Vec<usize>;
}
impl ParseWords for Vec<&str> {
   fn parse_2_to_f32(&self) -> [f32; 2] {
      const N: usize = 2;
      let mut elem = [0.0; N];
      for i in 1..=N {
         elem[i - 1] = self[i].parse::<f32>().unwrap_or(0.0)
      }
      elem[1] *= -1.0;
      elem
   }

   fn parse_3_to_f32(&self) -> [f32; 3] {
      const N: usize = 3;
      let mut elem = [0.0; N];
      for i in 1..=N {
         elem[i - 1] = self[i].parse::<f32>().unwrap_or(0.0)
      }
      elem
   }

   fn parse_to_usize(&self) -> Vec<usize> {
      let mut elem: Vec<usize> = Vec::new();
      for str in self {
         elem.push(str.parse::<usize>().unwrap_or(1) - 1);
      }
      elem
   }
}

pub(crate) enum NEObjErrKind {
   NonTriMesh,
}

pub(crate) struct NEObj {
   pub(crate) pos_attr: PosATTR,
   pub(crate) col_attr: ColATTR,
   pub(crate) uvm_attr: UVMATTR,
   pub(crate) nrm_attr: NrmATTR,
   pub(crate) indices: Indices,
}

impl NEObj {
   pub(crate) fn load_from_disk(path: &str) -> NEResult<NEObj> {
      let mut pos_attr = PosATTR::empty();
      let mut col_attr = ColATTR::empty();
      let mut uvm_attr = UVMATTR::empty();
      let mut nrm_attr = NrmATTR::empty();
      let mut indices = Indices::empty();

      let mut pos_data = Vec::new();
      let mut uvm_data = Vec::new();
      let mut nrm_data = Vec::new();
      let mut verts = HashMap::new();

      let obj_file = match file::load_from_disk(path) {
         NEResult::OK(of) => of,
         NEResult::ER(e) => return NEResult::ER(e),
      };
      let obj_src = BufReader::new(obj_file);

      for line_res in obj_src.lines() {
         let line = line_res.unwrap_or(" ".to_string());
         let words = line.split(' ').collect::<Vec<&str>>();
         if words.is_empty() {
            continue;
         }
         match words[0] {
            "v" => pos_data.push(words.parse_3_to_f32()),
            "vt" => uvm_data.push(words.parse_2_to_f32()),
            "vn" => nrm_data.push(words.parse_3_to_f32()),
            "f" => {
               if words.len() > 4 {
                  return NEResult::ER(NEError::Obj {
                     kind: NEObjErrKind::NonTriMesh,
                     path: path.to_string(),
                  });
               }
               for word in &words[1..] {
                  let tokens = word.split('/').collect::<Vec<&str>>();
                  let vert = tokens.parse_to_usize();
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

      NEResult::OK(NEObj {
         pos_attr,
         col_attr,
         uvm_attr,
         nrm_attr,
         indices,
      })
   }
}
