use cgmath::{Deg, Matrix4, ortho, perspective, Vector3};

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
   pub(crate) position: Vector3<f32>,
   pub(crate) rotation: Vector3<f32>,
}

impl NerveCamera {
   pub(crate) fn calc_proj_matrix(&mut self) {
      self.proj_matrix = match self.projection {
         CamProj::Perspective => perspective(
            Deg(50.0),
            (self.size.0 / self.size.1) as f32,
            self.clip.0,
            self.clip.1,
         ),
         CamProj::Orthographic => ortho(
            0.0,
            self.size.0 as f32,
            0.0,
            self.size.1 as f32,
            self.clip.0,
            self.clip.1,
         ),
      }
   }
   pub(crate) fn get_proj_matrix(&self) -> Matrix4<f32> {
      self.proj_matrix
   }

   pub fn set_proj(&mut self, proj: CamProj) {
      self.projection = proj;
      self.calc_proj_matrix()
   }
}
