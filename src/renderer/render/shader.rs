use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLuint};

#[derive(Clone, Copy)]
pub struct NerveShader {
   pub(crate) program_id: GLuint,
}

impl NerveShader {
   pub fn new(vert_path: &str, frag_path: &str) -> NerveShader {
      let mut success = gl::FALSE as GLint;
      let mut info_log = Vec::with_capacity(512);

      let mut compile_shader = |path: &str, shader_type: GLenum| {
         let src =
            std::ffi::CString::new(std::fs::read_to_string(path).unwrap().as_bytes()).unwrap();
         unsafe {
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());

            gl::CompileShader(shader);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
               gl::GetShaderInfoLog(
                  shader,
                  512,
                  std::ptr::null_mut(),
                  info_log.as_mut_ptr() as *mut GLchar,
               );
               panic!(
                  "failed to compile -> {}: \n{:?}",
                  path,
                  std::str::from_utf8(&info_log)
               );
            }
            return shader;
         }
      };
      let vert_shader = compile_shader(vert_path, gl::VERTEX_SHADER);
      let frag_shader = compile_shader(frag_path, gl::FRAGMENT_SHADER);

      unsafe {
         let program = gl::CreateProgram();
         gl::AttachShader(program, vert_shader);
         gl::AttachShader(program, frag_shader);
         gl::LinkProgram(program);

         gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
         if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
               program,
               512,
               std::ptr::null_mut(),
               info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
               "failed to link {vert_path}, {frag_path}: \n{:?}",
               std::str::from_utf8(&info_log)
            );
         }
         gl::DeleteShader(vert_shader);
         gl::DeleteShader(frag_shader);

         return NerveShader {
            program_id: program,
         };
      }
   }
   pub fn set(&self) {
      unsafe { gl::UseProgram(self.program_id) }
   }

   pub fn set_uniform_mat4(&self, location: u32, mat4: Matrix4<f32>) {
      unsafe {
         gl::UniformMatrix4fv(location as GLint, 1, gl::FALSE, mat4.as_ptr());
      }
   }
   pub fn kill(&self) {
      unsafe {
         gl::DeleteProgram(self.program_id);
      }
   }
}
