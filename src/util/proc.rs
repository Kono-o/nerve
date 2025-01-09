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
   let code = exit::SUCCESS;
   log_info!("process ended successfully! (code {code})");
   process::exit(code)
}

pub(crate) fn end_error() {
   let code = exit::ERROR;
   log_fatal!("process ended due to fatal error! (code {code})");
   process::exit(code)
}
