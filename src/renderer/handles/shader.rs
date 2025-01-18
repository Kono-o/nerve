use cgmath::Matrix4;

#[derive(Clone, Debug)]
pub struct NEShader {
   pub(crate) id: u32,
}

impl NEShader {
   pub(crate) fn temporary() -> NEShader {
      NEShader { id: 0 }
   }
}

pub enum Uniform {
   Matrix4(Matrix4<f32>),
   Int(i32),
}
