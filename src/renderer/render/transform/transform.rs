use std::ops::Add;
use cgmath::{Deg, Matrix4, Rad, SquareMatrix, vec3, Vector3};

#[derive(Clone)]
pub struct Transform {
   pub(crate) matrix: Matrix4<f32>,
   pub(crate) position: Vector3<f32>,
   pub(crate) rotation: Vector3<f32>,
   pub(crate) scale: Vector3<f32>,
}

impl Default for Transform {
   fn default() -> Self {
      Self {
         matrix: Matrix4::identity(),
         position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
         },
         rotation: Vector3 {
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
         position: vec3(x, y, z),
         ..Default::default()
      }
   }
   pub fn with_rot(x: f32, y: f32, z: f32) -> Self {
      Self {
         rotation: vec3(x, y, z),
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
      let pos_matrix = Matrix4::<f32>::from_translation(self.position);
      let rot_matrix = Matrix4::<f32>::from_angle_x(Rad::from(Deg(self.rotation.x)))
         * Matrix4::<f32>::from_angle_y(Rad::from(Deg(self.rotation.y)))
         * Matrix4::<f32>::from_angle_z(Rad::from(Deg(self.rotation.z)));
      let scale_matrix =
         Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

      self.matrix = scale_matrix * pos_matrix * rot_matrix;
   }

   pub fn translate(&mut self, x: f32, y: f32, z: f32) {
      self.position = self.position.add(vec3(x, y, z));
   }
   pub fn translate_x(&mut self, x: f32) {
      self.position.x += x;
   }
   pub fn translate_y(&mut self, y: f32) {
      self.position.y += y;
   }
   pub fn translate_z(&mut self, z: f32) {
      self.position.z += z;
   }

   pub fn set_translation(&mut self, x: f32, y: f32, z: f32) {
      self.position = vec3(x, y, z);
   }
   pub fn set_translation_x(&mut self, x: f32) {
      self.position.x = x;
   }
   pub fn set_translation_y(&mut self, y: f32) {
      self.position.y = y;
   }
   pub fn set_translation_z(&mut self, z: f32) {
      self.position.z = z;
   }

   pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
      self.rotation = self.rotation.add(vec3(x, y, z));
   }
   pub fn rotate_x(&mut self, x: f32) {
      self.rotation.x += x;
   }
   pub fn rotate_y(&mut self, y: f32) {
      self.rotation.y += y;
   }
   pub fn rotate_z(&mut self, z: f32) {
      self.rotation.z += z;
   }

   pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
      self.rotation = vec3(x, y, z);
   }
   pub fn set_rotation_x(&mut self, x: f32) {
      self.rotation.x = x;
   }
   pub fn set_rotation_y(&mut self, y: f32) {
      self.rotation.y = y;
   }
   pub fn set_rotation_z(&mut self, z: f32) {
      self.rotation.z = z;
   }

   pub fn scale(&mut self, x: f32, y: f32, z: f32) {
      self.rotation = self.scale.add(vec3(x, y, z));
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
      self.rotation = vec3(x, y, z);
   }
   pub fn set_scale_uniform(&mut self, xyz: f32) {
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
