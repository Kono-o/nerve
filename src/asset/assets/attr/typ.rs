use crate::ATTRType;

pub trait DataType {
   const ATTR_FORMAT: ATTRType;
   const BYTE_COUNT: usize;
   const ELEM_COUNT: usize;
   fn u8ify(&self) -> Vec<u8>;
}

macro_rules! u8ify {
   ([$t:ty; $s:literal]) => {
      fn u8ify(&self) -> Vec<u8> {
         let mut vec: Vec<u8> = Vec::new();
         for elem in self.iter() {
            for byte in elem.to_ne_bytes().iter() {
               vec.push(*byte);
            }
         }
         vec
      }
   };
   ($t:ty) => {
      fn u8ify(&self) -> Vec<u8> {
         let mut vec: Vec<u8> = Vec::new();
         for byte in self.to_ne_bytes().iter() {
            vec.push(*byte);
         }
         vec
      }
   };
}
macro_rules! datatype {
   ($type:ty, $attr_format:expr, $byte_count:expr) => {
      impl DataType for $type {
         const ATTR_FORMAT: ATTRType = $attr_format;
         const BYTE_COUNT: usize = $byte_count;
         const ELEM_COUNT: usize = 1;
         u8ify!($type);
      }

      impl DataType for [$type; 2] {
         const ATTR_FORMAT: ATTRType = $attr_format;
         const BYTE_COUNT: usize = $byte_count;
         const ELEM_COUNT: usize = 2;
         u8ify!([$type; 2]);
      }

      impl DataType for [$type; 3] {
         const ATTR_FORMAT: ATTRType = $attr_format;
         const BYTE_COUNT: usize = $byte_count;
         const ELEM_COUNT: usize = 3;
         u8ify!([$type; 3]);
      }

      impl DataType for [$type; 4] {
         const ATTR_FORMAT: ATTRType = $attr_format;
         const BYTE_COUNT: usize = $byte_count;
         const ELEM_COUNT: usize = 4;
         u8ify!([$type; 4]);
      }
   };
}

datatype!(i8, ATTRType::I8, 1);
datatype!(u8, ATTRType::U8, 1);
datatype!(i16, ATTRType::I16, 2);
datatype!(u16, ATTRType::U16, 2);
datatype!(i32, ATTRType::I32, 4);
datatype!(u32, ATTRType::U32, 4);
datatype!(f32, ATTRType::F32, 4);
datatype!(f64, ATTRType::F64, 8);
