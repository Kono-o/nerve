use crate::*;
use std::fmt::Debug;
use std::path::PathBuf;

macro_rules! str {
   ($t:expr) => {
      format!("{}", $t)
   };
}

pub struct NEMeshSrc {
   pub shader: NEShader,
   pub transform: Transform,
   pub pos_attr: PosATTR,
   pub col_attr: ColATTR,
   pub uvm_attr: UVMATTR,
   pub nrm_attr: NrmATTR,
   pub indices: Indices,
   pub cus_attrs: Vec<CustomATTR>,
}
impl Default for NEMeshSrc {
   fn default() -> Self {
      NEMeshSrc {
         shader: NEShader::empty(),
         transform: Default::default(),
         pos_attr: PosATTR::empty(),
         col_attr: ColATTR::empty(),
         uvm_attr: UVMATTR::empty(),
         nrm_attr: NrmATTR::empty(),
         indices: Indices::empty(),
         cus_attrs: Vec::new(),
      }
   }
}

impl NEMeshSrc {
   pub fn from_path(path: &str) -> NEResult<NEMeshSrc> {
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
                  NEResult::OK(o) => o,
                  NEResult::ER(e) => return NEResult::ER(e),
               };
               NEMeshSrc::from_obj(obj)
            }
            _ => unsupported,
         },
         None => not_valid,
      }
   }

   pub(crate) fn from_obj(obj: NEObj) -> NEResult<NEMeshSrc> {
      NEResult::OK(NEMeshSrc {
         pos_attr: obj.pos_attr,
         col_attr: obj.col_attr,
         uvm_attr: obj.uvm_attr,
         nrm_attr: obj.nrm_attr,
         indices: obj.indices,
         ..Self::default()
      })
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
