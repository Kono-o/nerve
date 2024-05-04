use cgmath::{Deg, Matrix4, ortho, perspective, Vector3};
use crate::renderer::Transform;

pub struct ClipDist {
   pub near: f32,
   pub far: f32,
}

pub enum CamProj {
   Orthographic,
   Perspective,
}

pub struct NerveCamera {
   pub(crate) size: (u32, u32),
   pub(crate) projection: CamProj,
   pub(crate) fov: f32,
   pub(crate) clip: (f32, f32),

   pub(crate) proj_matrix: Matrix4<f32>,
   pub(crate) view_matrix: Matrix4<f32>,
   pub transform: Transform,
}

impl NerveCamera {
   pub(crate) fn recalc_proj(&mut self) {
      self.proj_matrix = match self.projection {
         CamProj::Perspective => perspective(
            Deg(self.fov),
            self.size.0 as f32 / self.size.1 as f32,
            self.clip.0,
            self.clip.1,
         ),
         CamProj::Orthographic => ortho(
            -1.0,
            self.size.0 as f32 / self.size.1 as f32,
            -1.0,
            1.0,
            self.clip.0,
            self.clip.1,
         ),
      }
   }

   pub(crate) fn recalc_view(&mut self) {
      self.transform.calc_matrix();
      self.view_matrix = self.transform.matrix
   }

   pub fn resize(&mut self, width: u32, height: u32) {
      self.size.0 = width;
      self.size.1 = height
   }
   pub fn set_proj(&mut self, proj: CamProj) {
      self.projection = proj;
      self.recalc_proj()
   }

   pub fn set_fov(&mut self, fov: f32) {
      self.fov = fov;
      self.recalc_proj()
   }
}
