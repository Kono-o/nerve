use crate::util::{NEError, NEResult};
use crate::*;
use std::fmt::Debug;
use std::path::PathBuf;

macro_rules! str {
   ($t:expr) => {
      format!("{}", $t)
   };
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
      let pathbuf = PathBuf::from(path);

      let not_valid = NEResult::ER(NEError::File {
         kind: NEFileErrKind::NotValidPath,
         path: path.to_string(),
      });

      let unsupported = NEResult::ER(NEError::File {
         kind: NEFileErrKind::Unsupported,
         path: path.to_string(),
      });

      match pathbuf.extension() {
         Some(ex) => match ex.to_str().unwrap_or("") {
            "obj" => {
               let obj = match NEObj::load_from_disk(path) {
                  NEResult::ER(e) => return NEResult::ER(e),
                  NEResult::OK(o) => o,
               };
               NEResult::OK(NEMeshAsset::from_obj(obj))
            }
            _ => unsupported,
         },
         None => not_valid,
      }
   }

   pub(crate) fn from_obj(obj: NEObj) -> NEMeshAsset {
      NEMeshAsset {
         shader: NEShader::temporary(),
         transform: Transform::default(),
         pos_attr: obj.pos_attr,
         col_attr: obj.col_attr,
         uvm_attr: obj.uvm_attr,
         nrm_attr: obj.nrm_attr,
         cus_attrs: Vec::new(),
         indices: obj.indices,
      }
   }
   pub fn attach_custom_attr(&mut self, cus_attr: CustomATTR) {
      self.cus_attrs.push(cus_attr);
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
