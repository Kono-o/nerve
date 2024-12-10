use std::any::TypeId;

pub trait DataFormat {}

pub type Uint8x2 = (u8, u8);
impl DataFormat for Uint8x2 {}
pub type Uint8x4 = (u8, u8, u8, u8);
impl DataFormat for Uint8x4 {}

pub type Int8x2 = (i8, i8);
impl DataFormat for Int8x2 {}
pub type Int8x4 = (i8, i8, i8, i8);
impl DataFormat for Int8x4 {}

pub type Uint32 = u32;
impl DataFormat for Uint32 {}
pub type Uint32x2 = (u32, u32);
impl DataFormat for Uint32x2 {}
pub type Uint32x3 = (u32, u32, u32);
impl DataFormat for Uint32x3 {}
pub type Uint32x4 = (u32, u32, u32, u32);
impl DataFormat for Uint32x4 {}

pub type Int32 = i32;
impl DataFormat for Int32 {}
pub type Int32x2 = (i32, i32);
impl DataFormat for Int32x2 {}
pub type Int32x3 = (i32, i32, i32);
impl DataFormat for Int32x3 {}
pub type Int32x4 = (i32, i32, i32, i32);
impl DataFormat for Int32x4 {}

pub type Float32 = f32;
impl DataFormat for Float32 {}
pub type Float32x2 = (f32, f32);
impl DataFormat for Float32x2 {}
pub type Float32x3 = (f32, f32, f32);
impl DataFormat for Float32x3 {}
pub type Float32x4 = (f32, f32, f32, f32);
impl DataFormat for Float32x4 {}

pub type Float64 = f64;
impl DataFormat for Float64 {}
pub type Float64x2 = (f64, f64);
impl DataFormat for Float64x2 {}
pub type Float64x3 = (f64, f64, f64);
impl DataFormat for Float64x3 {}
pub type Float64x4 = (f64, f64, f64, f64);
impl DataFormat for Float64x4 {}

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
