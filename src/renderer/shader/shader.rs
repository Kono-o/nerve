use cgmath::Matrix4;
use gl::types::GLuint;

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

   pub fn kill(&mut self) {
      self.panic_if_not_compiled();
      unsafe {
         self.bind();
         gl::DeleteProgram(self.id);
         self.is_compiled = false
      }
   }
}
