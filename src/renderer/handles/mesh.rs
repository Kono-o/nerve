use crate::asset::ATTRInfo;
use crate::{ansi, Transform2D};
use crate::{log_info, NEShader, Transform3D};

#[derive(Clone, Debug, Copy)]
pub enum DrawMode {
   Points,
   Lines,
   Triangles,
   Strip,
}

impl Default for DrawMode {
   fn default() -> DrawMode {
      DrawMode::Triangles
   }
}

#[derive(Clone, Debug)]
pub(crate) struct MeshHandle {
   pub(crate) layouts: Vec<(ATTRInfo, u32)>,
   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) ind_count: u32,
   pub(crate) vao_id: u32,
   pub(crate) buf_id: u32,
   pub(crate) ind_id: u32,
}
impl MeshHandle {
   pub(crate) fn log_info(&self) {
      for (attr, loc) in &self.layouts {
         let attr_str = attr.name.as_string();
         let fmt_str = attr.fmt_as_string();
         log_info!("{attr_str} {fmt_str} at {loc}");
      }
      log_info!("vertices: {}", self.vert_count);
      log_info!(
         "indices: {}",
         match self.has_indices {
            true => self.ind_count.to_string(),
            false => "none".to_string(),
         }
      );
   }
}

macro_rules! mesh_struct {
   ($mesh:ident,$transform:ty) => {
      #[derive(Clone, Debug)]
      pub struct $mesh {
         pub(crate) visible: bool,
         pub(crate) handle: MeshHandle,
         pub(crate) draw_mode: DrawMode,
         pub(crate) shader: NEShader,
         pub transform: $transform,
      }

      impl $mesh {
         pub fn set_shader(&mut self, shader: NEShader) {
            self.shader = shader
         }
         pub fn get_draw_mode(&self) -> DrawMode {
            self.draw_mode
         }
         pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
            self.draw_mode = draw_mode
         }

         pub fn index_count(&self) -> u32 {
            self.handle.ind_count
         }
         pub fn vertex_count(&self) -> u32 {
            self.handle.vert_count
         }
         pub fn has_indices(&self) -> bool {
            self.handle.has_indices
         }
         pub fn is_empty(&self) -> bool {
            self.vertex_count() == 0
         }

         pub fn is_renderable(&self) -> bool {
            self.visible || !self.is_empty()
         }

         pub fn set_visibility(&mut self, enable: bool) {
            self.visible = enable;
         }
         pub fn toggle_visibility(&mut self) {
            self.visible = !self.visible;
         }

         pub fn update(&mut self) {
            self.transform.calc_matrix()
         }
         pub fn log_info(&self) {
            self.handle.log_info();
         }
      }
   };
}
mesh_struct!(NEMesh3D, Transform3D);
mesh_struct!(NEMesh2D, Transform2D);
