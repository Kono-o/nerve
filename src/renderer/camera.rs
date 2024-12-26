use crate::Transform;
use crate::WinSize;
use cgmath::*;

pub struct ClipDist {
   pub near: f32,
   pub far: f32,
}
pub enum CamProj {
   Orthographic,
   Perspective,
}

pub struct NerveCamera {
   pub(crate) size: WinSize,
   pub(crate) projection: CamProj,
   pub(crate) fov: f32,
   pub(crate) ortho_scale: f32,
   pub(crate) clip: (f32, f32),

   pub(crate) front: Vector3<f32>,
   pub(crate) proj_matrix: Matrix4<f32>,
   pub(crate) view_matrix: Matrix4<f32>,
   pub(crate) transform: Transform,
}

impl NerveCamera {
   pub fn new(size: WinSize, projection: CamProj) -> Self {
      let fov = 50.0;
      let (widthf, heightf) = (size.w as f32, size.h as f32);
      let proj_matrix = perspective(Deg(fov), widthf / heightf, 0.01, 1000.0);

      let pos = vec3(0.0, 0.0, 5.0);
      let rot = vec3(0.0, -90.0, 0.0);

      let pos_inverse = Matrix4::from_translation(vec3(-pos.x, -pos.y, -pos.z));
      let rot_inverse = Matrix4::<f32>::from_angle_x(Rad::from(Deg(-rot.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(-rot.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(-rot.z)));
      let view_matrix = pos_inverse * rot_inverse;

      Self {
         size,
         projection,
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

   pub(crate) fn recalc_proj(&mut self) {
      self.proj_matrix = match self.projection {
         CamProj::Perspective => perspective(
            Deg(self.fov),
            self.size.w as f32 / self.size.h as f32,
            self.clip.0,
            self.clip.1,
         ),
         CamProj::Orthographic => {
            let bound_w = (self.size.w as f32 / self.size.h as f32) * self.ortho_scale;
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
      self.update_front();
      let eye = point3(
         self.transform.pos.x,
         self.transform.pos.y,
         self.transform.pos.z,
      );
      let centre = point3(
         self.front.x + eye.x,
         self.front.y + eye.y,
         self.front.z + eye.z,
      );
      self.view_matrix = Matrix4::look_at_rh(eye, centre, vec3(0.0, 1.0, 0.0));
   }

   fn update_front(&mut self) {
      let pitch_cos = self.transform.rot.x.to_radians().cos();
      self.front = vec3(
         self.transform.rot.y.to_radians().cos() * pitch_cos,
         self.transform.rot.x.to_radians().sin(),
         self.transform.rot.y.to_radians().sin() * pitch_cos,
      )
      .normalize();
   }
}

impl NerveCamera {
   pub fn resize(&mut self, size: WinSize) {
      self.size = size;
      self.recalc_proj()
   }
   pub fn set_proj(&mut self, proj: CamProj) {
      self.projection = proj;
      self.recalc_proj()
   }
   pub fn set_fov(&mut self, fov: f32) {
      self.fov = fov;
      self.recalc_proj()
   }

   pub fn fly_forw(&mut self, speed: f32) {
      self.transform.pos += speed * self.front;
   }
   pub fn fly_back(&mut self, speed: f32) {
      self.transform.pos -= speed * self.front;
   }
   pub fn fly_left(&mut self, speed: f32) {
      self.transform.pos -= speed * self.front.cross(vec3(0.0, 1.0, 0.0).normalize());
   }
   pub fn fly_right(&mut self, speed: f32) {
      self.transform.pos += speed * self.front.cross(vec3(0.0, 1.0, 0.0).normalize());
   }
   pub fn fly_up(&mut self, speed: f32) {
      self.transform.translate_y(speed);
   }
   pub fn fly_down(&mut self, speed: f32) {
      self.transform.translate_y(-speed);
   }

   pub fn spin_x(&mut self, speed: f32) {
      self.transform.rotate_x(speed)
   }
   pub fn spin_y(&mut self, speed: f32) {
      self.transform.rotate_y(speed)
   }
   pub fn spin_z(&mut self, speed: f32) {
      self.transform.rotate_z(speed)
   }
}
