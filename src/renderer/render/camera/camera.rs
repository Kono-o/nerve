use cgmath::*;
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
   pub(crate) ortho_scale: f32,
   pub(crate) clip: (f32, f32),

   pub(crate) front: Vector3<f32>,
   pub(crate) proj_matrix: Matrix4<f32>,
   pub(crate) view_matrix: Matrix4<f32>,
   pub transform: Transform,
}

impl NerveCamera {
   pub(crate) fn default(width: i32, height: i32) -> Self {
      let (widthf, heightf) = (width as f32, height as f32);
      let fov = 50.0;
      let proj_matrix = perspective(Deg(fov), widthf / heightf, 0.01, 1000.0);

      let pos = vec3(0.0, 0.0, 5.0);
      let rot = vec3(0.0, 20.0, 0.0);

      let pos_inverse = Matrix4::from_translation(vec3(-pos.x, -pos.y, -pos.z));
      let rot_inverse = Matrix4::<f32>::from_angle_x(Rad::from(Deg(-rot.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(-rot.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(-rot.z)));
      let view_matrix = pos_inverse * rot_inverse;
      Self {
         size: (width as u32, height as u32),
         projection: CamProj::Perspective,
         fov,
         ortho_scale: 2.0,
         clip: (0.01, 1000.0),
         front: vec3(0.0, 0.0, -1.0),
         proj_matrix,
         view_matrix,
         transform: Transform {
            matrix: Matrix4::identity(),
            pos,
            rot,
            scale: vec3(1.0, 1.0, 1.0),
         },
      }
   }
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
         CamProj::Orthographic => {
            let bound_w = (self.size.0 as f32 / self.size.1 as f32) * self.ortho_scale;
            let bound_h = self.ortho_scale;
            ortho(
               -bound_w,
               bound_w,
               -bound_h,
               bound_h,
               self.clip.0,
               self.clip.1,
            )
         }
      }
   }

   pub(crate) fn recalc_view(&mut self) {
      let pos_inverse = Matrix4::from_translation(vec3(
         -self.transform.pos.x,
         -self.transform.pos.y,
         -self.transform.pos.z,
      ));
      let rot_inverse = Matrix4::<f32>::from_angle_x(Rad::from(Deg(-self.transform.rot.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(-self.transform.rot.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(-self.transform.rot.z)));
      self.view_matrix = pos_inverse * rot_inverse;
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
