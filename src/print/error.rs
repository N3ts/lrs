// Standard Libraries
//use std::path::PathBuf;

//use super::ExitStatus;

//pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  
}

/*impl From<Error> for ExitStatus{
  fn from(err: Error) -> Self {
    //match err {
    //  Error:: { serious,.. } |
    //  Error:: { serious,.. } | => {
    //    serious.into()
    //  },
    //}
  }
}*/

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    match self {
      _ => write!(fmt, "Print error"),
    }
  }
}
