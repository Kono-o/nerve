use glfw::Glfw;
use std::time::{Duration, Instant};

pub struct NETime {
   pub(crate) tps: f64,
   pub(crate) delta: f64,
   pub(crate) tick_count: u64,
   pub(crate) elapsed: f64,

   pub(crate) glfw: Glfw,
   pub(crate) prev_sec: Instant,
   pub(crate) prev_time: Instant,
   pub(crate) prev_deltas: Vec<f64>,
   pub(crate) prev_deltas_size: usize,
   pub(crate) start_time: Instant,
   pub(crate) current_time: Instant,
   pub(crate) local_tick: u32,
}

impl NETime {
   pub fn fps(&self) -> f64 {
      self.tps
   }
   pub fn delta(&self) -> f64 {
      self.delta
   }

   pub fn elapsed(&self) -> f64 {
      self.elapsed
   }

   pub fn now(&self) -> f64 {
      Instant::now().duration_since(self.start_time).as_secs_f64()
   }
   pub fn now_as_ms(&self) -> u64 {
      Instant::now().duration_since(self.start_time).as_millis() as u64
   }
   pub fn now_as_ns(&self) -> u64 {
      Instant::now().duration_since(self.start_time).as_nanos() as u64
   }

   pub fn sleep(&self, secs: f64) {
      let duration = Duration::from_secs_f64(secs);
      std::thread::sleep(duration)
   }

   pub fn sleep_ms(&self, millis: u64) {
      let duration = Duration::from_millis(millis);
      std::thread::sleep(duration)
   }

   pub fn sleep_ns(&self, nanos: u64) {
      let duration = Duration::from_nanos(nanos);
      std::thread::sleep(duration)
   }

   pub(crate) fn pre_update(&mut self) {
      self.calculate()
   }
   pub(crate) fn post_update(&mut self) {}

   fn calculate(&mut self) {
      self.tick_count += 1;
      self.local_tick += 1;
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
      self.tps = 1.0 / avg_delta;

      if self
         .current_time
         .duration_since(self.prev_sec)
         .as_secs_f32()
         >= 1.0
      {
         self.local_tick = 0;
         self.prev_sec = self.current_time
      }
   }
}
