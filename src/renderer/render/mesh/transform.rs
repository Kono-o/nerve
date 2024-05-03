use std::ops::Add;
use cgmath::vec3;
use crate::NerveMesh;

impl NerveMesh {
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

   //pub fn scale(&mut self, x: f32, y: f32, z: f32) {
   //   self.rotation = self.scale.add(vec3(x, y, z));
   //}
   //pub fn scale_uniformly(&mut self, xyz: f32) {
   //   self.scale(xyz, xyz, xyz);
   //}
   //pub fn scale_x(&mut self, x: f32) {
   //   self.scale.x += x;
   //}
   //pub fn scale_y(&mut self, y: f32) {
   //   self.scale.y += y;
   //}
   //pub fn scale_z(&mut self, z: f32) {
   //   self.scale.z += z;
   //}
   //
   //pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
   //   self.rotation = vec3(x, y, z);
   //}
   //pub fn set_scale_uniform(&mut self, xyz: f32) {
   //   self.set_scale(xyz, xyz, xyz);
   //}
   //pub fn set_scale_x(&mut self, x: f32) {
   //   self.scale.x = x;
   //}
   //pub fn set_scale_y(&mut self, y: f32) {
   //   self.scale.y = y;
   //}
   //pub fn set_scale_z(&mut self, z: f32) {
   //   self.scale.z = z;
   //}
}
