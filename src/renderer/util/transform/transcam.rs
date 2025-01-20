use crate::{CamProj, ClipDist, Size2D};
use cgmath::*;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct CamTransform {
   pub(crate) fov: f32,
   pub(crate) ortho_scale: f32,

   pub(crate) clip: ClipDist,
   pub(crate) size: Size2D,
   pub(crate) proj: CamProj,

   pub(crate) front: Vector3<f32>,

   pub(crate) persp_matrix: Matrix4<f32>,
   pub(crate) ortho_matrix: Matrix4<f32>,

   pub(crate) view_matrix: Matrix4<f32>,

   pub(crate) pos: Vector3<f32>,
   pub(crate) rot: Vector3<f32>,
}

impl CamTransform {
   fn calc_persp_matrix(&mut self) {
      self.persp_matrix = perspective(
         Deg(self.fov),
         self.size.aspect_ratio(),
         self.clip.near,
         self.clip.far,
      );
   }

   fn calc_ortho_matrix(&mut self) {
      let bound_w = self.size.aspect_ratio() * self.ortho_scale;
      let bound_h = self.ortho_scale;
      self.ortho_matrix = ortho(
         -bound_w,
         bound_w,
         -bound_h,
         bound_h,
         self.clip.near,
         self.clip.far,
      );
   }

   pub(crate) fn calc_matrices(&mut self) {
      self.calc_persp_matrix();
      self.calc_ortho_matrix();
      self.calc_view_matrix();
   }

   pub(crate) fn proj_matrix(&self) -> Matrix4<f32> {
      match self.proj {
         CamProj::Persp => self.persp_matrix,
         CamProj::Ortho => self.ortho_matrix,
      }
   }

   pub(crate) fn view_matrix(&self) -> Matrix4<f32> {
      self.view_matrix
   }

   pub(crate) fn calc_view_matrix(&mut self) {
      let pitch_cos = self.rot.x.to_radians().cos();
      self.front = vec3(
         self.rot.y.to_radians().cos() * pitch_cos,
         self.rot.x.to_radians().sin(),
         self.rot.y.to_radians().sin() * pitch_cos,
      )
      .normalize();

      let eye = point3(self.pos.x, self.pos.y, self.pos.z);
      let centre = point3(
         self.front.x + eye.x,
         self.front.y + eye.y,
         self.front.z + eye.z,
      );
      self.view_matrix = Matrix4::look_at_rh(eye, centre, vec3(0.0, 1.0, 0.0));
   }

   pub fn move_all(&mut self, x: f32, y: f32, z: f32) {
      self.pos = self.pos.add(vec3(x, y, z));
   }
   pub fn move_x(&mut self, x: f32) {
      self.pos.x += x;
   }
   pub fn move_y(&mut self, y: f32) {
      self.pos.y += y;
   }
   pub fn move_z(&mut self, z: f32) {
      self.pos.z += z;
   }

   pub fn set_pos_all(&mut self, x: f32, y: f32, z: f32) {
      self.pos = vec3(x, y, z);
   }
   pub fn set_pos_x(&mut self, x: f32) {
      self.pos.x = x;
   }
   pub fn set_pos_y(&mut self, y: f32) {
      self.pos.y = y;
   }
   pub fn set_pos_z(&mut self, z: f32) {
      self.pos.z = z;
   }

   pub fn rotate_all(&mut self, x: f32, y: f32, z: f32) {
      self.rot = self.rot.add(vec3(x, y, z));
   }
   pub fn rotate_x(&mut self, x: f32) {
      self.rot.x += x;
   }
   pub fn rotate_y(&mut self, y: f32) {
      self.rot.y += y;
   }
   pub fn rotate_z(&mut self, z: f32) {
      self.rot.z += z;
   }

   pub fn set_rot_all(&mut self, x: f32, y: f32, z: f32) {
      self.rot = vec3(x, y, z);
   }
   pub fn set_rot_x(&mut self, x: f32) {
      self.rot.x = x;
   }
   pub fn set_rot_y(&mut self, y: f32) {
      self.rot.y = y;
   }
   pub fn set_rot_z(&mut self, z: f32) {
      self.rot.z = z;
   }
}
