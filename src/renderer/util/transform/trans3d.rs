use cgmath::*;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Transform3D {
   pub(crate) matrix: Matrix4<f32>,
   pub(crate) pos: Vector3<f32>,
   pub(crate) rot: Vector3<f32>,
   pub(crate) scale: Vector3<f32>,
}

impl Default for Transform3D {
   fn default() -> Transform3D {
      Transform3D {
         matrix: Matrix4::identity(),
         pos: Vector3::new(0.0, 0.0, 0.0),
         rot: Vector3::new(0.0, 0.0, 0.0),
         scale: Vector3::new(1.0, 1.0, 1.0),
      }
   }
}

impl Transform3D {
   fn calc_pos_matrix(&self) -> Matrix4<f32> {
      Matrix4::<f32>::from_translation(self.pos)
   }

   fn calc_rot_matrix(&self) -> Matrix4<f32> {
      let x = Matrix4::<f32>::from_angle_x(Rad::from(Deg(self.rot.x)));
      let y = Matrix4::<f32>::from_angle_y(Rad::from(Deg(self.rot.y)));
      let z = Matrix4::<f32>::from_angle_z(Rad::from(Deg(self.rot.z)));
      x * y * z
   }

   fn calc_scale_matrix(&self) -> Matrix4<f32> {
      Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
   }

   pub(crate) fn calc_matrix(&mut self) {
      self.matrix = self.calc_pos_matrix() * self.calc_rot_matrix() * self.calc_scale_matrix();
   }

   pub fn pos(&self) -> Vector3<f32> {
      self.pos
   }
   pub fn rot(&self) -> Vector3<f32> {
      self.rot
   }
   pub fn scale(&self) -> Vector3<f32> {
      self.scale
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

   pub fn scale_all(&mut self, x: f32, y: f32, z: f32) {
      self.scale = self.scale.add(vec3(x, y, z));
   }
   pub fn scale_same(&mut self, xyz: f32) {
      self.scale_all(xyz, xyz, xyz);
   }
   pub fn scale_x(&mut self, x: f32) {
      self.scale.x += x;
   }
   pub fn scale_y(&mut self, y: f32) {
      self.scale.y += y;
   }
   pub fn scale_z(&mut self, z: f32) {
      self.scale.z += z;
   }

   pub fn set_scale_all(&mut self, x: f32, y: f32, z: f32) {
      self.scale = vec3(x, y, z);
   }
   pub fn set_scale_same(&mut self, xyz: f32) {
      self.set_scale_all(xyz, xyz, xyz);
   }
   pub fn set_scale_x(&mut self, x: f32) {
      self.scale.x = x;
   }
   pub fn set_scale_y(&mut self, y: f32) {
      self.scale.y = y;
   }
   pub fn set_scale_z(&mut self, z: f32) {
      self.scale.z = z;
   }
}
