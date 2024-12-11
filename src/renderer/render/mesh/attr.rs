use gl::types::GLenum;
use std::any::TypeId;

macro_rules! attribute {
   ($attr:ident, $type:ty) => {
      pub struct $attr(pub AttrData<$type>);
      impl $attr {
         pub fn from(vec: Vec<$type>) -> $attr {
            $attr(AttrData::Vec(vec))
         }

         pub fn from_array(array: &[$type]) -> $attr {
            $attr(AttrData::Vec(Vec::from(array)))
         }

         pub fn empty() -> $attr {
            $attr(AttrData::Empty)
         }
      }
   };
}

pub trait DataFormat {}
macro_rules! data_format {
   ($type_n: ident,$type_s: ty) => {
      pub type $type_n = $type_s;
      impl DataFormat for $type_n {}
   };
}

attribute!(PositionAttr, Float32x3);
attribute!(ColorAttr, Float32x3);
attribute!(UVMapAttr, Float32x2);
attribute!(NormalAttr, Float32x3);
attribute!(Indices, Int32);

data_format!(Uint8x2, (u8, u8));
data_format!(Uint8x4, (u8, u8, u8, u8));

data_format!(Int8x2, (i8, i8));
data_format!(Int8x4, (i8, i8, i8, i8));

data_format!(Uint32, u32);
data_format!(Uint32x2, (u32, u32));
data_format!(Uint32x3, (u32, u32, u32));
data_format!(Uint32x4, (u32, u32, u32, u32));

data_format!(Int32, i32);
data_format!(Int32x2, (i32, i32));
data_format!(Int32x3, (i32, i32, i32));
data_format!(Int32x4, (i32, i32, i32, i32));

data_format!(Float32, f32);
data_format!(Float32x2, (f32, f32));
data_format!(Float32x3, (f32, f32, f32));
data_format!(Float32x4, (f32, f32, f32, f32));

data_format!(Float64, f64);
data_format!(Float64x2, (f64, f64));
data_format!(Float64x3, (f64, f64, f64));
data_format!(Float64x4, (f64, f64, f64, f64));

macro_rules! is {
   ($id: expr, $ty: ty) => {
      match id == TypeId::of::<ty>() {
         true => true,
         false => false,
      };
   };
}

// returns (bytes in 1 element, no of elements)
pub(crate) fn get_format<T: DataFormat + 'static>(_t: &T) -> (GLenum, usize, usize) {
   let id = TypeId::of::<T>();

   let int8 = gl::BYTE;
   let uint8 = gl::UNSIGNED_BYTE;

   let int32 = gl::INT;
   let uint32 = gl::UNSIGNED_INT;

   let float32 = gl::FLOAT;
   let float64 = gl::DOUBLE;

   //INT8
   if id == TypeId::of::<Int8x2>() {
      (int8, 1, 2)
   } else if id == TypeId::of::<Int8x4>() {
      (int8, 1, 4)
   }
   //UINT8
   else if id == TypeId::of::<Uint8x2>() {
      (uint8, 1, 2)
   } else if id == TypeId::of::<Uint8x4>() {
      (uint8, 1, 4)
   }
   //INT32
   else if id == TypeId::of::<Int32>() {
      (int32, 4, 1)
   } else if id == TypeId::of::<Int32x2>() {
      (int32, 4, 2)
   } else if id == TypeId::of::<Int32x3>() {
      (int32, 4, 3)
   } else if id == TypeId::of::<Int32x4>() {
      (int32, 4, 4)
   }
   //UINT32
   else if id == TypeId::of::<Uint32>() {
      (uint32, 4, 1)
   } else if id == TypeId::of::<Uint32x2>() {
      (uint32, 4, 2)
   } else if id == TypeId::of::<Uint32x3>() {
      (uint32, 4, 3)
   } else if id == TypeId::of::<Uint32x4>() {
      (uint32, 4, 4)
   }
   //FLOAT32
   else if id == TypeId::of::<Float32>() {
      (float32, 4, 1)
   } else if id == TypeId::of::<Float32x2>() {
      (float32, 4, 2)
   } else if id == TypeId::of::<Float32x3>() {
      (float32, 4, 3)
   } else if id == TypeId::of::<Float32x4>() {
      (float32, 4, 4)
   }
   //FLOAT64
   else if id == TypeId::of::<Float64>() {
      (float64, 8, 1)
   } else if id == TypeId::of::<Float64x2>() {
      (float64, 8, 2)
   } else if id == TypeId::of::<Float64x3>() {
      (float64, 8, 3)
   } else if id == TypeId::of::<Float64x4>() {
      (float64, 8, 4)
   } else {
      (uint8, 0, 0)
   }
}

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
