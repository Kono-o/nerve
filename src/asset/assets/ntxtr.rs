use crate::asset::file;
use crate::renderer::TexFormat;
use crate::util::{ex, path};
use crate::{env, NEError, NEOption, NEResult, Size2D, TexFilter, TexWrap};
use flate2::read::ZlibDecoder;
use std::io::Read;

enum PNG {
   Parsed {
      bytes: Vec<u8>,
      size: Size2D,
      fmt: TexFormat,
   },
   Failed(String),
}

trait BytesToU32 {
   fn to_u32(self) -> u32;
}

impl BytesToU32 for &[u8] {
   fn to_u32(self) -> u32 {
      let bytes = self.try_into().unwrap();
      u32::from_be_bytes(bytes)
   }
}

impl PNG {
   fn parse(src: &[u8]) -> PNG {
      let len = src.len();
      let sign = &src[0..8];
      if len < 8 || sign != &[137, 80, 78, 71, 13, 10, 26, 10] {
         return PNG::Failed("invalid PNG signature".to_string());
      }
      let mut idat = Vec::new();
      let mut has_ihdr = false;

      let mut size = Size2D::empty();
      let mut bit_depth = 0;
      let mut fmt = TexFormat::RGBA(16);

      let mut offset = 8;
      while offset < len {
         let chunk_len = src[offset..offset + 4].to_u32() as usize;
         offset += 4;

         if offset + 8 + chunk_len > len {
            return PNG::Failed("chunk size exceeds file size".to_string());
         }

         let chunk_type = match String::from_utf8(src[offset..offset + 4].to_vec()) {
            Err(_) => return PNG::Failed("invalid chunk name".to_string()),
            Ok(s) => s,
         };
         offset += 4;

         let chunk_data = &src[offset..offset + chunk_len];
         offset += chunk_len;
         offset += 4; //crc

         match chunk_type.as_str() {
            "IHDR" => {
               if !has_ihdr && chunk_data.len() == 13 {
                  let width = chunk_data[0..4].to_u32();
                  let height = chunk_data[4..8].to_u32();

                  if width == 0 || height == 0 {
                     return PNG::Failed(format!("invalid size w {} h {}", width, height));
                  }
                  size = Size2D::from(width, height);

                  let valid_col_depths;
                  bit_depth = chunk_data[8];
                  (fmt, valid_col_depths) = match chunk_data[9] {
                     0 => (TexFormat::R(bit_depth), vec![1, 2, 4, 8, 16]),
                     2 => (TexFormat::RGB(bit_depth), vec![8, 16]),
                     4 => (TexFormat::RG(bit_depth), vec![8, 16]),
                     6 => (TexFormat::RGBA(bit_depth), vec![8, 16]),
                     3 => return PNG::Failed("palette indexed png unsupported".to_string()),
                     _ => return PNG::Failed("invalid color type".to_string()),
                  };
                  //let compression = chunk_data[10];
                  //let filter = chunk_data[11];
                  //let interlace = chunk_data[12];

                  if !valid_col_depths.contains(&bit_depth) {
                     return PNG::Failed("invalid bit depth".to_string());
                  }
                  has_ihdr = true;
               } else {
                  return PNG::Failed("invalid IHDR chunk".to_string());
               }
            }
            "IDAT" => idat.extend_from_slice(chunk_data),
            "IEND" => break,
            _ => {}
         }
      }
      if !has_ihdr {
         return PNG::Failed("no IHDR".to_string());
      }
      if idat.is_empty() {
         return PNG::Failed("no IDAT".to_string());
      }

      let mut bytes = Vec::new();
      let mut zlib_decoder = ZlibDecoder::new(&idat[..]);
      match zlib_decoder.read_to_end(&mut bytes) {
         Err(e) => return PNG::Failed("zlib decompress failed".to_string()),
         _ => {}
      };
      PNG::Parsed { bytes, size, fmt }
   }
}

pub struct NETexAsset {
   pub(crate) bytes: Vec<u8>,
   pub(crate) size: Size2D,
   pub(crate) fmt: TexFormat,

   pub(crate) filter: TexFilter,
   pub(crate) wrap: TexWrap,
}

impl NETexAsset {
   pub fn from_path(path: &str) -> NEResult<NETexAsset> {
      NETexAsset::from_path_raw(&env::concat_with_asset(path))
   }

   fn from_path_raw(raw_path: &str) -> NEResult<NETexAsset> {
      let file_name = match file::name(raw_path) {
         NEOption::Empty => return NEError::file_invalid(raw_path).pack(),
         NEOption::Exists(n) => n,
      };
      let _ = match file::ex(raw_path) {
         NEOption::Empty => return NEError::file_invalid(raw_path).pack(),
         NEOption::Exists(ex) => match ex.eq_ignore_ascii_case(ex::PNG) {
            false => return NEError::file_unsupported(raw_path, &ex).pack(),
            true => ex,
         },
      };
      let ntxtr_path = format!("{}{}.{}", path::TXTR_ASSET, file_name, ex::NTXTR);

      let file_exists = file::exists_on_disk(raw_path);
      let ntxtr_exists = file::exists_on_disk(&ntxtr_path);

      if !file_exists && !ntxtr_exists {
         let both_paths = format!("{} or {}", raw_path, ntxtr_path);
         return NEError::file_missing(&both_paths).pack();
      }
      //if file_exists {
      //write/overwrite ntxtr
      let tex_src = match file::read_as_bytes(raw_path) {
         NEResult::ER(e) => return e.pack(),
         NEResult::OK(tf) => tf,
      };
      let ntxtr = match PNG::parse(&tex_src) {
         PNG::Failed(msg) => return NEError::png_invalid(raw_path, msg).pack(),
         PNG::Parsed { bytes, size, fmt } => NETexAsset {
            bytes,
            size,
            fmt,
            filter: TexFilter::Linear,
            wrap: TexWrap::Repeat,
         },
      };
      NEResult::OK(ntxtr)

      //let ntxtr_name = format!("{file_name}.{}", ex::NTXTR);
      //match file::write_bytes_to_disk(path::TXTR_ASSET, &ntxtr_name, &tex_src) {
      //   NEResult::ER(e) => e.pack(),
      //   _ => NEResult::OK(ntxtr),
      //}
      //} else {
      //   //load new/pre-existing ntxtr
      //}
   }

   pub fn set_wrap(&mut self, wrap: TexWrap) {
      self.wrap = wrap
   }
   pub fn set_filter(&mut self, filter: TexFilter) {
      self.filter = filter
   }
}
