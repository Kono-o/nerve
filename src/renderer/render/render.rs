use crate::RGB;

pub enum PolygonMode {
   Points,
   WireFrame,
   Filled,
}

pub struct NerveRenderer;
impl NerveRenderer {
   pub fn set_bg(color: RGB) {
      let color = color.to_rgba(1.0);
      unsafe { gl::ClearColor(color.0, color.1, color.2, color.3) }
   }

   pub fn enable_depth(enable: bool) {
      unsafe {
         if enable {
            gl::Enable(gl::DEPTH_TEST)
         } else {
            gl::Disable(gl::DEPTH_TEST)
         }
      }
   }

   pub fn resize(width: i32, height: i32) {
      unsafe { gl::Viewport(0, 0, width, height) }
   }
   pub fn fill() {
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
   }

   pub fn polygon_mode(mode: PolygonMode) {
      unsafe {
         match mode {
            PolygonMode::WireFrame => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
            PolygonMode::Filled => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),

            PolygonMode::Points => {
               gl::PointSize(10.0);
               gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT)
            }
         }
      }
   }
   pub fn enable_msaa(enable: bool) {
      unsafe {
         if enable {
            gl::Enable(gl::MULTISAMPLE)
         } else {
            gl::Disable(gl::MULTISAMPLE)
         }
      }
   }
}
