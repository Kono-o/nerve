use crate::asset::file::NEFileErrKind;
use crate::asset::NEAssetErrKind;
use crate::engine::NEInitErrKind;
use crate::util::consts::ansi;
use crate::util::env;
use crate::{log_fatal, log_warn, proc, NECompileErrKind, NEOpenGLErrKind};

#[derive(Copy, Clone)]
pub enum NEErrorSeverity {
   Warn,
   Fatal,
}

pub enum NEError {
   Init {
      kind: NEInitErrKind,
   },
   OpenGL {
      kind: NEOpenGLErrKind,
   },
   File {
      kind: NEFileErrKind,
      path: String,
   },
   Asset {
      kind: NEAssetErrKind,
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
   pub(crate) fn vert_missing(path: &str) -> NEError {
      NEError::Asset {
         kind: NEAssetErrKind::VertEmpty,
         path: path.to_string(),
      }
   }

   pub(crate) fn frag_missing(path: &str) -> NEError {
      NEError::Asset {
         kind: NEAssetErrKind::FragEmpty,
         path: path.to_string(),
      }
   }

   pub(crate) fn non_triangle_mesh(path: &str) -> NEError {
      NEError::Asset {
         kind: NEAssetErrKind::NonTriangleMesh,
         path: path.to_string(),
      }
   }

   pub(crate) fn file_missing(path: &str) -> NEError {
      NEError::File {
         kind: NEFileErrKind::Missing,
         path: path.to_string(),
      }
   }

   pub(crate) fn file_invalid(path: &str) -> NEError {
      NEError::File {
         kind: NEFileErrKind::NotValid,
         path: path.to_string(),
      }
   }

   pub(crate) fn file_unsupported(path: &str, ex: &str) -> NEError {
      NEError::File {
         kind: NEFileErrKind::Unsupported(ex.to_string()),
         path: path.to_string(),
      }
   }

   pub(crate) fn file_couldnt_make(path: &str) -> NEError {
      NEError::File {
         kind: NEFileErrKind::CouldNotMake,
         path: path.to_string(),
      }
   }

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
         NEError::OpenGL { kind } => {
            let kind_msg = match kind {
               NEOpenGLErrKind::NoActiveContext => "no active context found",
               NEOpenGLErrKind::CouldParseVersion(s) => &format!("could not parse version {s}"),
               NEOpenGLErrKind::CStringFailed => "cstring failed",
               NEOpenGLErrKind::SPIRVNotFound => &format!(
                  "could not find [{}] or [{}]",
                  env::SPIRV_EXTENSIONS,
                  env::GL_SPIRV
               ),
            };
            severe = NEErrorSeverity::Fatal;
            format!("(opengl) -> {kind_msg}!")
         }
         NEError::File { kind, path } => {
            let kind_msg = match kind {
               NEFileErrKind::Missing => "does not exist",
               NEFileErrKind::NotValid => "is invalid",

               NEFileErrKind::CouldNotMake => "could not be created",
               NEFileErrKind::CouldNotRead => "could not be read",
               NEFileErrKind::CouldNotWrite => "could not be written",

               NEFileErrKind::NoPerms => "needs higher perms",

               NEFileErrKind::Unsupported(ex) => &format!(".{} is an unsupported format", ex),
               NEFileErrKind::Unknown => "unknown error",
            };
            severe = NEErrorSeverity::Fatal;
            format!("(file) -> {kind_msg}! [{path}]")
         }
         NEError::Asset { kind, path } => {
            let kind_msg = match kind {
               NEAssetErrKind::VertEmpty => "has no vertex src",
               NEAssetErrKind::FragEmpty => "has no fragment src",
               NEAssetErrKind::NonTriangleMesh => "mesh not triangulated",
            };
            severe = NEErrorSeverity::Fatal;
            format!("(asset) -> {kind_msg}! [{path}]")
         }
         NEError::Compile { kind, path, msg } => {
            let kind_msg = match kind {
               NECompileErrKind::NoGLSLValidator => &format!(
                  "[{}] does not exist, install Vulkan SDK from {}",
                  env::GLSL_VALIDATOR,
                  env::VULKAN_SDK_URL
               ),
               NECompileErrKind::GLSLCompileFailed => "compilation failed",
               NECompileErrKind::CreateProgramFailed => "program creation failed",
               NECompileErrKind::CreateShaderFailed => "shader creation failed",
               NECompileErrKind::CStringFailed => "could not parse src into c-str",
            };
            severe = NEErrorSeverity::Fatal;
            if msg.len() == 0 {
               format!("(spirv) -> {kind_msg}! [{path}]")
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
            log_fatal!("{msg}");
            proc::end_error()
         }
      }
   }
}
