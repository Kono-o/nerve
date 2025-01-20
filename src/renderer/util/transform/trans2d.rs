use cgmath::*;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Transform2D {
   pub(crate) matrix: Matrix4<f32>,
   pub(crate) pos: Vector2<f32>,
   pub(crate) rot: Vector2<f32>,
   pub(crate) scale: Vector2<f32>,
}

impl Default for Transform2D {
   fn default() -> Transform2D {
      Transform2D {
         matrix: Matrix4::identity(),
         pos: Vector2::new(0.0, 0.0),
         rot: Vector2::new(0.0, 0.0),
         scale: Vector2::new(1.0, 1.0),
      }
   }
}

impl Transform2D {
   fn calc_pos_matrix(&self) -> Matrix4<f32> {
      let vec3_pos = vec3(self.pos.x, self.pos.y, 0.0);
      Matrix4::<f32>::from_translation(vec3_pos)
   }

   fn calc_rot_matrix(&self) -> Matrix4<f32> {
      let x = Matrix4::<f32>::from_angle_x(Rad::from(Deg(self.rot.x)));
      let y = Matrix4::<f32>::from_angle_y(Rad::from(Deg(self.rot.y)));
      x * y
   }

   fn calc_scale_matrix(&self) -> Matrix4<f32> {
      Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0)
   }

   pub(crate) fn calc_matrix(&mut self) {
      self.matrix = self.calc_pos_matrix() * self.calc_rot_matrix() * self.calc_scale_matrix();
   }

   pub fn pos(&self) -> Vector2<f32> {
      self.pos
   }
   pub fn rot(&self) -> Vector2<f32> {
      self.rot
   }
   pub fn scale(&self) -> Vector2<f32> {
      self.scale
   }

   pub fn move_all(&mut self, x: f32, y: f32) {
      self.pos = self.pos.add(vec2(x, y));
   }
   pub fn move_x(&mut self, x: f32) {
      self.pos.x += x;
   }
   pub fn move_y(&mut self, y: f32) {
      self.pos.y += y;
   }

   pub fn set_pos_all(&mut self, x: f32, y: f32) {
      self.pos = vec2(x, y);
   }
   pub fn set_pos_x(&mut self, x: f32) {
      self.pos.x = x;
   }
   pub fn set_pos_y(&mut self, y: f32) {
      self.pos.y = y;
   }

   pub fn rotate_all(&mut self, x: f32, y: f32) {
      self.rot = self.rot.add(vec2(x, y));
   }
   pub fn rotate_x(&mut self, x: f32) {
      self.rot.x += x;
   }
   pub fn rotate_y(&mut self, y: f32) {
      self.rot.y += y;
   }

   pub fn set_rot_all(&mut self, x: f32, y: f32) {
      self.rot = vec2(x, y);
   }
   pub fn set_rot_x(&mut self, x: f32) {
      self.rot.x = x;
   }
   pub fn set_rot_y(&mut self, y: f32) {
      self.rot.y = y;
   }

   pub fn scale_all(&mut self, x: f32, y: f32) {
      self.scale = self.scale.add(vec2(x, y));
   }
   pub fn scale_same(&mut self, xy: f32) {
      self.scale_all(xy, xy);
   }
   pub fn scale_x(&mut self, x: f32) {
      self.scale.x += x;
   }
   pub fn scale_y(&mut self, y: f32) {
      self.scale.y += y;
   }

   pub fn set_scale_all(&mut self, x: f32, y: f32) {
      self.scale = vec2(x, y);
   }
   pub fn set_scale_same(&mut self, xy: f32) {
      self.set_scale_all(xy, xy);
   }
   pub fn set_scale_x(&mut self, x: f32) {
      self.scale.x = x;
   }
   pub fn set_scale_y(&mut self, y: f32) {
      self.scale.y = y;
   }
}
