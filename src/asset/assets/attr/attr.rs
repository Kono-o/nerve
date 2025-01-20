use crate::DataType;

#[derive(Clone, Debug)]
pub enum ATTRType {
   U8,
   I8,
   U16,
   I16,
   U32,
   I32,
   F32,
   F64,
}

#[derive(Clone, Debug)]
pub(crate) enum ATTRName {
   Custom(String),
   Pos,
   Col,
   UVM,
   Nrm,
   Ind,
}

impl ATTRName {
   pub(crate) fn as_string(&self) -> String {
      match self {
         ATTRName::Pos => "position".to_string(),
         ATTRName::Col => "color".to_string(),
         ATTRName::UVM => "uv map".to_string(),
         ATTRName::Nrm => "normals".to_string(),
         ATTRName::Ind => "indices".to_string(),
         ATTRName::Custom(n) => format!("{n}(custom)"),
      }
   }
}

#[derive(Clone, Debug)]
pub(crate) struct ATTRInfo {
   pub(crate) name: ATTRName,
   pub(crate) typ: ATTRType,
   pub(crate) byte_count: usize,
   pub(crate) elem_count: usize,
}
impl ATTRInfo {
   pub(crate) fn empty() -> ATTRInfo {
      ATTRInfo {
         name: ATTRName::Pos,
         typ: ATTRType::F32,
         byte_count: 0,
         elem_count: 0,
      }
   }
   pub(crate) fn fmt_as_string(&self) -> String {
      let typ_str = match self.typ {
         ATTRType::U8 => "u8",
         ATTRType::I8 => "i8",
         ATTRType::U16 => "u16",
         ATTRType::I16 => "i16",
         ATTRType::U32 => "u32",
         ATTRType::I32 => "i32",
         ATTRType::F32 => "f32",
         ATTRType::F64 => "f64",
      };
      match self.elem_count == 1 {
         false => format!("[{typ_str};{}]", self.elem_count),
         _ => format!("{typ_str}"),
      }
   }
}

macro_rules! attr {
   ($attr:ident, $typ:ty, $name: expr) => {
      #[derive(Debug, Clone)]
      pub struct $attr {
         pub(crate) data: Vec<$typ>,
         pub(crate) info: ATTRInfo,
      }
      impl $attr {
         pub fn empty() -> $attr {
            let mut info = ATTRInfo::empty();
            info.typ = <$typ>::ATTR_FORMAT;
            info.byte_count = <$typ>::BYTE_COUNT;
            info.elem_count = <$typ>::ELEM_COUNT;
            info.name = $name;
            $attr {
               data: Vec::new(),
               info,
            }
         }
         pub fn from(vec: Vec<$typ>) -> $attr {
            let mut attr = $attr::empty();
            for elem in vec.iter() {
               attr.data.push(*elem);
            }
            attr
         }
         pub fn from_array(array: &[$typ]) -> $attr {
            let mut vec = Vec::from(array);
            $attr::from(vec)
         }

         pub fn push(&mut self, elem: $typ) {
            self.data.push(elem);
         }
         pub fn is_empty(&self) -> bool {
            self.data.is_empty()
         }
      }
   };
}
attr!(PosATTR, [f32; 3], ATTRName::Pos);
attr!(ColATTR, [f32; 3], ATTRName::Col);
attr!(UVMATTR, [f32; 2], ATTRName::UVM);
attr!(NrmATTR, [f32; 3], ATTRName::Nrm);
attr!(IndATTR, u32, ATTRName::Ind);

#[derive(Debug)]
pub struct CustomATTR {
   pub(crate) data: Vec<u8>,
   pub(crate) info: ATTRInfo,
}
impl CustomATTR {
   pub fn empty<D: DataType>(name: &str) -> CustomATTR {
      let mut info = ATTRInfo::empty();
      info.typ = D::ATTR_FORMAT;
      info.byte_count = D::BYTE_COUNT;
      info.elem_count = D::ELEM_COUNT;
      info.name = ATTRName::Custom(name.to_string());
      CustomATTR {
         data: Vec::new(),
         info,
      }
   }
   pub fn from<D: DataType>(name: &str, vec: Vec<D>) -> CustomATTR {
      let mut attr = CustomATTR::empty::<D>(name);
      for elem in vec.iter() {
         let bytes = elem.u8ify();
         for byte in bytes.iter() {
            attr.data.push(*byte);
         }
      }
      attr
   }
   pub fn from_array<D: DataType + Clone>(name: &str, array: &[D]) -> CustomATTR {
      CustomATTR::from(name, Vec::from(array))
   }
   pub fn push<D: DataType>(&mut self, elem: D) {
      let bytes = elem.u8ify();
      for byte in bytes.iter() {
         self.data.push(*byte);
      }
   }

   pub fn is_empty(&self) -> bool {
      self.data.is_empty()
   }
}
