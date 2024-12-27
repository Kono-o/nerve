use crate::renderer::Renderer;
use crate::{Cull, DrawMode, NerveMesh, NerveShader, PolyMode, Uniform, WinSize, RGB};
use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};
use glfw::{Context, Glfw, PWindow};
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Copy, Clone)]
pub(crate) struct GLRenderer;

impl Renderer for GLRenderer {
   fn init(&self, window: &mut PWindow, _glfw: &mut Glfw) {
      window.make_current();
      gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
   }
   fn info(&self) -> (String, String, String) {
      unsafe {
         (
            CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8) //GPU
               .to_str()
               .unwrap_or("")
               .to_string(),
            CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) //API VERSION
               .to_str()
               .unwrap_or("")
               .to_string(),
            CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8) //GLSL VERSION
               .to_str()
               .unwrap_or("")
               .to_string(),
         )
      }
   }

   //STATE
   fn set_bg_color(&self, color: RGB) {
      unsafe {
         gl::ClearColor(color.0, color.1, color.2, 1.0);
      }
   }
   fn resize(&self, size: WinSize) {
      unsafe {
         gl::Viewport(0, 0, size.w as GLsizei, size.h as GLsizei);
      }
   }
   fn poly_mode(&self, mode: PolyMode) {
      unsafe {
         match mode {
            PolyMode::WireFrame => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
            PolyMode::Filled => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
            PolyMode::Points => {
               gl::PointSize(10.0);
               gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT)
            }
         }
      }
   }
   fn enable_msaa(&self, enable: bool) {
      unsafe {
         match enable {
            true => gl::Enable(gl::MULTISAMPLE),
            false => gl::Disable(gl::MULTISAMPLE),
         }
      }
   }
   fn enable_depth(&self, enable: bool) {
      unsafe {
         match enable {
            true => gl::Enable(gl::DEPTH_TEST),
            false => gl::Disable(gl::DEPTH_TEST),
         }
      }
   }
   fn enable_cull(&self, enable: bool) {
      unsafe {
         if enable {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
         } else {
            gl::Disable(gl::CULL_FACE);
         }
      }
   }
   fn set_cull_face(&self, face: Cull) {
      unsafe {
         match face {
            Cull::Clock => gl::FrontFace(gl::CW),
            Cull::AntiClock => gl::FrontFace(gl::CCW),
         }
      }
   }
   fn set_wire_width(&self, thickness: f32) {
      unsafe { gl::LineWidth(thickness) }
   }

   fn bind_program(&self, prog_id: GLuint) {
      unsafe { gl::UseProgram(prog_id) }
   }
   fn unbind_program(&self) {
      unsafe { gl::UseProgram(0) }
   }
   fn bind_texture(&self, tex_id: GLuint) {
      unsafe {
         gl::BindTexture(gl::TEXTURE_2D, tex_id);
      }
   }
   fn unbind_texture(&self) {
      unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
   }

   //SHADERS
   fn compile_shader(&self, src: &str, typ: GLenum) -> GLuint {
      let log_len = 256;
      let mut log = Vec::with_capacity(log_len);
      let mut success = gl::FALSE as GLint;
      let src = CString::new(src.as_bytes()).expect("src empty!");
      unsafe {
         let shader = gl::CreateShader(typ);
         gl::ShaderSource(shader, 1, &src.as_ptr(), ptr::null());
         gl::CompileShader(shader);
         log.set_len(log_len - 1);
         gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
         if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
               shader,
               log_len as GLsizei,
               ptr::null_mut(),
               log.as_mut_ptr() as *mut GLchar,
            );
            let log = std::str::from_utf8(&log).unwrap_or("");
            panic!("failed to compile shader: {}", log);
         }
         shader
      }
   }
   fn create_program(
      &self,
      vert: &str,
      frag: &str,
      image_ids: Vec<(String, GLuint)>,
   ) -> NerveShader {
      let log_len = 256;
      let mut log = Vec::with_capacity(log_len);
      let mut success = gl::FALSE as GLint;
      unsafe {
         let program = gl::CreateProgram();
         let vert_shader = self.compile_shader(vert, gl::VERTEX_SHADER);
         let frag_shader = self.compile_shader(frag, gl::FRAGMENT_SHADER);

         gl::AttachShader(program, vert_shader);
         gl::AttachShader(program, frag_shader);
         gl::LinkProgram(program);

         gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
         if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
               program,
               log_len as GLsizei,
               std::ptr::null_mut(),
               log.as_mut_ptr() as *mut GLchar,
            );
            let log = std::str::from_utf8(&log).unwrap_or("");
            panic!("failed to create program: {}", log);
         }
         gl::DeleteShader(vert_shader);
         gl::DeleteShader(frag_shader);
         NerveShader {
            id: program,
            image_ids,
            is_compiled: true,
         }
      }
   }
   fn set_uni(&self, id: GLuint, name: &str, uniform: Uniform) {
      unsafe {
         let c_name = CString::new(name).unwrap();
         let location = gl::GetUniformLocation(id, c_name.as_ptr());
         if location == -1 {
            panic!("uniform '{name}' does not exist!");
         } else {
            match uniform {
               Uniform::Matrix4(m) => gl::UniformMatrix4fv(location, 1, gl::FALSE, m.as_ptr()),
            }
         }
      }
   }
   fn set_uni_m4f32(&self, id: GLuint, name: &str, matrix: Matrix4<f32>) {
      unsafe {
         let c_name = CString::new(name).unwrap();
         let location = gl::GetUniformLocation(id, c_name.as_ptr());
         if location == -1 {
            panic!("uniform '{name}' does not exist!");
         } else {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr())
         }
      }
   }
   //BUFFERS

   //DRAW
   fn clear(&self) {
      unsafe {
         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      }
   }
   fn draw(&self, mesh: &NerveMesh) {
      let draw_mode = match_draw_mode_gl(&mesh.draw_mode);
      unsafe {
         mesh.vert_object.bind();
         if mesh.has_indices {
            mesh.index_object.bind();
            gl::DrawElements(
               draw_mode,
               mesh.ind_count as GLsizei,
               gl::UNSIGNED_INT,
               ptr::null(),
            );
         } else {
            gl::DrawArrays(draw_mode, 0, mesh.vert_count as GLsizei);
         }
      }
   }
}

fn match_draw_mode_gl(dm: &DrawMode) -> GLenum {
   match dm {
      DrawMode::Points => gl::POINTS,
      DrawMode::Lines => gl::LINES,
      DrawMode::Triangles => gl::TRIANGLES,
      DrawMode::Strip => gl::TRIANGLE_STRIP,
   }
}
