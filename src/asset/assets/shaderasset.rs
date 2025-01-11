use crate::*;

pub(crate) enum NEAssetErrKind {
   NonTriangleMesh,
   VertEmpty,
   FragEmpty,
}

enum GLSL {
   Parsed { v_src: String, f_src: String },
   CouldntParse { v_missing: bool, f_missing: bool },
}
impl GLSL {
   fn parse(src: &str) -> GLSL {
      let mut v_src = String::new();
      let mut f_src = String::new();

      let glsl_lines = src.lines();

      let (mut v_found, mut f_found) = (false, false);
      let mut cur_src = &mut v_src;

      for line in glsl_lines {
         let line = line.trim();
         match line {
            "//v" | "//V" | "//vert" | "//VERT" | "//vertex" | "//VERTEX" | "// v" | "// V"
            | "// vert" | "// VERT" | "// vertex" | "// VERTEX" => {
               cur_src = &mut v_src;
               v_found = true;
            }
            "//f" | "//F" | "//frag" | "//FRAG" | "//fragment" | "//FRAGMENT" | "// f" | "// F"
            | "// frag" | "// FRAG" | "// fragment" | "// FRAGMENT" => {
               cur_src = &mut f_src;
               f_found = true;
            }
            _ => {
               cur_src.push_str(line);
               cur_src.push_str("\n")
            }
         }
      }
      let (mut v_missing, mut f_missing) = (false, false);
      if v_src.is_empty() || !v_found {
         v_missing = true
      }
      if f_src.is_empty() || !f_found {
         f_missing = true
      }

      match v_missing || f_missing {
         true => GLSL::CouldntParse {
            v_missing,
            f_missing,
         },
         false => GLSL::Parsed { v_src, f_src },
      }
   }
}

pub struct NEShaderAsset {
   pub(crate) path: String,
   pub(crate) v_spv: Vec<u8>,
   pub(crate) f_spv: Vec<u8>,
}

impl NEShaderAsset {
   pub(crate) fn fallback() -> NEResult<NEShaderAsset> {
      NEShaderAsset::from_path("nerve/assets/glsl/fallback.glsl")
   }
   pub fn from_path(path: &str) -> NEResult<NEShaderAsset> {
      let file_name = match file::name(path) {
         NEOption::Empty => return NEResult::ER(NEError::file_invalid(path)),
         NEOption::Exists(n) => n,
      };
      let _ = match file::ex(path) {
         NEOption::Empty => return NEResult::ER(NEError::file_invalid(path)),
         NEOption::Exists(ex) => match ex.eq_ignore_ascii_case(ex::GLSL) {
            false => return NEResult::ER(NEError::file_unsupported(path, &ex)),
            true => ex,
         },
      };
      let nshdr_path = format!("{}{}.{}", path::SHDR_ASSET, file_name, ex::NSHDR);

      let file_exists = file::exists_on_disk(path);
      let nshdr_exists = file::exists_on_disk(&nshdr_path);

      if !file_exists && !nshdr_exists {
         let both_paths = format!("{} or {}", path, nshdr_path);
         return NEResult::ER(NEError::file_missing(&both_paths));
      }
      if file_exists {
         //write/overwrite nshdr
         let src = match file::read_as_string(path) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(s) => s,
         };
         let glsl = GLSL::parse(&src);
         match glsl {
            GLSL::CouldntParse {
               v_missing,
               f_missing,
            } => {
               return NEResult::ER(match (v_missing, f_missing) {
                  (true, _) => NEError::vert_missing(path),
                  (_, _) => NEError::frag_missing(path),
               })
            }

            GLSL::Parsed { v_src, f_src } => {
               let v_spv = match glsl_to_spv(&file_name, ShaderType::Vert, &v_src) {
                  NEResult::ER(e) => return NEResult::ER(e),
                  NEResult::OK(s) => s,
               };
               let f_spv = match glsl_to_spv(&file_name, ShaderType::Frag, &f_src) {
                  NEResult::ER(e) => return NEResult::ER(e),
                  NEResult::OK(s) => s,
               };

               let mut nshdr = Vec::new();

               let stride = 4;
               let v_spv_len = u32_to_vec_of_4_u8s(v_spv.len() as u32);
               let f_spv_len = u32_to_vec_of_4_u8s(f_spv.len() as u32);

               nshdr.extend_from_slice(&v_spv_len); //bytes 0 to 3 are the size of v_spv
               nshdr.extend_from_slice(&f_spv_len); //bytes 4 to 7 are the size of f_spv
               nshdr.extend_from_slice(&v_spv);
               nshdr.extend_from_slice(&f_spv);

               let nshdr_name = format!("{file_name}.{}", ex::NSHDR);
               match file::write_bytes_to_disk(path::SHDR_ASSET, &nshdr_name, &nshdr) {
                  NEResult::ER(e) => NEResult::ER(e),
                  _ => NEResult::OK(NEShaderAsset {
                     path: nshdr_path.clone(),
                     v_spv,
                     f_spv,
                  }),
               }
            }
         }
      } else {
         //load new/pre-existing nshdr
         let nshdr = match file::read_as_bytes(&nshdr_path) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(f) => f,
         };
         let stride = 4;
         let stride_x_2 = stride + stride;
         let v_bin_len = u32::from_ne_bytes(clone_slice_4(&nshdr[0..stride])) as usize;
         let f_bin_len = u32::from_ne_bytes(clone_slice_4(&nshdr[stride..stride_x_2])) as usize;

         let v_offset = stride_x_2 + v_bin_len;
         let f_offset = v_offset + f_bin_len;

         let v_spv = clone_slice(&nshdr[8..v_offset]);
         let f_spv = clone_slice(&nshdr[v_offset..f_offset]);

         NEResult::OK(NEShaderAsset {
            path: nshdr_path,
            v_spv,
            f_spv,
         })
      }
   }
}

fn clone_slice_4(bytes: &[u8]) -> [u8; 4] {
   let mut cloned_bytes = [0; 4];
   for i in 0..4 {
      cloned_bytes[i] = bytes[i]
   }
   cloned_bytes
}
fn clone_slice(bytes: &[u8]) -> Vec<u8> {
   let mut cloned_bytes = Vec::new();
   for byte in bytes {
      cloned_bytes.push(*byte)
   }
   cloned_bytes
}
fn u32_to_vec_of_4_u8s(n: u32) -> Vec<u8> {
   let mut vec = Vec::new();
   let bytes = n.u8ify();
   for i in 0..4 {
      if bytes.len() > i {
         vec.push(bytes[i])
      } else {
         vec.push(0)
      }
   }
   vec
}

fn glsl_to_spv(name: &str, typ: ShaderType, src: &str) -> NEResult<Vec<u8>> {
   let temp_path = path::TEMP;
   let ex = match typ {
      ShaderType::Vert => ex::VERT,
      ShaderType::Frag => ex::FRAG,
   };
   let name_ex = format!("{name}.{ex}");
   match file::write_str_to_disk(temp_path, &name_ex, &src) {
      NEResult::ER(e) => return NEResult::ER(e),
      _ => {}
   };
   let temp_file = format!("{temp_path}{name_ex}");
   let spv_file = format!("{temp_file}.{}", ex::NSHDR);
   gen_spv_from_glsl_to_path(&temp_file, &spv_file)
}
fn gen_spv_from_glsl_to_path(glsl_file: &str, spv_file: &str) -> NEResult<Vec<u8>> {
   let o = std::process::Command::new(env::GLSL_VALIDATOR_PATH)
      .arg("-G")
      //.arg("-Os")
      //.arg("-r")
      .arg(glsl_file)
      .arg("-o")
      .arg(spv_file)
      .output();
   match o {
      Ok(out) => {
         if !out.status.success() {
            NEResult::ER(NEError::Compile {
               msg: String::from_utf8_lossy(&out.stdout).to_string(),
               kind: NECompileErrKind::GLSLCompileFailed,
               path: glsl_file.to_string(),
            })
         } else {
            match file::read_as_bytes(spv_file) {
               NEResult::OK(spv) => NEResult::OK(spv),
               NEResult::ER(e) => NEResult::ER(e),
            }
         }
      }
      Err(_) => NEResult::ER(NEError::Compile {
         kind: NECompileErrKind::NoGLSLValidator,
         path: "".to_string(),
         msg: "".to_string(),
      }),
   }
}
