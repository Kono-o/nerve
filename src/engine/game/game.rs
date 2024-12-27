use crate::renderer::{NerveCamera, NerveRenderer};
use crate::{NerveEvents, NerveGameInfo, NerveWindow};

pub struct NerveGame {
   pub renderer: NerveRenderer,
   pub window: NerveWindow,
   pub events: NerveEvents,
   pub info: NerveGameInfo,
   pub cam: NerveCamera,
}

//PRIVATE
impl NerveGame {
   fn time_calc(&mut self) {
      self.info.time = self.window.glfw.get_time();

      let current = self.info.time;
      self.info.frame += 1;
      self.info.delta = (current - self.info.prev_time) as f32;
      self.info.prev_time = current;
      if current - self.info.prev_sec >= 1.0 {
         self.info.fps = self.info.frame as u32;
         self.info.frame = 0;
         self.info.prev_sec = current;
      }
   }
}
//PUBLIC
impl NerveGame {
   pub fn pre_update(&mut self) {
      self.time_calc();
      self.cam.recalc_view();
      self.renderer.draw_bg();

      if self.events.to_be_resized.0 {
         let new_size = self.events.to_be_resized.1;
         self.renderer.resize(new_size);
         self.cam.resize(new_size);
         self.events.to_be_resized.0 = false;
      }
      if self.events.to_be_closed {
         self.window.close();
      }
      self.window.pre_update();
      self.events.pre_update();
   }
   pub fn post_update(&mut self) {
      self.window.post_update();
      self.events.post_update()
   }

   pub fn set_cam(&mut self, camera: NerveCamera) {
      self.cam = camera
   }
}
