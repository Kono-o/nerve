use std::any::TypeId;
use std::mem::size_of;
use gl::types::{GLbyte, GLdouble, GLfloat, GLint};

#[derive(Debug)]
pub(crate) enum Type {
   Null,
   U8,
   I8,
   U32,
   I32,
   F32,
   F64,
}

pub trait DataFormat {}

pub(crate) fn get_format<T: DataFormat + 'static>(_t: &T) -> (Type, usize) {
   return if TypeId::of::<T>() == TypeId::of::<Uint8x2>() {
      (Type::U8, 2 * size_of::<GLbyte>())
   } else if TypeId::of::<T>() == TypeId::of::<Uint8x4>() {
      (Type::U8, 4 * size_of::<GLbyte>())
   } else if TypeId::of::<T>() == TypeId::of::<Int8x2>() {
      (Type::I8, 2 * size_of::<GLbyte>())
   } else if TypeId::of::<T>() == TypeId::of::<Int8x4>() {
      (Type::I8, 4 * size_of::<GLbyte>())
   } else if TypeId::of::<T>() == TypeId::of::<Uint32>() {
      (Type::U32, size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Uint32x2>() {
      (Type::U32, 2 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Uint32x3>() {
      (Type::U32, 3 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Uint32x4>() {
      (Type::U32, 4 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Int32>() {
      (Type::I32, size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Int32x2>() {
      (Type::I32, 2 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Int32x3>() {
      (Type::I32, 3 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Int32x4>() {
      (Type::I32, 4 * size_of::<GLint>())
   } else if TypeId::of::<T>() == TypeId::of::<Float32>() {
      (Type::F32, size_of::<GLfloat>())
   } else if TypeId::of::<T>() == TypeId::of::<Float32x2>() {
      (Type::F32, 2 * size_of::<GLfloat>())
   } else if TypeId::of::<T>() == TypeId::of::<Float32x3>() {
      (Type::F32, 3 * size_of::<GLfloat>())
   } else if TypeId::of::<T>() == TypeId::of::<Float32x4>() {
      (Type::F32, 4 * size_of::<GLfloat>())
   } else if TypeId::of::<T>() == TypeId::of::<Float64>() {
      (Type::F64, size_of::<GLdouble>())
   } else if TypeId::of::<T>() == TypeId::of::<Float64x2>() {
      (Type::F64, 2 * size_of::<GLdouble>())
   } else if TypeId::of::<T>() == TypeId::of::<Float64x3>() {
      (Type::F64, 3 * size_of::<GLdouble>())
   } else if TypeId::of::<T>() == TypeId::of::<Float64x4>() {
      (Type::F64, 4 * size_of::<GLdouble>())
   } else {
      (Type::Null, 0)
   };
}

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

pub enum AttrData<T: DataFormat> {
   Empty,
   Vec(Vec<T>),
}

impl<T: DataFormat> AttrData<T> {
   pub(crate) fn got_data(&self) -> Option<&Vec<T>> {
      match self {
         AttrData::Vec(v) => Some(v),
         _ => None,
      }
   }
}
