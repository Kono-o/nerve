use crate::renderer::NerveShader;
use gl::types::{GLint, GLsizei, GLuint};
use png::{BitDepth, ColorType};
use std::ffi::c_void;
use std::fs::File;

#[derive(Debug)]
pub(crate) struct TexInfo {
   pub(crate) exists: bool,
   pub(crate) width: u32,
   pub(crate) height: u32,
   pub(crate) bit_depth: u8,
   pub(crate) color_type: ColType,
   pub(crate) pixel_size: u32,
}

#[derive(Debug)]
pub(crate) enum ColType {
   GRAY,
   RGB,
   C256,
   GRAYA,
   RGBA,
}
impl TexInfo {
   pub(crate) fn empty() -> TexInfo {
      TexInfo {
         exists: false,
         width: 0,
         height: 0,
         bit_depth: 0,
         color_type: ColType::RGB,
         pixel_size: 0,
      }
   }
   pub(crate) fn pixel_size(&self) -> u32 {
      self.pixel_size
   }
}

pub struct Texture {
   tex: Vec<u8>,
   info: TexInfo,
}

impl Texture {
   pub fn empty() -> Texture {
      Texture {
         tex: Vec::new(),
         info: TexInfo::empty(),
      }
   }
   pub(crate) fn exists(&self) -> bool {
      self.info.exists
   }
   pub fn from(tex_path: &str) -> Texture {
      let mut tex = match File::open(tex_path) {
         Ok(file) => file,
         Err(error) => panic!("{tex_path}: {error}"),
      };
      let decoder = png::Decoder::new(tex);
      let mut reader = decoder.read_info().unwrap();
      let mut tex = vec![0; reader.output_buffer_size()];
      let info = reader.next_frame(&mut tex).unwrap();
      //let bytes = &buffer[..info.buffer_size()];
      //let mut tex = Vec::new();
      //for byte in bytes {
      //   tex.push(*byte);
      //}

      let bit = match info.bit_depth {
         BitDepth::One => 1,
         BitDepth::Two => 2,
         BitDepth::Four => 4,
         BitDepth::Eight => 8,
         BitDepth::Sixteen => 16,
      };
      let (col, elems) = match info.color_type {
         ColorType::Grayscale => (ColType::GRAY, 1),
         ColorType::Rgb => (ColType::RGB, 3),
         ColorType::Indexed => (ColType::C256, 1),
         ColorType::GrayscaleAlpha => (ColType::GRAYA, 2),
         ColorType::Rgba => (ColType::RGBA, 4),
      };
      let mut pixel_size = 0;

      let pixel_size = match col {
         ColType::C256 => panic!("indexed png currently unsupported!"),
         _ => (bit * elems) as u32,
      };

      Texture {
         tex,
         info: TexInfo {
            exists: true,
            width: info.width,
            height: info.height,
            bit_depth: bit,
            color_type: col,
            pixel_size,
         },
      }
   }
}

pub struct NerveShaderBuilder {
   dif_tex: Texture,
   nrm_tex: Texture,
   vert_src: String,
   frag_src: String,
}

impl Default for NerveShaderBuilder {
   fn default() -> Self {
      NerveShaderBuilder::empty()
   }
}
pub enum TexType {
   Diffuse,
   Normal,
}
impl NerveShaderBuilder {
   pub fn empty() -> Self {
      Self {
         dif_tex: Texture::empty(),
         nrm_tex: Texture::empty(),
         vert_src: "".to_string(),
         frag_src: "".to_string(),
      }
   }

   pub fn from(vert_src: &str, frag_src: &str) -> Self {
      NerveShaderBuilder::empty().attach_src(vert_src, frag_src)
   }
   pub fn attach_src(mut self, vert_src: &str, frag_src: &str) -> Self {
      self.vert_src = vert_src.to_string();
      self.frag_src = frag_src.to_string();
      self
   }

   pub fn attach_png(mut self, png_type: TexType, path: &str) -> Self {
      self.attach_tex(png_type, Texture::from(path))
   }

   pub fn attach_tex(mut self, png_type: TexType, tex: Texture) -> Self {
      match png_type {
         TexType::Diffuse => self.dif_tex = tex,
         TexType::Normal => self.nrm_tex = tex,
      }
      self
   }

   pub fn compile(&self) -> NerveShader {
      if self.vert_src.len() == 0 || self.frag_src.len() == 0 {
         NerveShader::default()
      } else {
         let mut image_ids: Vec<(String, GLuint)> = Vec::new();
         let mut tex_id: GLuint = 0;
         if self.dif_tex.exists() {
            let mut base_format = match self.dif_tex.info.color_type {
               ColType::GRAY => gl::RED,
               ColType::RGB => gl::RGB,
               ColType::C256 => gl::RGB,
               ColType::GRAYA => gl::RG,
               ColType::RGBA => gl::RGBA,
            };
            let sized_format = match (base_format, self.dif_tex.info.bit_depth) {
               //16
               (gl::RED, 16) => gl::R16,
               (gl::RG, 16) => gl::RG16,
               (gl::RGB, 16) => gl::RGB16,
               (gl::RGBA, 16) => gl::RGBA16,
               //8
               (gl::RED, _) => gl::R8,
               (gl::RG, _) => gl::RG8,
               (gl::RGB, _) => gl::RGB8,
               (gl::RGBA, _) => gl::RGBA8,
               //fallback
               _ => gl::RGBA8,
            };
            unsafe {
               gl::GenTextures(1, &mut tex_id);
               gl::BindTexture(gl::TEXTURE_2D, tex_id);
               gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
               gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
               gl::TexParameteri(
                  gl::TEXTURE_2D,
                  gl::TEXTURE_MIN_FILTER,
                  gl::LINEAR_MIPMAP_LINEAR as GLint,
               );
               gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
               gl::TexImage2D(
                  gl::TEXTURE_2D,
                  0,
                  sized_format as GLint,
                  self.dif_tex.info.width as GLsizei,
                  self.dif_tex.info.height as GLsizei,
                  0,
                  base_format,
                  gl::UNSIGNED_BYTE,
                  &self.dif_tex.tex[0] as *const u8 as *const c_void,
               );
               gl::GenerateMipmap(gl::TEXTURE_2D);
               //unbind
               gl::BindTexture(gl::TEXTURE_2D, 0);
               image_ids.push(("DIFFUSE".to_string(), tex_id));
            }
         }
         NerveShader::ship(&self.vert_src, &self.frag_src, image_ids)
      }
   }
}
