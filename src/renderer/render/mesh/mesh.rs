use std::ffi::CString;
use cgmath::{Deg, Matrix, Matrix4, perspective, Rad, SquareMatrix, vec3, Vector3};
use gl::types::*;
use crate::{NerveCanvas, NerveShader};

pub struct NerveMesh {
   pub(crate) shader: NerveShader,
   pub(crate) has_indices: bool,
   pub(crate) vert_count: u32,
   pub(crate) indices_count: u32,
   pub(crate) vao_id: GLuint,
   pub(crate) vbo_id: GLuint,

   pub(crate) transform: Matrix4<f32>,
   pub(crate) position: Vector3<f32>,
   pub(crate) rotation: Vector3<f32>,
   pub(crate) scale: Vector3<f32>,
}

impl Default for NerveMesh {
   fn default() -> Self {
      Self {
         shader: NerveShader { program_id: 0 },
         has_indices: false,
         vert_count: 0,
         indices_count: 0,
         vao_id: 0,
         vbo_id: 0,
         transform: Matrix4::identity(),
         position: vec3(0.0, 0.0, 0.0),
         rotation: vec3(0.0, 0.0, 0.0),
         scale: vec3(1.0, 1.0, 1.0),
      }
   }
}
impl NerveMesh {
   fn calc_transform(&mut self) {
      let pos_matrix = Matrix4::<f32>::from_translation(self.position);
      let rot_matrix = Matrix4::<f32>::from_angle_x(Rad(self.rotation.x))
         * Matrix4::<f32>::from_angle_y(Rad(self.rotation.y))
         * Matrix4::<f32>::from_angle_z(Rad(self.rotation.z));
      let scale_matrix =
         Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

      self.transform = pos_matrix * scale_matrix * rot_matrix;
   }

   pub fn draw_to(&mut self, canvas: &NerveCanvas) {
      unsafe {
         self.calc_transform();
         self.shader.set();

         self.shader.set_mat4("u_Model", self.transform);
         self.shader.set_mat4("u_Projection", canvas.cam.proj_matrix);

         gl::UniformMatrix4fv(
            2,
            1,
            gl::FALSE,
            &Matrix4::from_translation(vec3(0., 0., -3.))[0][0],
         );

         gl::BindVertexArray(self.vao_id);
         if self.has_indices {
            gl::DrawElements(
               gl::TRIANGLES,
               self.indices_count as GLsizei,
               gl::UNSIGNED_INT,
               std::ptr::null(),
            );
         } else {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vert_count as GLsizei);
         }
      }
   }

   pub fn kill(&self) {
      self.shader.kill();
      unsafe {
         gl::DeleteVertexArrays(1, self.vao_id as *const GLuint);
         gl::DeleteBuffers(1, self.vbo_id as *const GLuint);
      }
   }
}
