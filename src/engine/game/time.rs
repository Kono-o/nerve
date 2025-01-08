use glfw::Glfw;
use std::time::Instant;

pub struct NETime {
   pub fps: f64,
   pub delta: f64,
   pub frame: u64,
   pub elapsed: f64,

   pub(crate) glfw: Glfw,
   pub(crate) prev_sec: Instant,
   pub(crate) prev_time: Instant,
   pub(crate) prev_deltas: Vec<f64>,
   pub(crate) prev_deltas_size: usize,
   pub(crate) start_time: Instant,
   pub(crate) current_time: Instant,
   pub(crate) local_frame: u32,
}

impl NETime {
   pub(crate) fn pre_update(&mut self) {
      self.calculate()
   }
   pub(crate) fn post_update(&mut self) {}

   fn calculate(&mut self) {
      self.frame += 1;
      self.local_frame += 1;
      self.current_time = Instant::now();

      self.elapsed = self
         .current_time
         .duration_since(self.start_time)
         .as_secs_f64();

      self.delta = self
         .current_time
         .duration_since(self.prev_time)
         .as_secs_f64();

      self.prev_time = self.current_time;

      self.prev_deltas.push(self.delta);
      if self.prev_deltas.len() > self.prev_deltas_size {
         self.prev_deltas.remove(0);
      }

      let avg_delta = self.prev_deltas.iter().sum::<f64>() / self.prev_deltas.len() as f64;
      self.fps = 1.0 / avg_delta;

      if self
         .current_time
         .duration_since(self.prev_sec)
         .as_secs_f32()
         >= 1.0
      {
         self.local_frame = 0;
         self.prev_sec = self.current_time
      }
   }
}
