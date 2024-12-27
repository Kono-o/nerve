use cgmath::{Matrix, Matrix4};
use gl::types::{GLint, GLuint};
use std::ffi::CString;

#[derive(Clone, Debug)]
pub struct NerveShader {
   pub(crate) id: GLuint,
   pub(crate) image_ids: Vec<(String, GLuint)>,
   pub(crate) is_compiled: bool,
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
