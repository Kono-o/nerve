use crate::{NEShader, Transform};
use cgmath::Matrix4;

pub enum DrawMode {
   Points,
   Lines,
   Triangles,
   Strip,
}

pub struct NEMesh {
   pub visible: bool,
   pub transform: Transform,
   pub draw_mode: DrawMode,

   pub(crate) alive: bool,
   pub(crate) has_indices: bool,
   pub(crate) is_empty: bool,
   pub(crate) vert_count: u32,
   pub(crate) ind_count: u32,
   pub(crate) buf_id: (u32, u32),
   pub(crate) index_buf_id: u32,
   pub(crate) shader: NEShader,
   pub(crate) layouts: Vec<String>,
}

impl Default for NEMesh {
   fn default() -> Self {
      Self {
         visible: true,
         alive: true,
         transform: Transform::default(),
         has_indices: false,
         is_empty: true,
         vert_count: 0,
         ind_count: 0,
         buf_id: (0, 0),
         index_buf_id: 0,
         shader: NEShader::empty(),
         draw_mode: DrawMode::Triangles,
         layouts: vec![],
      }
   }
}
impl NEMesh {
   pub fn set_shader(&mut self, shader: NEShader) {
      self.shader = shader
   }
   pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
      self.draw_mode = draw_mode
   }
   pub fn set_visibility(&mut self, enable: bool) {
      self.visible = enable;
   }
   pub fn toggle_visibility(&mut self) {
      self.visible = !self.visible;
   }

   pub(crate) fn matrix(&self) -> Matrix4<f32> {
      self.transform.matrix
   }
   pub(crate) fn update(&mut self) {
      self.transform.calc_matrix()
   }
   pub fn display_layouts(&self) {
      for attr in self.layouts.clone() {
         println!("{}", attr);
      }
   }
   pub fn kill(&mut self) {
      self.alive = false;
      //self.vert_object.delete();
      //self.index_object.delete();
   }
}
