use crate::asset::io::file;
use crate::{NEError, NEResult};
use std::io::{BufRead, BufReader, Read};

pub(crate) enum NEGLSLErrKind {
   IsEmpty,
}

pub(crate) struct NEGLSL {
   pub(crate) v_src: String,
   pub(crate) f_src: String,
}

impl NEGLSL {
   pub(crate) fn load_both_from_disk(v_path: &str, f_path: &str) -> NEResult<NEGLSL> {
      let mut v_file = match file::load_from_disk(v_path) {
         NEResult::OK(vf) => vf,
         NEResult::ER(e) => return NEResult::ER(e),
      };
      let mut f_file = match file::load_from_disk(f_path) {
         NEResult::OK(vf) => vf,
         NEResult::ER(e) => return NEResult::ER(e),
      };
      let mut v_src = String::new();
      let mut f_src = String::new();

      let _ = v_file.read_to_string(&mut v_src);
      let _ = f_file.read_to_string(&mut f_src);

      if v_src.is_empty() {
         return NEResult::ER(NEError::GLSL {
            kind: NEGLSLErrKind::IsEmpty,
            path: v_path.to_string(),
         });
      }
      if f_src.is_empty() {
         return NEResult::ER(NEError::GLSL {
            kind: NEGLSLErrKind::IsEmpty,
            path: f_path.to_string(),
         });
      }
      NEResult::OK(NEGLSL { v_src, f_src })
   }

   pub(crate) fn load_from_disk(path: &str) -> NEResult<NEGLSL> {
      let mut file = match file::load_from_disk(path) {
         NEResult::OK(vf) => vf,
         NEResult::ER(e) => return NEResult::ER(e),
      };

      let mut v_src = String::new();
      let mut f_src = String::new();

      let glsl_src = BufReader::new(file);
      let (mut v_found, mut f_found) = (false, false);

      let mut cur_src: &mut String = &mut v_src;
      for line_res in glsl_src.lines() {
         let line = line_res.unwrap_or("".to_string());
         let trimmed_line = line.trim();
         match trimmed_line {
            "//v" | "//V" | "//vert" | "//VERT" | "//vertex" | "//VERTEX" | "// v" | "// V"
            | "// vert" | "// VERT" | "// vertex" | "// VERTEX" => {
               v_found = true;
               cur_src = &mut v_src
            }
            "//f" | "//F" | "//frag" | "//FRAG" | "//fragment" | "//FRAGMENT" | "// f" | "// F"
            | "// frag" | "// FRAG" | "// fragment" | "// FRAGMENT" => {
               f_found = true;
               cur_src = &mut f_src
            }
            _ => {
               cur_src.push_str(trimmed_line);
               cur_src.push_str("\n")
            }
         }
      }
      if v_src.is_empty() || v_src.is_empty() || !v_found || !f_found {
         return NEResult::ER(NEError::GLSL {
            kind: NEGLSLErrKind::IsEmpty,
            path: path.to_string(),
         });
      }

      NEResult::OK(NEGLSL { v_src, f_src })
   }
}
