use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct NerveShader {
   pub(crate) program_id: GLuint,
}

impl Default for NerveShader {
   fn default() -> Self {
      NerveShader::new(
         "nerve/assets/shaders/mesh/default.vert",
         "nerve/assets/shaders/mesh/default.frag",
      )
   }
}

impl NerveShader {
   pub fn new(vert_path: &str, frag_path: &str) -> NerveShader {
      let mut success = gl::FALSE as GLint;
      let mut info_log = Vec::with_capacity(512);

      let mut compile_shader = |path: &str, shader_type: GLenum| {
         if !PathBuf::from_str(path).unwrap().exists() {
            panic!("{path} does not exist!")
         }
         let src = CString::new(std::fs::read_to_string(path).unwrap().as_bytes()).unwrap();
         unsafe {
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());

            gl::CompileShader(shader);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
               gl::GetShaderInfoLog(
                  shader,
                  512,
                  std::ptr::null_mut(),
                  info_log.as_mut_ptr() as *mut GLchar,
               );
               panic!(
                  "failed to compile -> {}: \n{:?}",
                  path,
                  std::str::from_utf8(&info_log)
               );
            }
            return shader;
         }
      };
      let vert_shader = compile_shader(vert_path, gl::VERTEX_SHADER);
      let frag_shader = compile_shader(frag_path, gl::FRAGMENT_SHADER);

      unsafe {
         let program = gl::CreateProgram();
         gl::AttachShader(program, vert_shader);
         gl::AttachShader(program, frag_shader);
         gl::LinkProgram(program);

         gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
         if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
               program,
               512,
               std::ptr::null_mut(),
               info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
               "failed to link {vert_path}, {frag_path}: \n{:?}",
               std::str::from_utf8(&info_log)
            );
         }
         gl::DeleteShader(vert_shader);
         gl::DeleteShader(frag_shader);

         NerveShader {
            program_id: program,
         }
      }
   }
   pub(crate) fn set(&self) {
      unsafe { gl::UseProgram(self.program_id) }
   }

   pub(crate) fn unset(&self) {
      unsafe { gl::UseProgram(0) }
   }

   fn get_uniform_loc(&self, name: &str) -> GLint {
      unsafe {
         let c_name = CString::new(name).unwrap();
         let loc = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
         if loc == -1 {
            panic!("uniform with name {name} does not exist!")
         } else {
            return loc;
         }
      }
   }
   pub(crate) fn set_mat4(&self, u_name: &str, mat4: Matrix4<f32>) {
      unsafe {
         let location = self.get_uniform_loc(u_name);
         gl::UniformMatrix4fv(location, 1, gl::FALSE, mat4.as_ptr());
      }
   }
   pub fn kill(&self) {
      unsafe {
         self.unset();
         gl::DeleteProgram(self.program_id);
      }
   }
}
