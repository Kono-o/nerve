use crate::{Cull, PolyMode, WinSize, RGB};

pub(crate) trait Renderer {
   fn init(&self, window: &mut glfw::PWindow, glfw: &mut glfw::Glfw);

   //RENDERING
   fn set_bg(&self, color: RGB);
   fn clear_bg(&self);
   fn clear_depth(&self);

   fn resize(&self, size: WinSize);
   fn poly_mode(&self, mode: PolyMode);
   fn enable_msaa(&self, enable: bool);
   fn enable_depth(&self, enable: bool);
   fn enable_cull(&self, enable: bool);
   fn set_cull_face(&self, face: Cull);
   fn wire_thickness(&self, thickness: f32);

   //SHADERS

   //BUFFERS
}
