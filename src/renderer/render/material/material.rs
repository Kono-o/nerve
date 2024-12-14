use std::fs::File;
use std::io::Read;

pub struct Texture {
   tex: Vec<u8>,
   info: TexInfo,
}

#[derive(Debug)]
pub(crate) struct TexInfo {
   pub width: u32,
   pub height: u32,
   exists: bool,
   bit_depth: u8,
   color_type: ColorType,
}

#[derive(Debug)]
pub(crate) enum ColorType {
   GRAY,
   RGB,
   C256,
   GRAYA,
   RGBA,
}
impl TexInfo {
   pub(crate) fn empty() -> TexInfo {
      TexInfo {
         width: 0,
         height: 0,
         exists: false,
         bit_depth: 0,
         color_type: ColorType::RGB,
      }
   }
}

impl Texture {
   pub fn empty() -> Texture {
      Texture {
         tex: Vec::new(),
         info: TexInfo::empty(),
      }
   }
   pub fn from(tex_path: &str) -> Texture {
      let mut tex = match File::open(tex_path) {
         Ok(file) => file,
         Err(error) => panic!("{tex_path}: {error}"),
      };
      let mut buffer = Vec::new();
      tex.read_to_end(&mut buffer).expect("failed to read img!");

      let mut image = Vec::new();
      let png_sign = b"\x89PNG\r\n\x1a\n";
      if !buffer.starts_with(png_sign) {
         panic!("not a valid png!");
      }
      let mut offset = png_sign.len();
      let mut info = TexInfo::empty();

      while offset < buffer.len() {
         let length = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap()) as usize;
         offset += 4;
         let chunk_type = &buffer[offset..offset + 4];
         offset += 4;
         let chunk_data = &buffer[offset..offset + length];
         offset += length + 4;
         match chunk_type {
            b"IHDR" => {
               info.width = u32::from_be_bytes(chunk_data[0..4].try_into().unwrap());
               info.height = u32::from_be_bytes(chunk_data[4..8].try_into().unwrap());
               info.bit_depth = chunk_data[8];
               info.color_type = match chunk_data[9] {
                  0 => ColorType::GRAY,  //grayscale
                  2 => ColorType::RGB,   // rgb
                  3 => ColorType::C256,  // indexed-color (256 palette)
                  4 => ColorType::GRAYA, //grayscale + alpha
                  6 => ColorType::RGBA,  // rgb + alpha
                  _ => panic!("wierd color type!"),
               };
            }
            b"IDAT" => {
               image.extend_from_slice(chunk_data);
            }
            b"IEND" => {
               break;
            }
            _ => {}
         }
      }
      info.exists = true;
      println!("{:?}", info);
      Texture { tex: image, info }
   }
}

pub struct NerveMaterial {
   dif_tex: Texture,
   nrm_tex: Texture,
}

impl Default for NerveMaterial {
   fn default() -> Self {
      Self {
         dif_tex: Texture::empty(),
         nrm_tex: Texture::empty(),
      }
   }
}
impl NerveMaterial {}
