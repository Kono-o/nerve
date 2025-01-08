use cgmath::Matrix4;

#[derive(Clone, Debug)]
pub struct NEShader {
   pub(crate) id: u32,
   pub(crate) image_ids: Vec<u32>,
   pub(crate) exists_on_gpu: bool,
}

impl NEShader {
   pub(crate) fn temporary() -> NEShader {
      NEShader {
         id: 0,
         image_ids: Vec::new(),
         exists_on_gpu: false,
      }
   }
}

pub enum Uniform {
   Matrix4(Matrix4<f32>),
   Int(i32),
}
