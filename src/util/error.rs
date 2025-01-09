use crate::asset::{NEFileErrKind, NEGLSLErrKind, NEObjErrKind};
use crate::engine::NEInitErrKind;
use crate::util::consts::ansi;
use crate::{log_warn, proc, NECompileErrKind};

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
   GLSL {
      kind: NEGLSLErrKind,
      path: String,
   },
   Compile {
      kind: NECompileErrKind,
      path: String,
      msg: String,
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
               NEInitErrKind::NoVidMode => "no vid mode found",
               NEInitErrKind::WindowHasNoContext => "window has no context",
               NEInitErrKind::CouldNotMakeWindow => "could not make window",
               NEInitErrKind::Unknown(desc) => &format!("unknown error [{desc}]"),
            };
            severe = NEErrorSeverity::Fatal;
            format!("(init) -> {kind_msg}!")
         }
         NEError::File { kind, path } => {
            let kind_msg = match kind {
               NEFileErrKind::NoFile => "file does not exist",
               NEFileErrKind::NoPath => "path does not exist",

               NEFileErrKind::NotValidPath => "invalid path",
               NEFileErrKind::NotValidName => "invalid name",

               NEFileErrKind::CouldNotCreate => "could not create file",
               NEFileErrKind::CouldNotWrite => "could not write to file",

               NEFileErrKind::NoPerms => "no permissions",

               NEFileErrKind::Unsupported => "unsupported format",
               NEFileErrKind::Unknown => "unknown error",
            };
            severe = NEErrorSeverity::Fatal;
            format!("(file) -> {kind_msg}! [{path}]")
         }
         NEError::Obj { kind, path } => {
            let kind_msg = match kind {
               NEObjErrKind::NonTriMesh => "not triangulated",
            };
            severe = NEErrorSeverity::Fatal;
            format!("(obj) -> {kind_msg}! [{path}]")
         }
         NEError::GLSL { kind, path } => {
            let kind_msg = match kind {
               NEGLSLErrKind::IsEmpty => "has no src",
            };
            severe = NEErrorSeverity::Fatal;
            format!("(src) -> {kind_msg}! [{path}]")
         }
         NEError::Compile { kind, path, msg } => {
            let kind_msg = match kind {
               NECompileErrKind::NoGLSLValidator => {
                  "[glsl-lang-validator] does not exist, install Vulkan SDK from https://vulkan.lunarg.com/sdk/home"
               }
               NECompileErrKind::CompileFailed => {"compilation failed"}
               NECompileErrKind::CStringFailed => {"could not parse src into c-str"}
            };
            severe = NEErrorSeverity::Fatal;
            if path.len() == 0 {
               format!("(spirv) -> {kind_msg}! {msg}")
            } else {
               format!("(spirv) -> {kind_msg}! {msg} [{path}]")
            }
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
            log_warn!("{msg}");
            proc::end_error()
         }
      }
   }
}
