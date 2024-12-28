use crate::renderer::{Renderer, ShaderType};
use crate::{Cull, DrawMode, NerveTexture, PolyMode, Size2D, Uniform, RGB};
use cgmath::Matrix4;
use glfw::PWindow;

#[derive(Copy, Clone)]
pub(crate) struct VKRenderer;

impl Renderer for VKRenderer {
   fn init(&self, window: &mut PWindow) {
      if window.glfw.vulkan_supported() {
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
   fn resize(&self, size: Size2D) {
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
   fn bind_program(&self, id: u32) {
      todo!()
   }
   fn unbind_program(&self) {
      todo!()
   }

   fn bind_texture_at_slot(&self, tex_id: u32, slot: u32) {
      todo!()
   }

   fn unbind_texture(&self) {
      todo!()
   }

   fn create_shader(&self, src: &str, typ: ShaderType) -> u32 {
      todo!()
   }

   fn delete_shader(&self, id: u32) {
      todo!()
   }

   fn create_program(&self, vert: &str, frag: &str) -> u32 {
      todo!()
   }

   fn delete_program(&self, id: u32) {
      todo!()
   }

   fn create_texture_at_slot(&self, tex: &NerveTexture, slot: u32) -> u32 {
      todo!()
   }

   fn delete_texture(&self, id: u32) {
      todo!()
   }

   fn get_uni_location(&self, id: u32, name: &str) -> u32 {
      todo!()
   }

   fn set_uni(&self, id: u32, name: &str, uniform: Uniform) {
      todo!()
   }

   fn set_uni_i32(&self, id: u32, name: &str, int: i32) {
      todo!()
   }

   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>) {
      todo!()
   }

   fn clear(&self) {
      todo!()
   }

   fn draw(&self, draw_mode: &DrawMode, index_count: u32) {
      todo!()
   }

   fn draw_no_index(&self, draw_mode: &DrawMode, vert_count: u32) {
      todo!()
   }
}
