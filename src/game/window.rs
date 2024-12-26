use crate::WinSize;
use glfw::{Context, Glfw, PWindow, WindowMode};

pub struct NerveWindow {
   pub(crate) glfw: Glfw,
   pub(crate) window: PWindow,
   pub(crate) is_fullscreen: bool,
   pub(crate) size: WinSize,
}

impl NerveWindow {
   pub(crate) fn set_monitor(
      &mut self,
      mode: WindowMode,
      prev_pos: (i32, i32),
      prev_size: WinSize,
      refresh_rate: Option<u32>,
   ) {
      self.window.set_monitor(
         mode,
         prev_pos.0,
         prev_pos.1,
         prev_size.w,
         prev_size.h,
         refresh_rate,
      );
   }

   pub(crate) fn get_cursor_pos(&self) -> (f64, f64) {
      self.window.get_cursor_pos()
   }

   pub(crate) fn get_pos(&self) -> (i32, i32) {
      self.window.get_pos()
   }

   pub(crate) fn swap_buffers(&mut self) {
      self.window.swap_buffers()
   }
   pub(crate) fn should_close(&self) -> bool {
      self.window.should_close()
   }
   pub(crate) fn set_should_close(&mut self, should_close: bool) {
      self.window.set_should_close(should_close);
   }
   pub(crate) fn get_size(&mut self) -> WinSize {
      let (w, h) = self.window.get_size();
      WinSize::from(w as u32, h as u32)
   }
   pub(crate) fn set_size(&mut self, size: WinSize) {
      self.window.set_size(size.w as i32, size.h as i32);
   }
}
