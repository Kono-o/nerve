use std::any::TypeId;

macro_rules! attribute {
   ($attr:ident, $type:ty) => {
      pub struct $attr(pub AttrData<$type>);
      impl $attr {
         pub fn from(vec: Vec<$type>) -> $attr {
            $attr(AttrData::Vec(vec))
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

// returns (bytes in 1 element, no of elements)
pub(crate) fn get_format<T: DataFormat + 'static>(_t: &T) -> (usize, usize) {
   let id = TypeId::of::<T>();
   if id == TypeId::of::<Uint8x2>() {
      (1, 2)
   } else if id == TypeId::of::<Uint8x4>() {
      (1, 4)
   } else if id == TypeId::of::<Int8x2>() {
      (1, 2)
   } else if id == TypeId::of::<Int8x4>() {
      (1, 4)
   } else if id == TypeId::of::<Uint32>() {
      (4, 1)
   } else if id == TypeId::of::<Uint32x2>() {
      (4, 2)
   } else if id == TypeId::of::<Uint32x3>() {
      (4, 3)
   } else if id == TypeId::of::<Uint32x4>() {
      (4, 4)
   } else if id == TypeId::of::<Int32>() {
      (4, 1)
   } else if id == TypeId::of::<Int32x2>() {
      (4, 2)
   } else if id == TypeId::of::<Int32x3>() {
      (4, 3)
   } else if id == TypeId::of::<Int32x4>() {
      (4, 4)
   } else if id == TypeId::of::<Float32>() {
      (4, 1)
   } else if id == TypeId::of::<Float32x2>() {
      (4, 2)
   } else if id == TypeId::of::<Float32x3>() {
      (4, 3)
   } else if id == TypeId::of::<Float32x4>() {
      (4, 4)
   } else if id == TypeId::of::<Float64>() {
      (8, 1)
   } else if id == TypeId::of::<Float64x2>() {
      (8, 2)
   } else if id == TypeId::of::<Float64x3>() {
      (8, 3)
   } else if id == TypeId::of::<Float64x4>() {
      (8, 4)
   } else {
      (0, 0)
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
