use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct NerveShader {
   pub(crate) id: GLuint,
   pub(crate) image_ids: Vec<(String, GLuint)>,
   pub(crate) is_compiled: bool,
}

impl Default for NerveShader {
   fn default() -> NerveShader {
      NerveShader::ship(
         "nerve/assets/shaders/mesh/default.vert",
         "nerve/assets/shaders/mesh/default.frag",
         Vec::new(),
      )
   }
}

pub enum Uniform {
   Matrix4(Matrix4<f32>),
}

impl NerveShader {
   pub fn empty() -> NerveShader {
      NerveShader {
         id: 0,
         image_ids: vec![],
         is_compiled: false,
      }
   }
   pub fn ship(vert_path: &str, frag_path: &str, image_ids: Vec<(String, GLuint)>) -> NerveShader {
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
            id: program,
            image_ids,
            is_compiled: true,
         }
      }
   }
   pub fn is_compiled(&self) -> bool {
      self.is_compiled
   }
   pub(crate) fn panic_if_not_compiled(&self) {
      if !self.is_compiled {
         panic!("shader is not compiled!")
      }
   }
   pub(crate) fn bind(&self) {
      self.panic_if_not_compiled();
      unsafe {
         if self.image_ids.len() > 0 {
            //gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.image_ids[0].1);
         }
         gl::UseProgram(self.id)
      }
   }

   pub(crate) fn unbind(&self) {
      unsafe { gl::UseProgram(0) }
   }

   fn get_uniform_location(&self, name: &str) -> GLint {
      self.panic_if_not_compiled();
      unsafe {
         let c_name = CString::new(name).unwrap();
         let loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
         if loc == -1 {
            panic!("uniform {name} does not exist!")
         } else {
            loc
         }
      }
   }

   pub(crate) fn set_uniform(&self, u_name: &str, uniform: Uniform) {
      self.panic_if_not_compiled();
      let location = self.get_uniform_location(u_name);
      unsafe {
         match uniform {
            Uniform::Matrix4(m) => gl::UniformMatrix4fv(location, 1, gl::FALSE, m.as_ptr()),
         }
      }
   }
   pub fn kill(&mut self) {
      self.panic_if_not_compiled();
      unsafe {
         self.unbind();
         gl::DeleteProgram(self.id);
         self.is_compiled = false
      }
   }
}
