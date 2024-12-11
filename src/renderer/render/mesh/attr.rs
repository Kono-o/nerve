use gl::types::GLenum;
use std::any::TypeId;
use std::fmt::Display;

macro_rules! attribute {
   ($attr:ident, $typ:ty) => {
      pub struct $attr(pub AttrData<$typ>);
      impl $attr {
         pub fn from(vec: Vec<$typ>) -> $attr {
            $attr(AttrData::Vec(vec))
         }

         pub fn from_array(array: &[$typ]) -> $attr {
            $attr(AttrData::Vec(Vec::from(array)))
         }

         pub fn empty() -> $attr {
            $attr(AttrData::Empty)
         }

         pub fn is_empty(&self) -> bool {
            match self.0 {
               AttrData::Vec(_) => false,
               AttrData::Empty => true,
            }
         }
         pub fn has_data(&self) -> bool {
            match self.0 {
               AttrData::Vec(_) => true,
               AttrData::Empty => false,
            }
         }
         pub fn data(&self) -> (&Vec<$typ>, GLenum, usize, usize) {
            let data = self.0.get().unwrap();
            let (ty, bytes, elements) = get_format(&data[0]);
            (data, ty, bytes, elements)
         }
      }
   };
}

attribute!(PositionAttr, (f32, f32, f32));
attribute!(ColorAttr, (f32, f32, f32));
attribute!(UVMapAttr, (f32, f32));
attribute!(NormalAttr, (f32, f32, f32));
attribute!(Indices, i32);

pub struct CustomAttr<T: DataFormat>(pub AttrData<T>);

impl<D: DataFormat + 'static> CustomAttr<D> {
   pub fn from(vec: Vec<D>) -> CustomAttr<D> {
      CustomAttr(AttrData::Vec(vec))
   }

   pub fn from_array<U: Clone + Display + DataFormat>(array: &[U]) -> CustomAttr<U> {
      let mut vec = Vec::from(array);

      CustomAttr(AttrData::Vec(vec))
   }

   pub fn empty() -> CustomAttr<D> {
      CustomAttr(AttrData::Empty)
   }

   pub fn is_empty(&self) -> bool {
      match self.0 {
         AttrData::Vec(_) => false,
         AttrData::Empty => true,
      }
   }
   pub fn has_data(&self) -> bool {
      match self.0 {
         AttrData::Vec(_) => true,
         AttrData::Empty => false,
      }
   }
   pub fn data(&self) -> (&Vec<D>, GLenum, usize, usize) {
      let data = self.0.get().unwrap();
      let (ty, bytes, elements) = get_format(&data[0]);
      (data, ty, bytes, elements)
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
   } else if id == TypeId::of::<(i8, i8)>() {
      (int8, 1, 3)
   } else if id == TypeId::of::<(i8, i8, i8)>() {
      (int8, 1, 3)
   } else if id == TypeId::of::<(i8, i8, i8, i8)>() {
      (int8, 1, 4)
   }
   // UINT8
   else if id == TypeId::of::<u8>() {
      (uint8, 1, 1)
   } else if id == TypeId::of::<(u8, u8)>() {
      (uint8, 1, 2)
   } else if id == TypeId::of::<(u8, u8, u8)>() {
      (uint8, 1, 3)
   } else if id == TypeId::of::<(u8, u8, u8, u8)>() {
      (uint8, 1, 4)
   }
   // INT16
   else if id == TypeId::of::<i16>() {
      (int16, 2, 1)
   } else if id == TypeId::of::<(i16, i16)>() {
      (int16, 2, 2)
   } else if id == TypeId::of::<(i16, i16, i16)>() {
      (int16, 2, 3)
   } else if id == TypeId::of::<(i16, i16, i16, i16)>() {
      (int16, 2, 4)
   // UINT16
   } else if id == TypeId::of::<u16>() {
      (uint16, 2, 1)
   } else if id == TypeId::of::<(u16, u16)>() {
      (uint16, 2, 2)
   } else if id == TypeId::of::<(u16, u16, u16)>() {
      (uint16, 2, 3)
   } else if id == TypeId::of::<(u16, u16, u16, u16)>() {
      (uint16, 2, 4)
   }
   // INT32
   else if id == TypeId::of::<i32>() {
      (int32, 4, 1)
   } else if id == TypeId::of::<(i32, i32)>() {
      (int32, 4, 2)
   } else if id == TypeId::of::<(i32, i32, i32)>() {
      (int32, 4, 3)
   } else if id == TypeId::of::<(i32, i32, i32, i32)>() {
      (int32, 4, 4)
   }
   // UINT32
   else if id == TypeId::of::<u32>() {
      (uint32, 4, 1)
   } else if id == TypeId::of::<(u32, u32)>() {
      (uint32, 4, 2)
   } else if id == TypeId::of::<(u32, u32, u32)>() {
      (uint32, 4, 3)
   } else if id == TypeId::of::<(u32, u32, u32, u32)>() {
      (uint32, 4, 4)
   }
   // FLOAT32
   else if id == TypeId::of::<f32>() {
      (float32, 4, 1)
   } else if id == TypeId::of::<(f32, f32)>() {
      (float32, 4, 2)
   } else if id == TypeId::of::<(f32, f32, f32)>() {
      (float32, 4, 3)
   } else if id == TypeId::of::<(f32, f32, f32, f32)>() {
      (float32, 4, 4)
   }
   // FLOAT64
   else if id == TypeId::of::<f64>() {
      (float64, 8, 1)
   } else if id == TypeId::of::<(f64, f64)>() {
      (float64, 8, 2)
   } else if id == TypeId::of::<(f64, f64, f64)>() {
      (float64, 8, 3)
   } else if id == TypeId::of::<(f64, f64, f64, f64)>() {
      (float64, 8, 4)
   } else {
      (uint8, 0, 0)
   }
}

pub trait DataFormat {}

macro_rules! data_format {
   ($t: ty) => {
      impl DataFormat for $t {}
   };
}

data_format!(i8);
data_format!((i8, i8));
data_format!((i8, i8, i8));
data_format!((i8, i8, i8, i8));

data_format!(u8);
data_format!((u8, u8));
data_format!((u8, u8, u8));
data_format!((u8, u8, u8, u8));

data_format!(i16);
data_format!((i16, i16));
data_format!((i16, i16, i16));
data_format!((i16, i16, i16, i16));

data_format!(u16);
data_format!((u16, u16));
data_format!((u16, u16, u16));
data_format!((u16, u16, u16, u16));

data_format!(i32);
data_format!((i32, i32));
data_format!((i32, i32, i32));
data_format!((i32, i32, i32, i32));

data_format!(u32);
data_format!((u32, u32));
data_format!((u32, u32, u32));
data_format!((u32, u32, u32, u32));

data_format!(f32);
data_format!((f32, f32));
data_format!((f32, f32, f32));
data_format!((f32, f32, f32, f32));

//data_format!(f64);
//data_format!((f64, f64));
//data_format!((f64, f64, f64));
//data_format!((f64, f64, f64, f64));

pub enum AttrData<T: DataFormat> {
   Empty,
   Vec(Vec<T>),
}

impl<T: DataFormat> AttrData<T> {
   pub(crate) fn get(&self) -> Option<&Vec<T>> {
      match self {
         AttrData::Vec(v) => Some(v),
         _ => None,
      }
   }
}
