use cgmath::{vec3, Deg, Matrix4, Rad, SquareMatrix, Vector3};
use std::ops::Add;

#[derive(Clone)]
pub struct Transform {
   pub(crate) matrix: Matrix4<f32>,
   pub(crate) pos: Vector3<f32>,
   pub(crate) rot: Vector3<f32>,
   pub(crate) scale: Vector3<f32>,
}

impl Default for Transform {
   fn default() -> Self {
      Self {
         matrix: Matrix4::identity(),
         pos: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
         },
         rot: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
         },
         scale: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
         },
      }
   }
}

impl Transform {
   pub fn with_pos(x: f32, y: f32, z: f32) -> Self {
      Self {
         pos: vec3(x, y, z),
         ..Default::default()
      }
   }
   pub fn with_rot(x: f32, y: f32, z: f32) -> Self {
      Self {
         rot: vec3(x, y, z),
         ..Default::default()
      }
   }
   pub fn with_scale(x: f32, y: f32, z: f32) -> Self {
      Self {
         scale: vec3(x, y, z),
         ..Default::default()
      }
   }

   pub(crate) fn calc_matrix(&mut self) {
      let pos_matrix = Matrix4::<f32>::from_translation(self.pos);
      let rot_matrix = Matrix4::<f32>::from_angle_x(Rad::from(Deg(self.rot.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(self.rot.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(self.rot.z)));
      let scale_matrix =
         Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
      self.matrix = pos_matrix * rot_matrix * scale_matrix;
   }

   pub fn translate(&mut self, x: f32, y: f32, z: f32) {
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

   pub fn set_translation(&mut self, x: f32, y: f32, z: f32) {
      self.pos = vec3(x, y, z);
   }
   pub fn set_translation_x(&mut self, x: f32) {
      self.pos.x = x;
   }
   pub fn set_translation_y(&mut self, y: f32) {
      self.pos.y = y;
   }
   pub fn set_translation_z(&mut self, z: f32) {
      self.pos.z = z;
   }

   pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
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

   pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
      self.rot = vec3(x, y, z);
   }
   pub fn set_rotation_x(&mut self, x: f32) {
      self.rot.x = x;
   }
   pub fn set_rotation_y(&mut self, y: f32) {
      self.rot.y = y;
   }
   pub fn set_rotation_z(&mut self, z: f32) {
      self.rot.z = z;
   }

   pub fn scale(&mut self, x: f32, y: f32, z: f32) {
      self.scale = self.scale.add(vec3(x, y, z));
   }
   pub fn scale_uniformly(&mut self, xyz: f32) {
      self.scale(xyz, xyz, xyz);
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

   pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
      self.scale = vec3(x, y, z);
   }
   pub fn set_scale(&mut self, xyz: f32) {
      self.set_scale(xyz, xyz, xyz);
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
