use crate::asset::ATTRInfo;
use crate::renderer::{Renderer, ShaderType, TexFilter, TexFormat};
use crate::{
   ATTRType, Cull, DrawMode, NEResult, NETexture, PolyMode, Size2D, TexWrap, Uniform, RGB,
};
use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLsizeiptr};
use glfw::{Context, PWindow};
use std::ffi::{c_void, CStr, CString};
use std::ptr;

#[derive(Copy, Clone)]
pub(crate) struct GLRenderer;

const TEX: GLenum = gl::TEXTURE_2D;

impl Renderer for GLRenderer {
   fn init(&self, _name: String, window: &mut PWindow) -> NEResult<()> {
      window.make_current();
      gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
      NEResult::OK(())
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
   fn set_clear(&self, color: RGB) {
      unsafe {
         gl::ClearColor(color.0, color.1, color.2, 1.0);
      }
   }
   fn resize(&self, size: Size2D) {
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

   fn bind_program(&self, prog_id: u32) {
      unsafe { gl::UseProgram(prog_id) }
   }
   fn unbind_program(&self) {
      unsafe { gl::UseProgram(0) }
   }

   fn bind_texture_at(&self, tex_id: u32, slot: u32) {
      unsafe {
         gl::ActiveTexture(gl::TEXTURE0 + slot);
         gl::BindTexture(TEX, tex_id);
      }
   }
   fn unbind_texture(&self) {
      unsafe { gl::BindTexture(TEX, 0) }
   }

   fn bind_buffer(&self, v_id: u32, b_id: u32) {
      unsafe {
         gl::BindVertexArray(v_id);
         gl::BindBuffer(gl::ARRAY_BUFFER, b_id);
      }
   }
   fn unbind_buffer(&self) {
      unsafe {
         gl::BindVertexArray(0);
         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      }
   }

   fn bind_index_buffer(&self, id: u32) {
      unsafe {
         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
      }
   }

   fn unbind_index_buffer(&self) {
      unsafe {
         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
      }
   }

   //SHADERS
   fn create_shader(&self, src: &str, typ: ShaderType) -> u32 {
      let log_len = 256;
      let mut log = Vec::with_capacity(log_len);
      let mut success = gl::FALSE as GLint;
      let src = CString::new(src.as_bytes()).expect("src empty!");
      unsafe {
         let shader = gl::CreateShader(match_shader_type_gl(&typ));
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
         shader as u32
      }
   }
   fn delete_shader(&self, id: u32) {
      unsafe { gl::DeleteShader(id) }
   }

   fn create_program(&self, vert: &str, frag: &str) -> u32 {
      let log_len = 256;
      let mut log = Vec::with_capacity(log_len);
      let mut success = gl::FALSE as GLint;
      unsafe {
         let program = gl::CreateProgram();
         let vert_shader = self.create_shader(vert, ShaderType::Vert);
         let frag_shader = self.create_shader(frag, ShaderType::Frag);

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
         self.delete_shader(vert_shader);
         self.delete_shader(frag_shader);
         program as u32
      }
   }
   fn delete_program(&self, id: u32) {
      unsafe { gl::DeleteProgram(id) }
   }

   fn create_texture(&self, tex: &NETexture) -> u32 {
      let mut id = 0;
      unsafe {
         gl::GenTextures(1, &mut id);
         self.bind_texture_at(id, 0);

         let wrap = match_tex_wrap_gl(&tex.wrap);
         let (min_filter, max_filter) = match_tex_filter_gl(&tex.filter);

         gl::TexParameteri(TEX, gl::TEXTURE_WRAP_S, wrap);
         gl::TexParameteri(TEX, gl::TEXTURE_WRAP_T, wrap);
         gl::TexParameteri(TEX, gl::TEXTURE_MIN_FILTER, min_filter);
         gl::TexParameteri(TEX, gl::TEXTURE_MAG_FILTER, max_filter);

         let (base, size) = match_tex_format_gl(&tex.typ);
         let (width, height) = (tex.size.w as GLsizei, tex.size.h as GLsizei);

         gl::TexImage2D(
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
         gl::GenerateMipmap(TEX);
      }
      id as u32
   }
   fn delete_texture(&self, id: u32) {
      unsafe {
         gl::DeleteTextures(1, &id);
      }
   }

   fn get_uni_location(&self, id: u32, name: &str) -> u32 {
      unsafe {
         let c_name = CString::new(name).unwrap();
         let location = gl::GetUniformLocation(id, c_name.as_ptr());
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
         gl::Uniform1i(loc, int)
      }
   }
   fn set_uni_m4f32(&self, id: u32, name: &str, matrix: Matrix4<f32>) {
      unsafe {
         let loc = self.get_uni_location(id, name) as GLint;
         gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr())
      }
   }

   //BUFFERS
   fn create_buffer(&self) -> (u32, u32) {
      let (mut v_id, mut b_id): (u32, u32) = (0, 0);
      unsafe {
         gl::GenVertexArrays(1, &mut v_id);
         gl::GenBuffers(1, &mut b_id);
      }
      (v_id, b_id)
   }
   fn set_attr_layout(&self, attr: &ATTRInfo, attr_id: u32, stride: usize, local_offset: usize) {
      unsafe {
         gl::VertexAttribPointer(
            attr_id,
            attr.elem_count as GLint,
            match_attr_type(&attr.typ),
            gl::FALSE,
            stride as GLsizei,
            match local_offset {
               0 => ptr::null(),
               _ => local_offset as *const c_void,
            },
         );
         gl::EnableVertexAttribArray(attr_id);
      }
   }
   fn fill_buffer(&self, v_id: u32, b_id: u32, buffer: &Vec<u8>) {
      unsafe {
         self.bind_buffer(v_id, b_id);
         gl::BufferData(
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
         gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (buffer.len() * size_of::<GLint>()) as GLsizeiptr,
            &buffer[0] as *const u32 as *const c_void,
            gl::DYNAMIC_DRAW,
         );
      }
   }

   fn delete_buffer(&self, v_id: u32, b_id: u32) {
      unsafe {
         gl::DeleteVertexArrays(1, &v_id);
         gl::DeleteBuffers(1, &b_id);
      }
   }

   fn create_index_buffer(&self) -> u32 {
      let mut id: u32 = 0;
      unsafe {
         gl::GenBuffers(1, &mut id);
      }
      id
   }
   fn delete_index_buffer(&self, id: u32) {
      unsafe {
         gl::DeleteBuffers(1, &id);
      }
   }

   //DRAW
   fn clear(&self) {
      unsafe {
         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      }
   }
   fn draw(&self, draw_mode: &DrawMode, index_count: u32) {
      let draw_mode = match_draw_mode_gl(draw_mode);
      unsafe {
         gl::DrawElements(
            draw_mode,
            index_count as GLsizei,
            gl::UNSIGNED_INT,
            ptr::null(),
         );
      }
   }
   fn draw_no_index(&self, draw_mode: &DrawMode, vert_count: u32) {
      let draw_mode = match_draw_mode_gl(draw_mode);
      unsafe {
         gl::DrawArrays(draw_mode, 0, vert_count as GLsizei);
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
fn match_shader_type_gl(t: &ShaderType) -> GLenum {
   match t {
      ShaderType::Vert => gl::VERTEX_SHADER,
      ShaderType::Frag => gl::FRAGMENT_SHADER,
   }
}
fn match_tex_format_gl(tf: &TexFormat) -> (GLenum, GLint) {
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

fn match_tex_filter_gl(tf: &TexFilter) -> (GLint, GLint) {
   let (min, max) = match tf {
      TexFilter::Closest => (gl::NEAREST_MIPMAP_NEAREST, gl::NEAREST),
      TexFilter::Linear => (gl::LINEAR_MIPMAP_LINEAR, gl::LINEAR),
   };
   (min as GLint, max as GLint)
}

fn match_tex_wrap_gl(tf: &TexWrap) -> GLint {
   let wrap = match tf {
      TexWrap::Repeat => gl::REPEAT,
      TexWrap::Extend => gl::CLAMP_TO_EDGE,
      TexWrap::Clip => gl::CLAMP_TO_BORDER,
   };
   wrap as GLint
}

fn match_attr_type(attr_type: &ATTRType) -> GLenum {
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
