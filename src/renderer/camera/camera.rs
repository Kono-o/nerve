use crate::{CamTransform, Size2D};
use cgmath::*;

#[derive(Clone, Copy, Debug)]
pub struct ClipDist {
   pub(crate) near: f32,
   pub(crate) far: f32,
}

impl ClipDist {
   pub fn from(near: f32, far: f32) -> ClipDist {
      ClipDist { near, far }
   }
}

#[derive(Clone, Copy, Debug)]
pub enum CamProj {
   Ortho,
   Persp,
}

pub struct NECamera {
   pub(crate) transform: CamTransform,
   pub(crate) initialized: bool,
}

impl NECamera {
   pub(crate) fn start(&mut self) {
      self.initialized = true
   }

   pub(crate) fn pre_update(&mut self) {
      self.transform.calc_matrices();
   }

   pub(crate) fn update(&mut self) {}

   pub(crate) fn post_update(&mut self) {}

   pub(crate) fn end(&mut self) {}
}

impl NECamera {
   pub fn new() -> NECamera {
      let mut cam = NECamera::from(Size2D { w: 1, h: 1 }, CamProj::Persp);
      cam.initialized = false;
      cam
   }

   pub fn from(size: Size2D, proj: CamProj) -> Self {
      let fov = 75.0;
      let clip = ClipDist::from(0.01, 1000.0);

      let pos = vec3(0.0, 0.0, 5.0);
      let rot = vec3(0.0, -90.0, 0.0);

      let pos_inverse = Matrix4::from_translation(vec3(-pos.x, -pos.y, -pos.z));
      let rot_inverse = Matrix4::<f32>::from_angle_x(Rad::from(Deg(-rot.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(-rot.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(-rot.z)));

      let view_matrix = pos_inverse * rot_inverse;

      let mut transform = CamTransform {
         pos,
         rot,
         fov,
         clip,
         size,
         proj,
         view_matrix,
         ortho_scale: 2.0,
         front: vec3(0.0, 0.0, -1.0),
         persp_matrix: Matrix4::identity(),
         ortho_matrix: Matrix4::identity(),
      };
      transform.calc_matrices();

      Self {
         transform,
         initialized: true,
      }
   }

   pub fn fov(&self) -> f32 {
      self.transform.fov
   }
   pub fn ortho_scale(&self) -> f32 {
      self.transform.ortho_scale
   }

   pub fn proj(&self) -> CamProj {
      self.transform.proj
   }

   pub fn clip(&self) -> ClipDist {
      self.transform.clip
   }

   pub fn set_clip(&mut self, clip: ClipDist) {
      self.transform.clip = clip
   }

   pub fn set_clip_near(&mut self, near: f32) {
      self.transform.clip.near = near
   }
   pub fn set_clip_far(&mut self, far: f32) {
      self.transform.clip.far = far
   }
   pub fn set_size(&mut self, size: Size2D) {
      self.transform.size = size;
   }
   pub fn set_proj(&mut self, proj: CamProj) {
      self.transform.proj = proj;
   }
   pub fn set_fov(&mut self, fov: f32) {
      self.transform.fov = fov;
   }
   pub fn add_fov(&mut self, value: f32) {
      self.transform.fov += value;
   }
   pub fn set_ortho_scale(&mut self, value: f32) {
      self.transform.ortho_scale = value;
   }
   pub fn add_ortho_scale(&mut self, value: f32) {
      self.transform.ortho_scale += value;
   }

   pub fn fly_forw(&mut self, speed: f32) {
      self.transform.pos += speed * self.transform.front;
   }
   pub fn fly_back(&mut self, speed: f32) {
      self.transform.pos -= speed * self.transform.front;
   }
   pub fn fly_left(&mut self, speed: f32) {
      self.transform.pos -= speed * self.transform.front.cross(vec3(0.0, 1.0, 0.0).normalize());
   }
   pub fn fly_right(&mut self, speed: f32) {
      self.transform.pos += speed * self.transform.front.cross(vec3(0.0, 1.0, 0.0).normalize());
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
