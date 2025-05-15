// Cargo Modules
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
  #[from]
  FS(super::filesystem::Error),
  #[from]
  IO(std::io::Error)
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Self::FS(inner) => write!(fmt, "{}", inner),
      Self::IO(inner) => write!(fmt, "{}", inner)
    }
  }
}
