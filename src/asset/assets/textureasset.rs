use crate::Size2D;
use png::{BitDepth, ColorType};
use std::fs::File;

#[derive(Debug, Clone, Copy)]
pub(crate) enum TexFormat {
   R(u8), //(bit depth)
   RG(u8),
   RGB(u8),
   RGBA(u8),
   Palette(u8),
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

impl TexFormat {
   pub(crate) fn bit_depth(&self) -> u8 {
      *match self {
         TexFormat::R(b) => b,
         TexFormat::RG(b) => b,
         TexFormat::RGB(b) => b,
         TexFormat::RGBA(b) => b,
         TexFormat::Palette(b) => b,
      }
   }
   pub(crate) fn elem_count(&self) -> u8 {
      match self {
         TexFormat::R(_) => 1,
         TexFormat::RG(_) => 2,
         TexFormat::RGB(_) => 3,
         TexFormat::RGBA(_) => 4,
         TexFormat::Palette(_) => 3,
      }
   }
}

pub struct NETexture {
   pub(crate) exists: bool,
   pub(crate) bytes: Vec<u8>,
   pub(crate) bit_depth: u8,
   pub(crate) pixel_size: u8,

   pub(crate) typ: TexFormat,
   pub(crate) filter: TexFilter,
   pub(crate) wrap: TexWrap,
   pub(crate) size: Size2D,
}

impl NETexture {
   pub fn empty() -> NETexture {
      NETexture {
         bytes: Vec::new(),
         exists: false,
         typ: TexFormat::RGB(8),
         filter: TexFilter::Closest,
         wrap: TexWrap::Repeat,
         size: Size2D::empty(),
         bit_depth: 0,
         pixel_size: 0,
      }
   }

   pub fn from(tex_path: &str, filter: TexFilter, wrap: TexWrap) -> NETexture {
      let mut tex = match File::open(tex_path) {
         Ok(file) => file,
         Err(error) => panic!("{tex_path}: {error}"),
      };
      let decoder = png::Decoder::new(tex);
      let mut reader = decoder.read_info().unwrap();
      let mut bytes = vec![0; reader.output_buffer_size()];
      let info = reader.next_frame(&mut bytes).unwrap();

      let bit_depth = match info.bit_depth {
         BitDepth::One => 1,
         BitDepth::Two => 2,
         BitDepth::Four => 4,
         BitDepth::Eight => 8,
         BitDepth::Sixteen => 16,
      };

      let tex_fmt = match info.color_type {
         ColorType::Grayscale => TexFormat::R(bit_depth),
         ColorType::GrayscaleAlpha => TexFormat::RG(bit_depth),
         ColorType::Rgb => TexFormat::RGB(bit_depth),
         ColorType::Indexed => TexFormat::Palette(bit_depth),
         ColorType::Rgba => TexFormat::RGBA(bit_depth),
      };

      let mut pixel_size = tex_fmt.elem_count() * bit_depth;

      let size = Size2D::from(info.width, info.height);
      NETexture {
         bytes,
         exists: true,
         size,
         bit_depth,
         typ: tex_fmt,
         pixel_size,
         filter,
         wrap,
      }
   }
}
