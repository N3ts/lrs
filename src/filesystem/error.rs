// Standard Libraries
use std::path::PathBuf;

use derive_more::From;

use super::{ExitStatus, IntoExitStatus};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
  PathInvalidUTF8Error {
    path: PathBuf,
  },
  FileInvalidUTF8Error {
    path: String,
  },
  CannotOpenDirectoryError {
    path: PathBuf,
    message: String
  },
  #[from]
  CannotReadFileInDirectoryError {
    name: String,
    message: String
  },
  CannotAccessFileError {
    name: String,
    message: String,
  },
  CannotReadSymbolicLinkError {
    path: PathBuf,
    message: String,
  },
  CannotDetermineDevInoError {
    path: PathBuf,
    message: String,
  },
  DirectoryAlreadyListedError { 
    name: String
  }
}

impl IntoExitStatus for Error {
  fn into_exit_status(&self, cli_arg: bool) -> ExitStatus {
    ExitStatus::from(cli_arg)
  }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Self::PathInvalidUTF8Error { path  } => write!(fmt, "path contains invalid UTF-8: {:?}", path),
      Self::FileInvalidUTF8Error { path } => write!(fmt, "file name in Directory \"{}\" contains invalid UTF-8:", path), 
      Self::CannotOpenDirectoryError { path, message} => write!(fmt, "cannot open directory {:?}: {}", path, message),
      Self::CannotReadFileInDirectoryError { name, message } => write!(fmt, "reading file in directory '{}': {}", name, message),
      Self::CannotAccessFileError { name , message} => write!(fmt, "cannot access '{}': {}", name, message),
      Self::CannotReadSymbolicLinkError { path , message} => write!(fmt, "cannot read symbolic link {:?}: {}", path, message),
      Self::CannotDetermineDevInoError { path, message} => write!(fmt, "cannot determine device and inode of {:?}: {}", path, message),
      Self::DirectoryAlreadyListedError { name} => write!(fmt, "{}: not listing already-listed directory", name),
    }
  }
}