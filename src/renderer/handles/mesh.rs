use crate::{ansi, log_info};
use crate::{NEShader, Transform};
use cgmath::Matrix4;

#[derive(Clone, Debug, Copy)]
pub enum DrawMode {
   Points,
   Lines,
   Triangles,
   Strip,
}

#[derive(Clone, Debug)]
pub struct NEMesh {
   pub alive: bool,
   pub visible: bool,
   pub transform: Transform,
   pub draw_mode: DrawMode,

   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) ind_count: u32,
   pub(crate) buf_id: (u32, u32),
   pub(crate) index_buf_id: u32,
   pub(crate) shader: NEShader,
   pub(crate) layouts: Vec<String>,
}

impl NEMesh {
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
      self.ind_count
   }
   pub fn vertex_count(&self) -> u32 {
      self.vert_count
   }
   pub fn has_indices(&self) -> bool {
      self.has_indices
   }
   pub fn is_empty(&self) -> bool {
      self.vert_count == 0
   }

   pub fn is_renderable(&self) -> bool {
      self.visible || self.alive || !self.is_empty()
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
   pub fn log_info(&self) {
      for attr in self.layouts.clone() {
         log_info!("{}", attr);
      }
      log_info!(
         "life: {}",
         match self.alive {
            true => {
               let vis = match self.visible {
                  true => "visible",
                  false => "hidden",
               };
               format!("ALIVE [{}]", vis)
            }
            false => "DEAD".to_string(),
         }
      );
      log_info!(
         "mode: {}",
         match self.draw_mode {
            DrawMode::Points => "POINTS",
            DrawMode::Lines => "LINES",
            DrawMode::Triangles => "TRIANGLE",
            DrawMode::Strip => "STRIP",
         }
      );
      log_info!("verts: {}", self.vert_count);
      log_info!(
         "index: {}",
         match self.has_indices {
            true => {
               format!("{} (exists)", self.ind_count)
            }
            false => "0, (none)".to_string(),
         }
      );
   }
}