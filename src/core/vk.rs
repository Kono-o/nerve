use crate::renderer::Renderer;
use crate::{Cull, NerveMesh, NerveShader, PolyMode, Uniform, WinSize, RGB};
use cgmath::Matrix4;
use gl::types::{GLenum, GLuint};
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

   //STATE
   fn set_bg_color(&self, color: RGB) {
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
   fn set_wire_width(&self, thickness: f32) {
      todo!()
   }
   fn bind_program(&self, id: GLuint) {
      todo!()
   }
   fn unbind_program(&self) {
      todo!()
   }
   fn bind_texture(&self, tex_id: GLuint) {
      todo!()
   }
   fn unbind_texture(&self) {
      todo!()
   }

   //SHADER
   fn compile_shader(&self, src: &str, typ: GLenum) -> GLuint {
      todo!()
   }
   fn create_program(
      &self,
      vert: &str,
      frag: &str,
      image_ids: Vec<(String, GLuint)>,
   ) -> NerveShader {
      todo!()
   }
   fn set_uni(&self, id: GLuint, name: &str, uniform: Uniform) {
      todo!()
   }

   fn set_uni_m4f32(&self, id: GLuint, name: &str, matrix: Matrix4<f32>) {
      todo!()
   }

   //BUFFERS

   //DRAW
   fn clear(&self) {
      todo!()
   }
   fn draw(&self, mesh: &NerveMesh) {
      todo!()
   }
}
