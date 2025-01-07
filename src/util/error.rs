use crate::asset::{NEFileErrKind, NEObjErrKind};
use crate::engine::NEInitErrKind;
use crate::util::consts::ansi;
use crate::{log_fatal, log_warn, proc};

#[derive(Copy, Clone)]
pub enum NEErrorSeverity {
   Warn,
   Fatal,
}

pub enum NEError {
   Init {
      kind: NEInitErrKind,
   },
   File {
      kind: NEFileErrKind,
      path: String,
   },
   Obj {
      kind: NEObjErrKind,
      path: String,
   },
   Custom {
      severity: NEErrorSeverity,
      msg: String,
   },
}

impl NEError {
   pub fn custom(severity: NEErrorSeverity, msg: &str) -> NEError {
      NEError::Custom {
         severity,
         msg: msg.to_string(),
      }
   }
   pub fn msg(&self) -> (NEErrorSeverity, String) {
      let mut severe = NEErrorSeverity::Warn;
      let msg = match self {
         NEError::Init { kind } => {
            let kind_msg = match kind {
               NEInitErrKind::GlfwInit => "glfw init failed",
               NEInitErrKind::APIUnavailable(api) => &format!("{api} unavailable"),
               NEInitErrKind::APIWrongVersion(api) => &format!("{api} invalid version"),
               NEInitErrKind::APIUnsupported(api) => &format!("{api} unsupported"),
               NEInitErrKind::NoMonitor => "no monitor found",
               NEInitErrKind::NotVidMode => "no vid mode found",
               NEInitErrKind::WindowHasNoContext => "window has no context",
               NEInitErrKind::CouldNotMakeWindow => "could not make window",
               NEInitErrKind::Unknown(desc) => &format!("unknown error [{desc}]"),
            };
            severe = NEErrorSeverity::Fatal;
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
            severe = NEErrorSeverity::Fatal;
            format!("(file) -> {kind_msg} [{path}]")
         }
         NEError::Obj { kind, path } => {
            let kind_msg = match kind {
               NEObjErrKind::NonTriMesh => "not triangulated!",
            };
            format!("(obj) -> {kind_msg} [{path}]")
         }
         NEError::Custom { severity, msg } => {
            severe = *severity;
            format!("(custom) -> {msg}")
         }
      };
      (severe, format!("NERVE ERROR: {msg}"))
   }
   pub fn log(&self) {
      let (severe, msg) = self.msg();
      match severe {
         NEErrorSeverity::Warn => {
            log_warn!("{msg}");
         }
         NEErrorSeverity::Fatal => {
            log_fatal!("{msg}");
            proc::end_error()
         }
      }
   }
}
