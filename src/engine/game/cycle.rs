pub struct NECycle {
   pub is_paused: bool,
}

impl NECycle {
   pub fn pause(&mut self) {
      self.is_paused = true
   }
   pub fn unpause(&mut self) {
      self.is_paused = false
   }
   pub fn toggle_pause(&mut self) {
      self.is_paused = !self.is_paused
   }
}
