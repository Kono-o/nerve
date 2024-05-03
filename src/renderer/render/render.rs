use crate::RGB;

pub enum PolygonMode {
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

   pub fn fill() {
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
   }

   pub fn polygon_mode(mode: PolygonMode) {
      unsafe {
         gl::PolygonMode(
            gl::FRONT_AND_BACK,
            match mode {
               PolygonMode::WireFrame => gl::LINE,
               PolygonMode::Filled => gl::FILL,
            },
         )
      }
   }
}
