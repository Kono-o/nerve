use crate::Size2D;
use crate::Transform;
use cgmath::*;

pub struct ClipDist {
   pub near: f32,
   pub far: f32,
}
pub enum CamProj {
   Ortho,
   Persp,
}

pub struct NECamera {
   pub fov: f32,
   pub(crate) ortho_scale: f32,
   pub(crate) clip: (f32, f32),
   pub(crate) size: Size2D,
   pub(crate) proj: CamProj,
   pub(crate) front: Vector3<f32>,
   pub(crate) proj_matrix: Matrix4<f32>,
   pub(crate) view_matrix: Matrix4<f32>,
   pub(crate) transform: Transform,
   pub(crate) initialized: bool,
}

impl NECamera {
   fn update_proj(&mut self) {
      self.proj_matrix = match self.proj {
         CamProj::Persp => perspective(
            Deg(self.fov),
            self.size.w as f32 / self.size.h as f32,
            self.clip.0,
            self.clip.1,
         ),
         CamProj::Ortho => {
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
   fn update_view(&mut self) {
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

   pub(crate) fn pre_update(&mut self) {
      self.update_proj();
      self.update_view()
   }
   pub(crate) fn post_update(&mut self) {}
}

impl NECamera {
   pub fn new() -> NECamera {
      let mut cam = NECamera::from(Size2D { w: 1, h: 1 }, CamProj::Persp);
      cam.initialized = false;
      cam
   }

   pub fn from(size: Size2D, proj: CamProj) -> Self {
      let fov = 75.0;
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
         proj,
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
         initialized: true,
      }
   }
   pub fn set_size(&mut self, size: Size2D) {
      self.size = size;
   }
   pub fn set_proj(&mut self, proj: CamProj) {
      self.proj = proj;
   }
   pub fn set_fov(&mut self, fov: f32) {
      self.fov = fov;
   }
   pub fn add_pov(&mut self, value: f32) {
      self.fov += value;
   }
   pub fn set_ortho_scale(&mut self, value: f32) {
      self.ortho_scale = value;
   }
   pub fn add_ortho_scale(&mut self, value: f32) {
      self.ortho_scale += value;
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
      self.transform.move_y(speed);
   }
   pub fn fly_down(&mut self, speed: f32) {
      self.transform.move_y(-speed);
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
