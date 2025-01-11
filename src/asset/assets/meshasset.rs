use crate::*;
use std::collections::HashMap;

macro_rules! stringify {
   ($t:expr) => {
      format!("{}", $t)
   };
}

enum OBJ {
   Parsed {
      pos_attr: PosATTR,
      col_attr: ColATTR,
      uvm_attr: UVMATTR,
      nrm_attr: NrmATTR,
      indices: Indices,
   },
   NonTriangle(String),
}

impl OBJ {
   fn parse(src: &str) -> OBJ {
      let mut pos_attr = PosATTR::empty();
      let mut col_attr = ColATTR::empty();
      let mut uvm_attr = UVMATTR::empty();
      let mut nrm_attr = NrmATTR::empty();
      let mut indices = Indices::empty();

      let mut pos_data = Vec::new();
      let mut uvm_data = Vec::new();
      let mut nrm_data = Vec::new();
      type Vert = Vec<usize>;
      let mut verts: Vec<Vert> = Vec::new();
      let mut unique_verts = HashMap::new();

      for line in src.lines() {
         let line = line.trim();
         let words = line.split(' ').collect::<Vec<&str>>();
         if words.is_empty() {
            continue;
         }
         match words[0] {
            "v" => pos_data.push(words.parse_3_to_f32()),
            "vt" => uvm_data.push(words.parse_2_to_f32()),
            "vn" => nrm_data.push(words.parse_3_to_f32()),
            "f" => {
               if words.len() != 4 {
                  return OBJ::NonTriangle(line.to_string());
               }
               for word in &words[1..] {
                  let tokens = word.split('/').collect::<Vec<&str>>();
                  let vert = tokens.parse_to_usize();
                  verts.push(vert);
               }
            }
            _ => {}
         }
      }
      let attr_count = verts[0].len();
      let pos_exists = attr_count > 0;
      let uvm_exists = attr_count > 1;
      let nrm_exists = attr_count > 2;

      let def_uvm = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0]];
      let def_col = [1.0, 1.0, 1.0];
      let def_nrm = [1.0, 1.0, 1.0];
      for (i, vert) in verts.iter().enumerate() {
         let pos_index = match pos_exists {
            true => Some(vert[0]),
            _ => None,
         };
         let uvm_index = match uvm_exists {
            true => Some(vert[1]),
            _ => None,
         };
         let nrm_index = match nrm_exists {
            true => Some(vert[2]),
            _ => None,
         };

         let key = (pos_index, uvm_index, nrm_index);
         if unique_verts.contains_key(&key) {
            let idx = unique_verts[&key] as u32;
            indices.shove(idx);
         } else {
            let v_local = i % 3;
            let new = pos_attr.data.len();
            unique_verts.insert(key, new);
            pos_attr.shove(match pos_index {
               Some(id) => pos_data[id],
               None => [0.0; 3],
            });
            uvm_attr.shove(match uvm_index {
               Some(id) => uvm_data[id],
               None => def_uvm[v_local],
            });
            nrm_attr.shove(match nrm_index {
               Some(id) => nrm_data[id],
               None => def_nrm,
            });
            col_attr.shove(def_col);
            indices.shove(new as u32);
         }
      }

      pos_attr.calc_info();
      col_attr.calc_info();
      uvm_attr.calc_info();
      nrm_attr.calc_info();
      indices.calc_info();

      OBJ::Parsed {
         pos_attr,
         col_attr,
         uvm_attr,
         nrm_attr,
         indices,
      }
   }
}

pub struct NEMeshAsset {
   pub(crate) shader: NEShader,
   pub(crate) transform: Transform,
   pub(crate) pos_attr: PosATTR,
   pub(crate) col_attr: ColATTR,
   pub(crate) uvm_attr: UVMATTR,
   pub(crate) nrm_attr: NrmATTR,
   pub(crate) cus_attrs: Vec<CustomATTR>,
   pub(crate) indices: Indices,
}

impl NEMeshAsset {
   pub fn from_path(path: &str) -> NEResult<NEMeshAsset> {
      NEMeshAsset::from_path_raw(&env::concat_with_asset(path))
   }
   fn from_path_raw(raw_path: &str) -> NEResult<NEMeshAsset> {
      let file_name = match file::name(raw_path) {
         NEOption::Empty => return NEResult::ER(NEError::file_invalid(raw_path)),
         NEOption::Exists(n) => n,
      };
      let _ = match file::ex(raw_path) {
         NEOption::Empty => return NEResult::ER(NEError::file_invalid(raw_path)),
         NEOption::Exists(ex) => match ex.eq_ignore_ascii_case(ex::OBJ) {
            false => return NEResult::ER(NEError::file_unsupported(raw_path, &ex)),
            true => ex,
         },
      };
      let nmesh_path = format!("{}{}.{}", path::MESH_ASSET, file_name, ex::NMESH);

      let file_exists = file::exists_on_disk(raw_path);
      let nmesh_exists = file::exists_on_disk(&nmesh_path);

      if !file_exists && !nmesh_exists {
         let both_paths = format!("{} or {}", raw_path, nmesh_path);
         return NEResult::ER(NEError::file_missing(&both_paths));
      }
      if file_exists {
         //write/overwrite nmesh
         let obj_src = match file::read_as_string(raw_path) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(of) => of,
         };
         let nmesh = match OBJ::parse(&obj_src) {
            OBJ::NonTriangle(line) => {
               return NEResult::ER(NEError::non_triangulated(raw_path, line))
            }
            OBJ::Parsed {
               pos_attr,
               col_attr,
               uvm_attr,
               nrm_attr,
               indices,
            } => NEMeshAsset {
               shader: NEShader::temporary(),
               transform: Transform::default(),
               cus_attrs: Vec::new(),
               pos_attr,
               col_attr,
               uvm_attr,
               nrm_attr,
               indices,
            },
         };

         let nmesh_name = format!("{file_name}.{}", ex::NMESH);
         match file::write_str_to_disk(path::MESH_ASSET, &nmesh_name, &obj_src) {
            NEResult::ER(e) => NEResult::ER(e),
            _ => NEResult::OK(nmesh),
         }
      } else {
         //load new/pre-existing nmesh
         let nmesh_src = match file::read_as_string(&nmesh_path) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(f) => f,
         };
         let nmesh = match OBJ::parse(&nmesh_src) {
            OBJ::NonTriangle(line) => {
               return NEResult::ER(NEError::non_triangulated(raw_path, line))
            }
            OBJ::Parsed {
               pos_attr,
               col_attr,
               uvm_attr,
               nrm_attr,
               indices,
            } => NEMeshAsset {
               shader: NEShader::temporary(),
               transform: Transform::default(),
               cus_attrs: Vec::new(),
               pos_attr,
               col_attr,
               uvm_attr,
               nrm_attr,
               indices,
            },
         };
         NEResult::OK(nmesh)
      }
   }

   pub fn attach_custom_attr(&mut self, cus_attr: CustomATTR) {
      self.cus_attrs.push(cus_attr);
   }

   pub fn has_no_attr(&self) -> bool {
      let no_attr = self.starts_with_custom();
      let no_cus_attr = self.cus_attrs.len() == 0;
      no_attr && no_cus_attr
   }

   pub fn starts_with_custom(&self) -> bool {
      self.pos_attr.is_empty()
         && self.col_attr.is_empty()
         && self.uvm_attr.is_empty()
         && self.nrm_attr.is_empty()
   }
   pub fn set_shader(&mut self, shader: NEShader) {
      self.shader = shader;
   }
   pub(crate) fn has_custom_attrs(&self) -> bool {
      !self.cus_attrs.is_empty()
   }
}

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
