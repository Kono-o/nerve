use crate::Size2D;
use png::{BitDepth, ColorType};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

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

pub struct NerveTexture {
   pub(crate) bytes: Vec<u8>,
   pub(crate) exists: bool,
   pub(crate) typ: TexFormat,
   pub(crate) filter: TexFilter,
   pub(crate) wrap: TexWrap,
   pub(crate) size: Size2D,
   pub(crate) bit_depth: u8,
   pub(crate) pixel_size: u32,
}

impl NerveTexture {
   pub fn empty() -> NerveTexture {
      NerveTexture {
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

   pub fn from(tex_path: &str, filter: TexFilter, wrap: TexWrap) -> NerveTexture {
      let mut tex = match File::open(tex_path) {
         Ok(file) => file,
         Err(error) => panic!("{tex_path}: {error}"),
      };
      let decoder = png::Decoder::new(tex);
      let mut reader = decoder.read_info().unwrap();
      let mut bytes = vec![0; reader.output_buffer_size()];
      let info = reader.next_frame(&mut bytes).unwrap();
      //let bytes = &buffer[..info.buffer_size()];
      //let mut tex = Vec::new();
      //for byte in bytes {
      //   tex.push(*byte);
      //}

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

      let mut pixel_size = match tex_fmt {
         TexFormat::R(b) => b,
         TexFormat::RG(b) => b * 2,
         TexFormat::RGB(b) => b * 3,
         TexFormat::RGBA(b) => b * 4,
         TexFormat::Palette(_) => panic!("indexed png not supported!"),
      } as u32;

      let size = Size2D::from(info.width, info.height);
      NerveTexture {
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

pub struct NerveShaderSrc {
   pub(crate) textures: Vec<NerveTexture>,
   pub(crate) vert_src: String,
   pub(crate) frag_src: String,
}

impl Default for NerveShaderSrc {
   fn default() -> Self {
      NerveShaderSrc::from(
         "nerve/assets/shaders/mesh/default.vert",
         "nerve/assets/shaders/mesh/default.frag",
      )
      .attach_png(
         "nerve/assets/textures/missing.png",
         TexFilter::Closest,
         TexWrap::Repeat,
      )
   }
}

impl NerveShaderSrc {
   pub fn empty() -> Self {
      Self {
         textures: Vec::new(),
         vert_src: String::new(),
         frag_src: String::new(),
      }
   }

   pub fn from(vert_src: &str, frag_src: &str) -> Self {
      NerveShaderSrc::empty().attach_src(vert_src, frag_src)
   }
   pub fn attach_src(mut self, vert_path: &str, frag_path: &str) -> Self {
      let (vert_src, frag_src) = match (
         PathBuf::from_str(&vert_path).unwrap().exists(),
         PathBuf::from_str(&frag_path).unwrap().exists(),
      ) {
         (true, true) => (
            fs::read_to_string(vert_path).unwrap_or("".to_string()),
            fs::read_to_string(frag_path).unwrap_or("".to_string()),
         ),
         _ => panic!("shader src do not exist!"),
      };
      if vert_src.is_empty() || frag_src.is_empty() {
         panic!("shader is empty!");
      }
      self.vert_src = vert_src;
      self.frag_src = frag_src;
      self
   }

   pub fn attach_png(mut self, path: &str, filter: TexFilter, wrap: TexWrap) -> Self {
      self.attach_tex(NerveTexture::from(path, filter, wrap))
   }

   pub fn attach_tex(mut self, tex: NerveTexture) -> Self {
      self.textures.push(tex);
      self
   }
}
