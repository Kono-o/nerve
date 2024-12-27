use crate::WinSize;
use glfw::{Context, Glfw, PWindow, SwapInterval, WindowMode};

pub struct NerveWindow {
   pub(crate) glfw: Glfw,
   pub(crate) window: PWindow,

   pub(crate) prev_cursor_pos: (u32, u32),
   pub(crate) cursor_offset: (i32, i32),
   pub(crate) prev_pos: (i32, i32),
   pub(crate) prev_size: WinSize,

   pub is_fullscreen: bool,
   pub is_resizable: bool,
   pub is_running: bool,
   pub is_vsync: bool,

   pub size: WinSize,
   pub pos: (i32, i32),
   pub title: String,
   pub cursor_pos: (u32, u32),
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
   pub(crate) fn pre_update(&mut self) {
      self.size = self.get_size();
      self.pos = self.window.get_pos();
      self.cursor_pos = self.get_cursor_pos();
      self.cursor_offset = self.get_cursor_offset();
      self.glfw.poll_events()
   }
   pub(crate) fn post_update(&mut self) {
      self.swap();
   }

   fn get_size(&mut self) -> WinSize {
      let (w, h) = self.window.get_size();
      WinSize::from(w as u32, h as u32)
   }
   fn get_pos(&self) -> (i32, i32) {
      self.window.get_pos()
   }
   fn get_cursor_pos(&self) -> (u32, u32) {
      let (x, y) = self.window.get_cursor_pos();
      (x as u32, y as u32)
   }
   fn get_cursor_offset(&mut self) -> (i32, i32) {
      let (x, y) = self.cursor_pos;
      self.prev_cursor_pos = (x, y);
      (
         x as i32 - self.prev_cursor_pos.0 as i32,
         self.prev_cursor_pos.1 as i32 - y as i32,
      )
   }

   fn swap(&mut self) {
      self.window.swap_buffers()
   }

   pub fn close(&mut self) {
      self.window.set_should_close(false);
      self.is_running = false
   }
   pub fn set_size(&mut self, size: WinSize) {
      self.window.set_size(size.w as i32, size.h as i32);
   }
   pub fn set_fullscreen(&mut self, enable: bool) {
      if self.is_fullscreen != enable {
         self.is_fullscreen = enable;
         self.toggle_fullscreen()
      }
   }
   pub fn toggle_fullscreen(&mut self) {
      if self.is_fullscreen {
         self.set_monitor(
            WindowMode::Windowed,
            self.prev_pos,
            self.prev_size.shave(1),
            None,
         );
      } else {
         self.prev_pos = self.get_pos();
         self.prev_size = self.get_size();

         self.glfw.with_primary_monitor(|_, m| {
            let monitor = m.expect("no monitor found!");
            let mode = monitor.get_video_mode().unwrap();
            self.window.set_monitor(
               WindowMode::FullScreen(&monitor),
               0,
               0,
               mode.width,
               mode.height,
               Some(mode.refresh_rate),
            );
         })
      }
      self.is_fullscreen = !self.is_fullscreen;
   }
   pub fn set_resizable(&mut self, enable: bool) {
      if self.is_resizable != enable {
         self.is_resizable = enable;
         self.toggle_resizable()
      }
   }
   pub fn toggle_resizable(&mut self) {
      self.is_resizable = !self.is_resizable;
      self.window.set_resizable(self.is_resizable);
   }
   pub fn set_vsync(&mut self, enable: bool) {
      self.window.glfw.set_swap_interval(match enable {
         true => SwapInterval::Adaptive,
         false => SwapInterval::None,
      });
      self.is_vsync = enable;
   }
   pub fn toggle_vsync(&mut self) {
      self.is_vsync = !self.is_vsync;
      self.set_vsync(self.is_vsync);
   }
}
