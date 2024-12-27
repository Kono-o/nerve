use crate::renderer::{NerveCamera, NerveRenderer};
use crate::{NerveEvents, NerveGameInfo, NerveWindow, WinSize};

pub struct NerveGame {
   pub renderer: NerveRenderer,
   pub window: NerveWindow,
   pub events: NerveEvents,
   pub info: NerveGameInfo,
   pub cam: NerveCamera,
}

impl NerveGame {
   fn resize_children(&mut self, new_size: WinSize) {
      self.renderer.set_size(new_size);
      self.cam.set_size(new_size);
   }
   fn handle_events(&mut self) {
      if self.events.window_resize_event.0 {
         self.resize_children(self.events.window_resize_event.1);
         self.events.window_resize_event.0 = false;
      }

      if self.events.window_close_event {
         self.window.close();
      }
   }

   pub fn pre_update(&mut self) {
      self.renderer.pre_update();
      self.window.pre_update();
      self.events.pre_update();
      self.info.pre_update();
      self.cam.pre_update();
      self.handle_events();
   }
   pub fn post_update(&mut self) {
      self.renderer.post_update();
      self.window.post_update();
      self.events.post_update();
      self.info.post_update();
      self.cam.post_update();
   }

   pub fn set_cam(&mut self, camera: NerveCamera) {
      self.cam = camera
   }
}
