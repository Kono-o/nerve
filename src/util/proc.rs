use crate::{ansi, log_fatal};
use crate::{exit, log_info, log_warn};
use std::process;

pub(crate) fn end(code: i32) {
   match code {
      exit::SUCCESS => end_success(),
      exit::ERROR => end_error(),
      _ => {
         log_warn!("exit code {} is not valid!", code);
      }
   }
}

pub(crate) fn end_success() {
   log_info!("process ended successfully! (code 0)");
   process::exit(exit::SUCCESS)
}

pub(crate) fn end_error() {
   log_fatal!("process ended due to error! (code 1)");
   process::exit(exit::ERROR)
}
