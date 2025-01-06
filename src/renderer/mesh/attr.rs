use std::any::TypeId;

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
pub(crate) struct ATTRInfo {
   pub(crate) typ: ATTRType,
   pub(crate) typ_str: String,
   pub(crate) exists: bool,
   pub(crate) byte_count: usize,
   pub(crate) elem_count: usize,
}
impl ATTRInfo {
   pub(crate) fn empty() -> ATTRInfo {
      ATTRInfo {
         typ: ATTRType::U8,
         typ_str: "".to_string(),
         exists: false,
         byte_count: 0,
         elem_count: 0,
      }
   }
}

pub trait DataType {
   fn u8ify(&self) -> Vec<u8>;
}
macro_rules! impl_dataformat {
   ([$t:ty; $s:literal]) => {
      impl DataType for [$t; $s] {
         fn u8ify(&self) -> Vec<u8> {
            let mut vec: Vec<u8> = Vec::new();
            for elem in self.iter() {
               for byte in elem.to_ne_bytes().iter() {
                  vec.push(*byte);
               }
            }
            vec
         }
      }
   };
   ($t:ty) => {
      impl DataType for $t {
         fn u8ify(&self) -> Vec<u8> {
            let mut vec: Vec<u8> = Vec::new();
            for byte in self.to_ne_bytes().iter() {
               vec.push(*byte);
            }
            vec
         }
      }
   };
}
macro_rules! dataformat {
   ($t:ty) => {
      impl_dataformat!($t);
      impl_dataformat!([$t; 2]);
      impl_dataformat!([$t; 3]);
      impl_dataformat!([$t; 4]);
   };
}
dataformat!(i8);
dataformat!(u8);
dataformat!(i16);
dataformat!(u16);
dataformat!(i32);
dataformat!(u32);
dataformat!(f32);
dataformat!(f64);

macro_rules! attribute {
   ($attr:ident, $typ:ty) => {
      #[derive(Debug)]
      pub struct $attr {
         pub(crate) data: Vec<$typ>,
         pub(crate) info: ATTRInfo,
      }
      impl $attr {
         pub fn from(vec: Vec<$typ>) -> $attr {
            let mut data: Vec<$typ> = Vec::new();
            let vec_len = vec.len();
            for elem in vec.iter() {
               data.push(*elem);
            }
            let mut info = ATTRInfo::empty();
            if vec_len > 0 {
               info.exists = true;
               (info.typ, info.typ_str, info.byte_count, info.elem_count) = get_format(&vec[0]);
            }
            $attr { data, info }
         }
         pub fn from_array(array: &[$typ]) -> $attr {
            let mut vec = Vec::from(array);
            $attr::from(vec)
         }

         pub fn calc_info(&mut self) {
            if self.data.len() > 0 {
               self.info.exists = true;
               (
                  self.info.typ,
                  self.info.typ_str,
                  self.info.byte_count,
                  self.info.elem_count,
               ) = get_format(&self.data[0]);
            }
         }
         pub fn empty() -> $attr {
            $attr {
               data: Vec::new(),
               info: ATTRInfo::empty(),
            }
         }
         pub fn shove(&mut self, elem: $typ) {
            self.data.push(elem);
         }
         pub fn is_empty(&self) -> bool {
            !self.info.exists
         }
         pub fn has_data(&self) -> bool {
            self.info.exists
         }
         pub fn data(&self) -> &Vec<$typ> {
            &self.data
         }
         pub fn info(&mut self) -> ATTRInfo {
            ATTRInfo {
               typ: self.info.typ.clone(),
               typ_str: self.info.typ_str.clone(),
               exists: self.info.exists,
               byte_count: self.info.byte_count,
               elem_count: self.info.elem_count,
            }
         }
      }
   };
}
attribute!(PosATTR, [f32; 3]);
attribute!(ColATTR, [f32; 3]);
attribute!(UVMATTR, [f32; 2]);
attribute!(NrmATTR, [f32; 3]);
attribute!(Indices, u32);

pub struct CustomATTR {
   pub(crate) data: Vec<u8>,
   pub(crate) info: ATTRInfo,
}
impl CustomATTR {
   pub fn from<T: DataType + 'static>(vec: Vec<T>) -> CustomATTR {
      let mut data: Vec<u8> = Vec::new();
      let vec_len = vec.len();
      for elem in vec.iter() {
         let bytes = elem.u8ify();
         for byte in bytes.iter() {
            data.push(*byte);
         }
      }
      let mut info = ATTRInfo::empty();
      if vec_len > 0 {
         info.exists = true;
         (info.typ, info.typ_str, info.byte_count, info.elem_count) = get_format(&vec[0]);
      }
      CustomATTR { data, info }
   }
   pub fn from_array<T: DataType + Clone + 'static>(array: &[T]) -> CustomATTR {
      let mut vec = Vec::from(array);
      CustomATTR::from(vec)
   }
   pub fn empty() -> CustomATTR {
      CustomATTR {
         data: Vec::new(),
         info: ATTRInfo::empty(),
      }
   }
   pub fn is_empty(&self) -> bool {
      !self.info.exists
   }
   pub fn has_data(&self) -> bool {
      self.info.exists
   }
   pub fn data(&self) -> &Vec<u8> {
      &self.data
   }
   pub fn info(&self) -> ATTRInfo {
      ATTRInfo {
         typ: self.info.typ.clone(),
         typ_str: self.info.typ_str.clone(),
         exists: self.info.exists,
         byte_count: self.info.byte_count,
         elem_count: self.info.elem_count,
      }
   }
}

// returns attr (type, bytes in 1 element, no of elements)
pub(crate) fn get_format<T: DataType + 'static>(_t: &T) -> (ATTRType, String, usize, usize) {
   let id = TypeId::of::<T>();

   let int8 = ATTRType::I8;
   let uint8 = ATTRType::U8;

   let int16 = ATTRType::I16;
   let uint16 = ATTRType::U16;

   let int32 = ATTRType::I32;
   let uint32 = ATTRType::U32;

   let float32 = ATTRType::F32;
   let float64 = ATTRType::F64;

   // INT8
   if id == TypeId::of::<i8>() {
      (int8, "i8x1".to_string(), 1, 1)
   } else if id == TypeId::of::<[i8; 2]>() {
      (int8, "i8x2".to_string(), 1, 2)
   } else if id == TypeId::of::<[i8; 3]>() {
      (int8, "i8x3".to_string(), 1, 3)
   } else if id == TypeId::of::<[i8; 4]>() {
      (int8, "i8x4".to_string(), 1, 4)
   }
   // UINT8
   else if id == TypeId::of::<u8>() {
      (uint8, "u8x1".to_string(), 1, 1)
   } else if id == TypeId::of::<[u8; 2]>() {
      (uint8, "u8x2".to_string(), 1, 2)
   } else if id == TypeId::of::<[u8; 3]>() {
      (uint8, "u8x3".to_string(), 1, 3)
   } else if id == TypeId::of::<[u8; 4]>() {
      (uint8, "u8x4".to_string(), 1, 4)
   }
   // INT16
   else if id == TypeId::of::<i16>() {
      (int16, "i16x1".to_string(), 2, 1)
   } else if id == TypeId::of::<[i16; 2]>() {
      (int16, "i16x2".to_string(), 2, 2)
   } else if id == TypeId::of::<[i16; 3]>() {
      (int16, "i16x3".to_string(), 2, 3)
   } else if id == TypeId::of::<[i16; 4]>() {
      (int16, "i16x4".to_string(), 2, 4)
   // UINT16
   } else if id == TypeId::of::<u16>() {
      (uint16, "u16x1".to_string(), 2, 1)
   } else if id == TypeId::of::<[u16; 2]>() {
      (uint16, "u16x2".to_string(), 2, 2)
   } else if id == TypeId::of::<[u16; 3]>() {
      (uint16, "u16x3".to_string(), 2, 3)
   } else if id == TypeId::of::<[u16; 4]>() {
      (uint16, "u16x4".to_string(), 2, 4)
   // INT32
   } else if id == TypeId::of::<i32>() {
      (int32, "i32x1".to_string(), 4, 1)
   } else if id == TypeId::of::<[i32; 2]>() {
      (int32, "i32x2".to_string(), 4, 2)
   } else if id == TypeId::of::<[i32; 3]>() {
      (int32, "i32x3".to_string(), 4, 3)
   } else if id == TypeId::of::<[i32; 4]>() {
      (int32, "i32x4".to_string(), 4, 4)
   // UINT32
   } else if id == TypeId::of::<u32>() {
      (uint32, "u32x1".to_string(), 4, 1)
   } else if id == TypeId::of::<[u32; 2]>() {
      (uint32, "u32x2".to_string(), 4, 2)
   } else if id == TypeId::of::<[u32; 3]>() {
      (uint32, "u32x3".to_string(), 4, 3)
   } else if id == TypeId::of::<[u32; 4]>() {
      (uint32, "u32x4".to_string(), 4, 4)
   // FLOAT32
   } else if id == TypeId::of::<f32>() {
      (float32, "f32x1".to_string(), 4, 1)
   } else if id == TypeId::of::<[f32; 2]>() {
      (float32, "f32x2".to_string(), 4, 2)
   } else if id == TypeId::of::<[f32; 3]>() {
      (float32, "f32x3".to_string(), 4, 3)
   } else if id == TypeId::of::<[f32; 4]>() {
      (float32, "f32x4".to_string(), 4, 4)
   // FLOAT64
   } else if id == TypeId::of::<f64>() {
      (float64, "f64x1".to_string(), 8, 1)
   } else if id == TypeId::of::<[f64; 2]>() {
      (float64, "f64x2".to_string(), 8, 2)
   } else if id == TypeId::of::<[f64; 3]>() {
      (float64, "f64x3".to_string(), 8, 3)
   } else if id == TypeId::of::<[f64; 4]>() {
      (float64, "f64x4".to_string(), 8, 4)
   } else {
      (uint8, "unknown".to_string(), 0, 0)
   }
}
