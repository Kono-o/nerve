use crate::asset::file;
use crate::renderer::TexFormat;
use crate::util::{ex, path};
use crate::{env, NEError, NEOption, NEResult, Size2D, TexFilter, TexWrap};
use flate2::read::ZlibDecoder;
use std::io::Read;

enum PNG {
   Parsed {
      bytes: Vec<u8>,
      w_bytes: Vec<u8>,
      h_bytes: Vec<u8>,
      size: Size2D,
      fmt: TexFormat,
   },
   Failed(String),
}

trait BytesToU32 {
   fn u32ify(self) -> u32;
}

impl BytesToU32 for &[u8] {
   fn u32ify(self) -> u32 {
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

      let mut bit_depth = 0;
      let mut fmt = TexFormat::RGBA(16);
      let (mut w_bytes, mut h_bytes) = (Vec::new(), Vec::new());

      let mut offset = 8;
      while offset < len {
         let chunk_len = src[offset..offset + 4].u32ify() as usize;
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
                  w_bytes = chunk_data[0..4].to_vec();
                  h_bytes = chunk_data[4..8].to_vec();

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
                  let compression = chunk_data[10];
                  let filter = chunk_data[11];
                  //let interlace = chunk_data[12];

                  if compression != 0 {
                     return PNG::Failed("invalid compression".to_string());
                  }
                  if filter != 0 {
                     return PNG::Failed("invalid filtering".to_string());
                  }
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
         Err(_) => return PNG::Failed("zlib decompress failed".to_string()),
         _ => {}
      };
      let size = Size2D::from(w_bytes.u32ify(), h_bytes.u32ify());
      let bytes = match bytes.unfiltered(&size, &fmt) {
         None => return PNG::Failed("invalid filter type".to_string()),
         Some(ub) => ub,
      };

      PNG::Parsed {
         bytes,
         w_bytes,
         h_bytes,
         size,
         fmt,
      }
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
   pub(crate) fn fallback() -> NEResult<NETexAsset> {
      NETexAsset::from_path_raw("nerve/assets/txtr/fallback.png")
   }

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
      if file_exists {
         //write/overwrite ntxtr
         let tex_src = match file::read_as_bytes(raw_path) {
            NEResult::ER(e) => return e.pack(),
            NEResult::OK(tf) => tf,
         };
         let (bytes, w_bytes, h_bytes, size, fmt) = match PNG::parse(&tex_src) {
            PNG::Failed(msg) => return NEError::png_invalid(raw_path, msg).pack(),
            PNG::Parsed {
               bytes,
               w_bytes,
               h_bytes,
               size,
               fmt,
            } => (bytes, w_bytes, h_bytes, size, fmt),
         };

         let mut ntxtr = Vec::new();
         ntxtr.push(fmt.channels());
         ntxtr.push(fmt.bit_depth());
         ntxtr.extend_from_slice(&w_bytes);
         ntxtr.extend_from_slice(&h_bytes);
         ntxtr.extend_from_slice(&bytes);

         let ntxtr_name = format!("{file_name}.{}", ex::NTXTR);
         match file::write_bytes_to_disk(path::TXTR_ASSET, &ntxtr_name, &ntxtr) {
            NEResult::ER(e) => e.pack(),
            _ => NEResult::OK(NETexAsset {
               bytes,
               size,
               fmt,
               filter: TexFilter::Linear,
               wrap: TexWrap::Clip,
            }),
         }
      } else {
         //load new/pre-existing ntxtr
         let ntxtr = match file::read_as_bytes(&ntxtr_path) {
            NEResult::ER(e) => return e.pack(),
            NEResult::OK(f) => f,
         };
         let channels = ntxtr[0];
         let bit_depth = ntxtr[1];
         let w_bytes = &ntxtr[2..6];
         let h_bytes = &ntxtr[6..10];
         let bytes = ntxtr[10..].to_vec();

         let fmt = TexFormat::from(channels, bit_depth);
         let size = Size2D::from(w_bytes.u32ify(), h_bytes.u32ify());

         NEResult::OK(NETexAsset {
            bytes,
            size,
            fmt,
            filter: TexFilter::Linear,
            wrap: TexWrap::Clip,
         })
      }
   }

   pub fn set_wrap(&mut self, wrap: TexWrap) {
      self.wrap = wrap
   }
   pub fn set_filter(&mut self, filter: TexFilter) {
      self.filter = filter
   }
}

trait UnFilter {
   fn unfiltered(self, size: &Size2D, fmt: &TexFormat) -> Option<Vec<u8>>;
}

impl UnFilter for Vec<u8> {
   fn unfiltered(self, size: &Size2D, fmt: &TexFormat) -> Option<Vec<u8>> {
      let bpp = ((fmt.bit_depth() * fmt.channels() + 7) / 8) as usize;

      let mut unfiltered = Vec::with_capacity(self.len());
      let bytes_per_scanline = 1 + size.w as usize * bpp;

      for y in 0..size.h as usize {
         let start = y * bytes_per_scanline;
         let filter_type = self[start];
         let raw_scanline = &self[start + 1..start + 1 + size.w as usize * bpp];
         let prev_scanline = if y == 0 {
            None
         } else {
            Some(&unfiltered[(y - 1) * size.w as usize * bpp..y * size.w as usize * bpp])
         };

         let mut unfiltered_scanline = vec![0u8; (size.w as usize * bpp)];
         match filter_type {
            0 => unfiltered_scanline.copy_from_slice(raw_scanline),
            1 => {
               for i in 0..raw_scanline.len() {
                  let left = if i >= bpp {
                     unfiltered_scanline[i - bpp]
                  } else {
                     0
                  };
                  unfiltered_scanline[i] = raw_scanline[i].wrapping_add(left);
               }
            }
            2 => {
               for i in 0..raw_scanline.len() {
                  let above = if let Some(prev) = prev_scanline {
                     prev[i]
                  } else {
                     0
                  };
                  unfiltered_scanline[i] = raw_scanline[i].wrapping_add(above);
               }
            }
            3 => {
               for i in 0..raw_scanline.len() {
                  let left = if i >= bpp {
                     unfiltered_scanline[i - bpp]
                  } else {
                     0
                  };
                  let above = if let Some(prev) = prev_scanline {
                     prev[i]
                  } else {
                     0
                  };
                  unfiltered_scanline[i] =
                     raw_scanline[i].wrapping_add((left as u16 + above as u16) as u8 / 2);
               }
            }
            4 => {
               for i in 0..raw_scanline.len() {
                  let left = if i >= bpp {
                     unfiltered_scanline[i - bpp]
                  } else {
                     0
                  };
                  let above = if let Some(prev) = prev_scanline {
                     prev[i]
                  } else {
                     0
                  };
                  let upper_left = if i >= bpp && prev_scanline.is_some() {
                     prev_scanline.unwrap()[i - bpp]
                  } else {
                     0
                  };
                  unfiltered_scanline[i] =
                     raw_scanline[i].wrapping_add(paeth(left, above, upper_left));
               }
            }
            _ => return None,
         }

         unfiltered.extend_from_slice(&unfiltered_scanline);
      }
      Some(unfiltered)
   }
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
   let p = a as i16 + b as i16 - c as i16;
   let pa = (p - a as i16).abs();
   let pb = (p - b as i16).abs();
   let pc = (p - c as i16).abs();
   if pa <= pb && pa <= pc {
      a
   } else if pb <= pc {
      b
   } else {
      c
   }
}
