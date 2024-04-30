use crate::renderer::render::RGBA;

pub enum PolygonMode {
   WireFrame,
   Filled,
}
pub struct NerveRender;
impl NerveRender {
   pub fn set_fill(col: RGBA) {
      unsafe { gl::ClearColor(col.0, col.1, col.2, col.3) }
   }
   pub fn fill() {
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
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
