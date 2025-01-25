use crate::{ansi, log_event};
use crate::{NECamera, NEEvents, NERenderer, NERuntime, NETime, NEWindow};

pub struct NEGameRef<'a> {
   pub cam: &'a mut NECamera,
   pub world: &'a mut hecs::World,
   pub renderer: &'a mut NERenderer,
   pub window: &'a mut NEWindow,
   pub events: &'a mut NEEvents,
   pub time: &'a mut NETime,
}

pub struct NEScene {
   pub(crate) name: String,
   pub(crate) world: hecs::World,
   pub(crate) runtime: Box<dyn NERuntime>,
   pub cam: NECamera,
}

struct DefaultRuntime;

impl NERuntime for DefaultRuntime {
   fn start(&mut self, _game: &mut NEGameRef) {}
   fn pre_update(&mut self, _game: &mut NEGameRef) {}
   fn update(&mut self, _game: &mut NEGameRef) {}
   fn post_update(&mut self, _game: &mut NEGameRef) {}
   fn end(&mut self, _game: &mut NEGameRef) {}
   fn render(&mut self, _game: &mut NEGameRef) {}
}

impl NEScene {
   pub fn new(name: &str) -> NEScene {
      NEScene {
         name: name.to_string(),
         world: hecs::World::new(),
         runtime: Box::new(DefaultRuntime),
         cam: NECamera::new(),
      }
   }

   pub fn replace_runtime(&mut self, runtime: Box<dyn NERuntime>) {
      self.runtime = runtime
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
      log_event!("scene [{}] run!", self.name);
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.start(&mut game_ref);
      self.cam.start()
   }

   pub(crate) fn pre_update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.pre_update(&mut game_ref);
      self.cam.pre_update()
   }

   pub(crate) fn update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.update(&mut game_ref);
      self.cam.update()
   }

   pub(crate) fn post_update(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.post_update(&mut game_ref);
      self.cam.post_update()
   }

   pub(crate) fn end(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.end(&mut game_ref);
      self.cam.end();
      log_event!("scene [{}] end!", self.name);
   }

   pub fn render(
      &mut self,
      renderer: &mut NERenderer,
      window: &mut NEWindow,
      events: &mut NEEvents,
      time: &mut NETime,
   ) {
      let mut game_ref = NEGameRef {
         cam: &mut self.cam,
         world: &mut self.world,
         renderer,
         window,
         events,
         time,
      };
      self.runtime.render(&mut game_ref);
   }
}
