use crate::asset::{ATTRInfo, TexFormat};
use crate::renderer::{Renderer, ShaderType};
use crate::util::misc;
use crate::{ansi, NEShaderAsset, NETexture, TexFilter, TexWrap, RGB};
use crate::{log_info, ATTRType, Cull, DrawMode, NEError, NEResult, PolyMode, Size2D, Uniform};
use cgmath::{Matrix, Matrix4};
use glfw::{Context, PWindow};
use gll as gl;
use gll::types::*;
use gll::{ContextInitError, HasContext};
use std::ffi::{c_void, CString};
use std::ptr;

pub enum NEOpenGLErrKind {
   NoActiveContext,
   CouldParseVersion(String),
   SPIRVNotFound,
}

pub struct GLInfo {
   glsl_ver: String,
   device: String,
   spirv_compat: bool,
}
pub(crate) struct GLRenderer {
   gl: gl::Context,
   info: GLInfo,
}
const TEX: u32 = gl::TEXTURE_2D;

pub(crate) fn gl_renderer_init(window: &mut PWindow) -> NEResult<GLRenderer> {
   window.make_current();
   unsafe {
      match gl::Context::load(|symbol| window.get_proc_address(symbol)) {
         Ok(gl) => {
            let device = gl.get_parameter_string(gl::RENDERER);
            let glsl_ver = gl.get_parameter_string(gl::SHADING_LANGUAGE_VERSION);

            let arb = &gl.extensions;
            let mut spirv_compat = false;
            if arb.contains(misc::SPIRV_EXTENSIONS) && arb.contains(misc::GL_SPV_EXTENSION) {
               spirv_compat = true
            } else {
               return NEResult::ER(NEError::OpenGL {
                  kind: NEOpenGLErrKind::SPIRVNotFound,
               });
            }

            NEResult::OK(GLRenderer {
               gl,
               info: GLInfo {
                  glsl_ver,
                  device,
                  spirv_compat,
               },
            })
         }
         Err(e) => NEResult::ER(NEError::OpenGL {
            kind: match e {
               ContextInitError::CouldParseVersion(s) => NEOpenGLErrKind::CouldParseVersion(s),
               _ => NEOpenGLErrKind::NoActiveContext,
            },
         }),
      }
   }
}

impl Renderer for GLRenderer {
   fn log_info(&self) {
      let (v0, v1) = (self.gl.version.major, self.gl.version.minor);
      let spv = match self.info.spirv_compat {
         true => "[with spirv]",
         false => "",
      };
      let glsl = &self.info.glsl_ver;
      let device = &self.info.device;
      log_info!("BACKEND");
      log_info!("> api: OpenGL {v0}.{v1}0 {spv}");
      log_info!("> shaders: {glsl}");
      log_info!("> gpu: {device}\n");
   }

   //STATE
   fn set_clear(&self, color: RGB) {
      unsafe {
         self.gl.raw.ClearColor(color.0, color.1, color.2, 1.0);
      }
   }
   fn resize(&self, size: Size2D) {
      unsafe {
         self.gl.raw.Viewport(0, 0, size.w as i32, size.h as i32);
      }
   }
   fn poly_mode(&self, mode: PolyMode) {
      let gl = &self.gl;
      unsafe {
         match mode {
            PolyMode::WireFrame => gl.raw.PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
            PolyMode::Filled => gl.raw.PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
            PolyMode::Points => {
               gl.raw.PointSize(10.0);
               gl.raw.PolygonMode(gl::FRONT_AND_BACK, gl::POINT)
            }
         }
      }
   }
   fn enable_msaa(&self, enable: bool) {
      let gl = &self.gl;
      unsafe {
         match enable {
            true => gl.raw.Enable(gl::MULTISAMPLE),
            false => gl.raw.Disable(gl::MULTISAMPLE),
         }
      }
   }
   fn enable_depth(&self, enable: bool) {
      let gl = &self.gl;
      unsafe {
         match enable {
            true => gl.raw.Enable(gl::DEPTH_TEST),
            false => gl.raw.Disable(gl::DEPTH_TEST),
         }
      }
   }
   fn enable_cull(&self, enable: bool) {
      let gl = &self.gl;
      unsafe {
         match enable {
            true => {
               gl.raw.Enable(gl::CULL_FACE);
               gl.raw.CullFace(gl::BACK);
            }
            false => gl.raw.Disable(gl::CULL_FACE),
         }
      }
   }
   fn set_cull_face(&self, face: Cull) {
      let gl = &self.gl;
      unsafe {
         match face {
            Cull::Clock => gl.raw.FrontFace(gl::CW),
            Cull::AntiClock => gl.raw.FrontFace(gl::CCW),
         }
      }
   }
   fn set_wire_width(&self, width: f32) {
      unsafe { self.gl.raw.LineWidth(width) }
   }

   fn bind_program(&self, prog_id: u32) {
      unsafe { self.gl.raw.UseProgram(prog_id) }
   }
   fn unbind_program(&self) {
      unsafe { self.gl.raw.UseProgram(0) }
   }

   fn bind_texture_at(&self, tex_id: u32, slot: u32) {
      let gl = &self.gl;
      unsafe {
         gl.raw.ActiveTexture(gl::TEXTURE0 + slot);
         gl.raw.BindTexture(TEX, tex_id);
      }
   }
   fn unbind_texture(&self) {
      unsafe {
         self.gl.raw.BindTexture(TEX, 0);
      }
   }

   fn bind_buffer(&self, v_id: u32, b_id: u32) {
      let gl = &self.gl;
      unsafe {
         gl.raw.BindVertexArray(v_id);
         gl.raw.BindBuffer(gl::ARRAY_BUFFER, b_id);
      }
   }
   fn unbind_buffer(&self) {
      let gl = &self.gl;
      unsafe {
         gl.raw.BindVertexArray(0);
         gl.raw.BindBuffer(gl::ARRAY_BUFFER, 0);
      }
   }

   fn bind_index_buffer(&self, id: u32) {
      unsafe {
         self.gl.raw.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
      }
   }
   fn unbind_index_buffer(&self) {
      unsafe {
         self.gl.raw.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
      }
   }

   fn create_spv_shader(&self, spv: &Vec<u8>, typ: ShaderType) -> NEResult<u32> {
      let gl = &self.gl;
      unsafe {
         let shader = gl.raw.CreateShader(gl_match_shader_type(&typ));
         gl.raw.ShaderBinary(
            1,
            &shader,
            gl::SHADER_BINARY_FORMAT_SPIR_V,
            &spv[0] as *const u8 as *const c_void,
            spv.len() as GLsizei,
         );
         let n = ptr::null();
         let entry = CString::new("main").unwrap(); //unwrap is ok here coz "main" is literal
         gl.raw.SpecializeShader(shader, entry.as_ptr(), 0, n, n);

         match gl_shader_compile_failure(shader, gl) {
            NEResult::OK(_) => NEResult::OK(shader as u32),
            NEResult::ER(e) => NEResult::ER(e),
         }
      }
   }

   //SHADERS
   fn create_src_shader(&self, src: &str, typ: ShaderType) -> NEResult<u32> {
      let src = match CString::new(src.as_bytes()) {
         Err(_) => return NEResult::ER(NEError::cstring_failed("src")),
         Ok(s) => s,
      };
      let gl = &self.gl;
      unsafe {
         let shader = gl.raw.CreateShader(gl_match_shader_type(&typ));
         gl.raw.ShaderSource(shader, 1, &src.as_ptr(), ptr::null());
         gl.raw.CompileShader(shader);

         match gl_shader_compile_failure(shader, gl) {
            NEResult::OK(_) => NEResult::OK(shader as u32),
            NEResult::ER(e) => NEResult::ER(e),
         }
      }
   }

   fn delete_shader(&self, id: u32) {
      unsafe { self.gl.raw.DeleteShader(id) }
   }

   fn create_spv_program(&self, nshdr: &NEShaderAsset) -> NEResult<u32> {
      let gl = &self.gl;
      unsafe {
         let program = gl.raw.CreateProgram();
         let v_shader = match self.create_spv_shader(&nshdr.v_spv, ShaderType::Vert) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(vs) => vs,
         };
         let f_shader = match self.create_spv_shader(&nshdr.f_spv, ShaderType::Frag) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(fs) => fs,
         };

         gl.raw.AttachShader(program, v_shader);
         gl.raw.AttachShader(program, f_shader);
         gl.raw.LinkProgram(program);

         match gl_program_link_failure(program, gl) {
            NEResult::ER(e) => NEResult::ER(e),
            NEResult::OK(_) => {
               self.delete_shader(v_shader);
               self.delete_shader(f_shader);
               NEResult::OK(program as u32)
            }
         }
      }
   }

   fn create_src_program(&self, vert: &str, frag: &str) -> NEResult<u32> {
      let gl = &self.gl;
      unsafe {
         let program = gl.raw.CreateProgram();
         let v_shader = match self.create_src_shader(vert, ShaderType::Vert) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(vs) => vs,
         };
         let f_shader = match self.create_src_shader(frag, ShaderType::Frag) {
            NEResult::ER(e) => return NEResult::ER(e),
            NEResult::OK(fs) => fs,
         };

         gl.raw.AttachShader(program, v_shader);
         gl.raw.AttachShader(program, f_shader);
         gl.raw.LinkProgram(program);

         match gl_program_link_failure(program, gl) {
            NEResult::ER(e) => NEResult::ER(e),
            NEResult::OK(_) => {
               self.delete_shader(v_shader);
               self.delete_shader(f_shader);
               NEResult::OK(program as u32)
            }
         }
      }
   }
   fn delete_program(&self, id: u32) {
      unsafe { self.gl.raw.DeleteProgram(id) }
   }

   fn create_texture(&self, tex: &NETexture) -> u32 {
      let mut id = 0;
      let gl = &self.gl;
      unsafe {
         gl.raw.GenTextures(1, &mut id);
         self.bind_texture_at(id, 0);

         let wrap = gl_match_tex_wrap(&tex.wrap);
         let (min_filter, max_filter) = gl_match_tex_filter(&tex.filter);

         gl.raw.TexParameteri(TEX, gl::TEXTURE_WRAP_S, wrap);
         gl.raw.TexParameteri(TEX, gl::TEXTURE_WRAP_T, wrap);
         gl.raw
            .TexParameteri(TEX, gl::TEXTURE_MIN_FILTER, min_filter);
         gl.raw
            .TexParameteri(TEX, gl::TEXTURE_MAG_FILTER, max_filter);

         let (base, size) = gl_match_tex_format(&tex.typ);
         let (width, height) = (tex.size.w as GLsizei, tex.size.h as GLsizei);

         gl.raw.TexImage2D(
            TEX,
            0,
            size,
            width,
            height,
            0,
            base,
            gl::UNSIGNED_BYTE,
            &tex.bytes[0] as *const u8 as *const c_void,
         );
         gl.raw.GenerateMipmap(TEX);
      }
      id as u32
   }
   fn delete_texture(&self, id: u32) {
      unsafe {
         self.gl.raw.DeleteTextures(1, &id);
      }
   }

   fn get_uni_location(&self, id: u32, name: &str) -> u32 {
      unsafe {
         let c_name = CString::new(name).unwrap();
         let location = self.gl.raw.GetUniformLocation(id, c_name.as_ptr());
         if location == -1 {
            panic!("uniform '{name}' does not exist!");
         } else {
            location as u32
         }
      }
   }
   fn set_uni(&self, id: u32, name: &str, uniform: Uniform) {
      match uniform {
         Uniform::Matrix4(m) => self.set_uni_m4f32(id, name, m),
         Uniform::Int(i) => self.set_uni_i32(id, name, i),
      }
   }

   fn set_uni_i32(&self, id: u32, name: &str, int: i32) {
      unsafe {
         let loc = self.get_uni_location(id, name) as GLint;
         self.gl.raw.Uniform1i(loc, int)
      }
   }
   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>) {
      unsafe {
         let loc = self.get_uni_location(id, name) as GLint;
         self
            .gl
            .raw
            .UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr())
      }
   }

   //BUFFERS
   fn create_buffer(&self) -> (u32, u32) {
      let (mut v_id, mut b_id): (u32, u32) = (0, 0);
      let gl = &self.gl;
      unsafe {
         gl.raw.GenVertexArrays(1, &mut v_id);
         gl.raw.GenBuffers(1, &mut b_id);
      }
      (v_id, b_id)
   }
   fn set_attr_layout(&self, attr: &ATTRInfo, attr_id: u32, stride: usize, local_offset: usize) {
      let gl = &self.gl;
      unsafe {
         gl.raw.VertexAttribPointer(
            attr_id,
            attr.elem_count as GLint,
            gl_match_attr_type(&attr.typ),
            gl::FALSE,
            stride as GLsizei,
            match local_offset {
               0 => ptr::null(),
               _ => local_offset as *const c_void,
            },
         );
         gl.raw.EnableVertexAttribArray(attr_id);
      }
   }
   fn fill_buffer(&self, v_id: u32, b_id: u32, buffer: &Vec<u8>) {
      unsafe {
         self.bind_buffer(v_id, b_id);
         self.gl.raw.BufferData(
            gl::ARRAY_BUFFER,
            (buffer.len() * 4) as GLsizeiptr,
            &buffer[0] as *const u8 as *const c_void,
            gl::DYNAMIC_DRAW,
         );
      }
   }

   fn fill_index_buffer(&self, id: u32, buffer: &Vec<u32>) {
      unsafe {
         self.bind_index_buffer(id);
         self.gl.raw.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (buffer.len() * size_of::<GLint>()) as GLsizeiptr,
            &buffer[0] as *const u32 as *const c_void,
            gl::DYNAMIC_DRAW,
         );
      }
   }

   fn delete_buffer(&self, v_id: u32, b_id: u32) {
      let gl = &self.gl;
      unsafe {
         gl.raw.DeleteVertexArrays(1, &v_id);
         gl.raw.DeleteBuffers(1, &b_id);
      }
   }

   fn create_index_buffer(&self) -> u32 {
      let mut id: u32 = 0;
      unsafe {
         self.gl.raw.GenBuffers(1, &mut id);
      }
      id
   }
   fn delete_index_buffer(&self, id: u32) {
      unsafe {
         self.gl.raw.DeleteBuffers(1, &id);
      }
   }

   //DRAW
   fn clear(&self) {
      unsafe {
         self
            .gl
            .raw
            .Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      }
   }
   fn draw_indexed(&self, draw_mode: &DrawMode, index_count: u32) {
      let draw_mode = gl_match_draw_mode(draw_mode);
      unsafe {
         self.gl.raw.DrawElements(
            draw_mode,
            index_count as GLsizei,
            gl::UNSIGNED_INT,
            ptr::null(),
         );
      }
   }
   fn draw_array(&self, draw_mode: &DrawMode, vert_count: u32) {
      let draw_mode = gl_match_draw_mode(draw_mode);
      unsafe {
         self.gl.raw.DrawArrays(draw_mode, 0, vert_count as GLsizei);
      }
   }
}

fn gl_match_draw_mode(dm: &DrawMode) -> GLenum {
   match dm {
      DrawMode::Points => gl::POINTS,
      DrawMode::Lines => gl::LINES,
      DrawMode::Triangles => gl::TRIANGLES,
      DrawMode::Strip => gl::TRIANGLE_STRIP,
   }
}
fn gl_match_shader_type(t: &ShaderType) -> GLenum {
   match t {
      ShaderType::Vert => gl::VERTEX_SHADER,
      ShaderType::Frag => gl::FRAGMENT_SHADER,
   }
}
fn gl_match_tex_format(tf: &TexFormat) -> (GLenum, GLint) {
   let base = match tf {
      TexFormat::R(_) => gl::RED,
      TexFormat::RG(_) => gl::RG,
      TexFormat::RGB(_) => gl::RGB,
      TexFormat::Palette(_) => gl::RGB,
      TexFormat::RGBA(_) => gl::RGBA,
   };
   let sized = match (base, tf.bit_depth()) {
      (gl::RED, 16) => gl::R16,
      (gl::RG, 16) => gl::RG16,
      (gl::RGB, 16) => gl::RGB16,
      (gl::RGBA, 16) => gl::RGBA16,

      (gl::RED, _) => gl::R8,
      (gl::RG, _) => gl::RG8,
      (gl::RGB, _) => gl::RGB8,
      (gl::RGBA, _) => gl::RGBA8,

      _ => gl::RGB8,
   };
   (base, sized as GLint)
}
fn gl_match_tex_filter(tf: &TexFilter) -> (GLint, GLint) {
   let (min, max) = match tf {
      TexFilter::Closest => (gl::NEAREST_MIPMAP_NEAREST, gl::NEAREST),
      TexFilter::Linear => (gl::LINEAR_MIPMAP_LINEAR, gl::LINEAR),
   };
   (min as GLint, max as GLint)
}
fn gl_match_tex_wrap(tf: &TexWrap) -> GLint {
   let wrap = match tf {
      TexWrap::Repeat => gl::REPEAT,
      TexWrap::Extend => gl::CLAMP_TO_EDGE,
      TexWrap::Clip => gl::CLAMP_TO_BORDER,
   };
   wrap as GLint
}
fn gl_match_attr_type(attr_type: &ATTRType) -> GLenum {
   match attr_type {
      ATTRType::I8 => gl::BYTE,
      ATTRType::U8 => gl::UNSIGNED_BYTE,
      ATTRType::I16 => gl::SHORT,
      ATTRType::U16 => gl::UNSIGNED_SHORT,
      ATTRType::I32 => gl::INT,
      ATTRType::U32 => gl::UNSIGNED_INT,
      ATTRType::F32 => gl::FLOAT,
      ATTRType::F64 => gl::DOUBLE,
   }
}

unsafe fn gl_shader_compile_failure(shader: GLuint, gl: &gl::Context) -> NEResult<()> {
   let mut success = gl::FALSE as GLint;
   gl.raw.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
   if success != gl::TRUE as GLint {
      let mut log_len = 0;
      gl.raw
         .GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
      println!("shad {}", log_len);
      let mut log = Vec::new();
      log.resize(log_len as usize - 1, 0);

      gl.raw.GetShaderInfoLog(
         shader,
         log_len as GLsizei,
         ptr::null_mut(),
         log.as_mut_ptr() as *mut GLchar,
      );
      println!("{:?}", log);
      let log = std::str::from_utf8(&log)
         .unwrap_or("unreachable-log")
         .to_string();
      NEResult::ER(NEError::create_shader_failed(log))
   } else {
      NEResult::OK(())
   }
}

unsafe fn gl_program_link_failure(program: GLuint, gl: &gl::Context) -> NEResult<()> {
   let mut success = gl::FALSE as GLint;
   gl.raw.GetProgramiv(program, gl::LINK_STATUS, &mut success);
   if success != gl::TRUE as GLint {
      let mut log_len = 0;
      gl.raw
         .GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);
      println!("prog {}", log_len);
      let mut log = Vec::new();
      log.resize(log_len as usize - 1, 0);

      gl.raw.GetProgramInfoLog(
         program,
         log_len as GLsizei,
         ptr::null_mut(),
         log.as_mut_ptr() as *mut GLchar,
      );
      println!("{:?}", log);
      let log = std::str::from_utf8(&log)
         .unwrap_or("unreachable-log")
         .to_string();
      NEResult::ER(NEError::create_program_failed(log))
   } else {
      NEResult::OK(())
   }
}
