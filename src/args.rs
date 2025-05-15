// Cargo Modules
use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
pub struct Args {
    /// Path of File, Directory or Symlink
    #[arg(value_name = "FILE(s)")]
    pub paths: Vec<String>,
    
    /// Currently equivalenal to -A, due to using std::fs instead of libc. Otherwise:
    /// Do not ignore files starting with "."
    #[arg(short='a', long="all", action = ArgAction::SetTrue)]
    pub all: bool,

    // Use column listing format
    //#[arg(short='x', action = ArgAction::SetTrue)]
    //pub horizontal: bool,

    /// Use long listing format
    #[arg(short='l', action = ArgAction::SetTrue)]
    pub long: bool,
    
    // Use row listing format
    //#[arg(short='x', action = ArgAction::SetTrue)]
    //pub horizontal: bool,

    /// Do not list implied . and ..
    #[arg(short='A', long="almost-all", action = ArgAction::SetTrue)]
    pub almost_all: bool,

    /// When showing file information for symbolic links, show information for the 
    /// file referenced instead
    #[arg(short='L', long="dereference", action = ArgAction::SetTrue)]
    pub dereference: bool,

    /// List subdirectories recursively
    #[arg(short='R', long="recursive", action = ArgAction::SetTrue)]
    pub recursive: bool,
}