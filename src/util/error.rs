use crate::asset::{NEFileErrKind, NEObjErrKind};
use crate::engine::NEInitErrKind;
use crate::util::consts::{ansi, exit};
use std::process;

macro_rules! colprintln {
    ($fmt:expr, $color:expr) => {
       let fmt = format!($fmt);
        println!("{}{}{}", $color.0, fmt, $color.1);
    };
    ($fmt:expr, $color:expr, $($args:tt)*) => {
       let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", $color.0, fmt, $color.1);
    };
}

pub enum NEError {
   Init { kind: NEInitErrKind },
   File { kind: NEFileErrKind, path: String },
   Obj { kind: NEObjErrKind, path: String },
}

impl NEError {
   pub fn msg(&self) -> String {
      let msg = match self {
         NEError::Init { kind } => {
            let kind_msg = match kind {
               NEInitErrKind::GlfwInit => "glfw init failed",
               NEInitErrKind::APIUnavailable(api) => &format!("{api} is unavailable"),
               NEInitErrKind::APIWrongVersion(api) => &format!("{api} unsupported/invalid version"),
               NEInitErrKind::NoMonitor => "no monitor found",
               NEInitErrKind::NotVidMode => "no vid mode found",
               NEInitErrKind::WindowHasNoContext => "window has no context",
               NEInitErrKind::CouldNotMakeWindow => "could not make window",
               NEInitErrKind::Unknown(desc) => &format!("unknown error {desc}"),
            };
            format!("(init) -> {kind_msg}")
         }
         NEError::File { kind, path } => {
            let kind_msg = match kind {
               NEFileErrKind::NoFile => "does not exist",
               NEFileErrKind::NoPerms => "no permissions",
               NEFileErrKind::NotValidPath => "path not valid",
               NEFileErrKind::Unsupported => "unsupported type",
               NEFileErrKind::Unknown => "unknown error",
            };
            format!("(file) -> {kind_msg} [{path}]")
         }
         NEError::Obj { kind, path } => {
            let kind_msg = match kind {
               NEObjErrKind::NonTriMesh => "not triangulated!",
            };
            format!("(obj) -> {kind_msg} [{path}]")
         }
      };
      format!("NERVE ERROR: {msg}")
   }
   pub fn log(&self) {
      let msg = self.msg();

      colprintln!("{msg}", ansi::BRIGHT_RED);
   }
   pub fn log_and_exit(&self) {
      self.log();
      process::exit(exit::ERROR);
   }
}
