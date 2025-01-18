use crate::Size2D;

#[derive(Debug, Clone)]
pub(crate) enum TexFormat {
   R(u8), //(bit depth)
   RG(u8),
   RGB(u8),
   RGBA(u8),
}

impl TexFormat {
   pub(crate) fn channels(&self) -> u8 {
      match self {
         TexFormat::R(_) => 1,
         TexFormat::RG(_) => 2,
         TexFormat::RGB(_) => 3,
         TexFormat::RGBA(_) => 4,
      }
   }
   pub(crate) fn bit_depth(&self) -> u8 {
      *match self {
         TexFormat::R(bd) => bd,
         TexFormat::RG(bd) => bd,
         TexFormat::RGB(bd) => bd,
         TexFormat::RGBA(bd) => bd,
      }
   }
   pub(crate) fn pixel_size(&self) -> u8 {
      self.channels() * self.bit_depth()
   }
}

#[derive(Debug, Clone, Copy)]
pub enum TexFilter {
   Closest,
   Linear,
}

#[derive(Debug, Clone, Copy)]
pub enum TexWrap {
   Repeat,
   Extend,
   Clip,
}

#[derive(Clone, Debug)]
pub struct NETexture {
   pub(crate) id: u32,
   pub(crate) size: Size2D,
   pub(crate) format: TexFormat,
   pub(crate) filter: TexFilter,
   pub(crate) wrap: TexWrap,
}

impl NETexture {
   pub fn get_size(&self) -> Size2D {
      self.size
   }

   pub fn get_wrap(&self) -> TexWrap {
      self.wrap
   }
   pub fn set_wrap(&mut self, wrap: TexWrap) {
      self.wrap = wrap
   }

   pub fn get_filter(&self) -> TexFilter {
      self.filter
   }
   pub fn set_filter(&mut self, filter: TexFilter) {
      self.filter = filter
   }
}
