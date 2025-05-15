// Standard Libraries
use super::ExitCode;
use super::error::Error;

#[derive(Copy, Clone, Default, Debug)]
pub enum ExitStatus {
  #[default]
  ExitSuccess = 0,
  LrsMinorProblem = 1,
  LrsFailure = 2,
}

impl ExitStatus {
  pub fn update(&mut self, error: Error, cli_arg: bool) {
    eprintln!("lrs: {}", &error);
    let new_status: ExitStatus = error.into_exit_status(cli_arg);

    if (*self as u8) < (new_status as u8) {
        *self = new_status;
    } 
  }
}

impl From<&ExitStatus> for u8 {
  fn from(exit_status: &ExitStatus) -> Self {
    *exit_status as u8
  }
}

impl From<ExitStatus> for ExitCode {
  fn from(exit_status: ExitStatus) -> Self {
    ExitCode::from(exit_status as u8)
  }
}

impl From<bool> for ExitStatus {
  fn from(serious: bool) -> Self {
    if serious { ExitStatus::LrsMinorProblem } 
    else { ExitStatus::LrsFailure }
  } 
}

pub trait IntoExitStatus {
  fn into_exit_status(&self, cli_arg: bool) -> ExitStatus;
}

impl IntoExitStatus for Error {
  fn into_exit_status(&self, cli_arg: bool) -> ExitStatus {
    ExitStatus::from(cli_arg)
  }
}