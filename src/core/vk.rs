use crate::renderer::Renderer;
use crate::{Cull, PolyMode, WinSize, RGB};
use glfw::{Glfw, PWindow};

#[derive(Copy, Clone)]
pub(crate) struct VKRenderer;

impl Renderer for VKRenderer {
   fn init(&self, _window: &mut PWindow, glfw: &mut Glfw) {
      if glfw.vulkan_supported() {
         println!("vk available!")
      } else {
         println!("vk not available!")
      }
   }

   fn info(&self) -> (String, String, String) {
      todo!()
   }

   fn set_bg_color(&self, color: RGB) {
      todo!()
   }
   fn clear_bg(&self) {
      todo!()
   }
   fn clear_depth(&self) {
      todo!()
   }
   fn resize(&self, size: WinSize) {
      todo!()
   }
   fn poly_mode(&self, mode: PolyMode) {
      todo!()
   }
   fn enable_msaa(&self, enable: bool) {
      todo!()
   }
   fn enable_depth(&self, enable: bool) {
      todo!()
   }
   fn enable_cull(&self, enable: bool) {
      todo!()
   }
   fn set_cull_face(&self, face: Cull) {
      todo!()
   }
   fn wire_thickness(&self, thickness: f32) {
      todo!()
   }
}
