use crate::{NerveCamera, NerveMesh, NerveShader};

pub trait Scene {
   fn startup(&mut self);
   fn render(&mut self);
   fn kill(&mut self);
}

pub(crate) struct DefaultScene {
   camera: NerveCamera,
   meshes: Vec<NerveMesh>,
   //shaders: Vec<NerveShader>,
}

impl Scene for DefaultScene {
   fn startup(&mut self) {
      let mesh_shader = NerveShader::new(
         "nerve/assets/shaders/vcolor.vert",
         "nerve/assets/shaders/vcolor.frag",
      );

   }

   fn render(&mut self) {
      todo!()
   }

   fn kill(&mut self) {
      todo!()
   }
}
