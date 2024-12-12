use gl::types::GLenum;
use std::any::TypeId;

pub(crate) struct Info {
   pub(crate) typ: GLenum,
   pub(crate) exists: bool,
   pub(crate) byte_count: usize,
   pub(crate) elem_count: usize,
}
impl Info {
   pub(crate) fn new() -> Info {
      Info {
         typ: 0,
         exists: false,
         byte_count: 0,
         elem_count: 0,
      }
   }
}

pub trait DataFormat {
   fn u8ify(&self) -> Vec<u8>;
}
macro_rules! impl_dataformat {
   ([$t:ty; $s:literal]) => {
      impl DataFormat for [$t; $s] {
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
      impl DataFormat for $t {
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
      pub struct $attr {
         pub(crate) data: Vec<$typ>,
         pub(crate) info: Info,
      }
      impl $attr {
         pub fn from(vec: Vec<$typ>) -> $attr {
            let mut data: Vec<$typ> = Vec::new();
            let vec_len = vec.len();
            for elem in vec.iter() {
               data.push(*elem);
            }
            let mut info = Info::new();
            if vec_len > 0 {
               info.exists = true;
               (info.typ, info.byte_count, info.elem_count) = get_format(&vec[0]);
            }
            $attr { data, info }
         }
         pub fn from_array(array: &[$typ]) -> $attr {
            let mut vec = Vec::from(array);
            $attr::from(vec)
         }
         pub fn empty() -> $attr {
            $attr {
               data: Vec::new(),
               info: Info::new(),
            }
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
         pub fn info(&mut self) -> Info {
            Info {
               typ: self.info.typ,
               exists: self.info.exists,
               byte_count: self.info.byte_count,
               elem_count: self.info.elem_count,
            }
         }
      }
   };
}
attribute!(PositionAttr, [f32; 3]);
attribute!(ColorAttr, [f32; 3]);
attribute!(UVMapAttr, [f32; 2]);
attribute!(NormalAttr, [f32; 3]);
attribute!(Indices, u32);
pub struct CustomAttr {
   pub(crate) data: Vec<u8>,
   pub(crate) info: Info,
}
impl CustomAttr {
   pub fn from<T: DataFormat + 'static>(vec: Vec<T>) -> CustomAttr {
      let mut data: Vec<u8> = Vec::new();
      let vec_len = vec.len();
      for elem in vec.iter() {
         let bytes = elem.u8ify();
         for byte in bytes.iter() {
            data.push(*byte);
         }
      }
      let mut info = Info::new();
      if vec_len > 0 {
         info.exists = true;
         (info.typ, info.byte_count, info.elem_count) = get_format(&vec[0]);
      }
      CustomAttr { data, info }
   }
   pub fn from_array<T: DataFormat + Clone + 'static>(array: &[T]) -> CustomAttr {
      let mut vec = Vec::from(array);
      CustomAttr::from(vec)
   }
   pub fn empty() -> CustomAttr {
      CustomAttr {
         data: Vec::new(),
         info: Info::new(),
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
   pub fn info(&mut self) -> Info {
      Info {
         typ: self.info.typ,
         exists: self.info.exists,
         byte_count: self.info.byte_count,
         elem_count: self.info.elem_count,
      }
   }
}

// returns (type in gl enum, bytes in 1 element, no of elements)
pub(crate) fn get_format<T: DataFormat + 'static>(_t: &T) -> (GLenum, usize, usize) {
   let id = TypeId::of::<T>();

   let int8 = gl::BYTE;
   let uint8 = gl::UNSIGNED_BYTE;

   let int8 = gl::BYTE;
   let uint8 = gl::UNSIGNED_BYTE;

   let int16 = gl::SHORT;
   let uint16 = gl::UNSIGNED_SHORT;

   let int32 = gl::INT;
   let uint32 = gl::UNSIGNED_INT;

   let float32 = gl::FLOAT;
   let float64 = gl::DOUBLE;

   // INT8
   if id == TypeId::of::<i8>() {
      (int8, 1, 1)
   } else if id == TypeId::of::<[i8; 2]>() {
      (int8, 1, 2)
   } else if id == TypeId::of::<[i8; 3]>() {
      (int8, 1, 3)
   } else if id == TypeId::of::<[i8; 4]>() {
      (int8, 1, 4)
   }
   // UINT8
   else if id == TypeId::of::<u8>() {
      (uint8, 1, 1)
   } else if id == TypeId::of::<[u8; 2]>() {
      (uint8, 1, 2)
   } else if id == TypeId::of::<[u8; 3]>() {
      (uint8, 1, 3)
   } else if id == TypeId::of::<[u8; 4]>() {
      (uint8, 1, 4)
   }
   // INT16
   else if id == TypeId::of::<i16>() {
      (int16, 2, 1)
   } else if id == TypeId::of::<[i16; 2]>() {
      (int16, 2, 2)
   } else if id == TypeId::of::<[i16; 3]>() {
      (int16, 2, 3)
   } else if id == TypeId::of::<[i16; 4]>() {
      (int16, 2, 4)
   }
   // UINT16
   else if id == TypeId::of::<u16>() {
      (uint16, 2, 1)
   } else if id == TypeId::of::<[u16; 2]>() {
      (uint16, 2, 2)
   } else if id == TypeId::of::<[u16; 3]>() {
      (uint16, 2, 3)
   } else if id == TypeId::of::<[u16; 4]>() {
      (uint16, 2, 4)
   }
   // INT32
   else if id == TypeId::of::<i32>() {
      (int32, 4, 1)
   } else if id == TypeId::of::<[i32; 2]>() {
      (int32, 4, 2)
   } else if id == TypeId::of::<[i32; 3]>() {
      (int32, 4, 3)
   } else if id == TypeId::of::<[i32; 4]>() {
      (int32, 4, 4)
   }
   // UINT32
   else if id == TypeId::of::<u32>() {
      (uint32, 4, 1)
   } else if id == TypeId::of::<[u32; 2]>() {
      (uint32, 4, 2)
   } else if id == TypeId::of::<[u32; 3]>() {
      (uint32, 4, 3)
   } else if id == TypeId::of::<[u32; 4]>() {
      (uint32, 4, 4)
   }
   // FLOAT32
   else if id == TypeId::of::<f32>() {
      (float32, 4, 1)
   } else if id == TypeId::of::<[f32; 2]>() {
      (float32, 4, 2)
   } else if id == TypeId::of::<[f32; 3]>() {
      (float32, 4, 3)
   } else if id == TypeId::of::<[f32; 4]>() {
      (float32, 4, 4)
   }
   // FLOAT64
   else if id == TypeId::of::<f64>() {
      (float64, 8, 1)
   } else if id == TypeId::of::<[f64; 2]>() {
      (float64, 8, 2)
   } else if id == TypeId::of::<[f64; 3]>() {
      (float64, 8, 3)
   } else if id == TypeId::of::<[f64; 4]>() {
      (float64, 8, 4)
   } else {
      (uint8, 0, 0)
   }
}
