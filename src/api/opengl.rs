use crate::api::r#trait::Renderer;
use crate::{Cull, PolyMode, WinSize, RGB};
use gl::types::GLsizei;
use glfw::{Context, Glfw, PWindow};

#[derive(Copy, Clone)]
pub(crate) struct GLRenderer;

impl Renderer for GLRenderer {
   fn init(&self, window: &mut PWindow, _glfw: &mut Glfw) {
      window.make_current();
      gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
   }

   //RENDERING
   fn set_bg(&self, color: RGB) {
      unsafe {
         gl::ClearColor(color.0, color.1, color.2, 1.0);
      }
   }
   fn clear_bg(&self) {
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
   }
   fn clear_depth(&self) {
      unsafe {
         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
   fn wire_thickness(&self, thickness: f32) {
      unsafe { gl::LineWidth(thickness) }
   }

   //SHADERS

   //BUFFERS
}
