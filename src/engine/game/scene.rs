use crate::{NECamera, NEEvents, NEObject, NERenderer, NETime, NEWindow};

pub struct NEGameRef<'a> {
   pub camera: &'a mut NECamera,
   pub renderer: &'a mut NERenderer,
   pub window: &'a mut NEWindow,
   pub events: &'a mut NEEvents,
   pub time: &'a mut NETime,
}

pub struct NEScene {
   pub(crate) objects: Vec<Box<dyn NEObject>>,
   pub cam: NECamera,
}

impl NEScene {
   pub fn new() -> NEScene {
      NEScene {
         objects: Vec::new(),
         cam: NECamera::new(),
      }
   }

   pub fn attach_object(&mut self, object: Box<dyn NEObject>) -> u32 {
      self.objects.push(object);
      (self.objects.len() - 1) as u32
   }

   pub fn replace_cam(&mut self, cam: NECamera) {
      self.cam = cam;
   }
}

impl NEScene {
   pub(crate) fn start(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         camera: &mut self.cam,
         renderer,
         window,
         events,
         time,
      };
      for mut object in self.objects.iter_mut() {
         object.start(&mut game_ref)
      }
   }

   pub(crate) fn pre_update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      self.cam.pre_update();
      let mut game_ref = NEGameRef {
         camera: &mut self.cam,
         renderer,
         window,
         events,
         time,
      };
      for mut object in self.objects.iter_mut() {
         object.pre_update(&mut game_ref)
      }
   }

   pub(crate) fn update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         camera: &mut self.cam,
         renderer,
         window,
         events,
         time,
      };
      for mut object in self.objects.iter_mut() {
         object.update(&mut game_ref)
      }
   }

   pub(crate) fn post_update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         camera: &mut self.cam,
         renderer,
         window,
         events,
         time,
      };
      for mut object in self.objects.iter_mut() {
         object.post_update(&mut game_ref)
      }
   }

   pub(crate) fn end(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         camera: &mut self.cam,
         renderer,
         window,
         events,
         time,
      };
      for mut object in self.objects.iter_mut() {
         object.end(&mut game_ref)
      }
   }
}
