// Cargo Modules
use clap::Parser;

// Standard Libraries
use std::process::ExitCode;

// Project Modules
mod exit_status;
mod args;
mod working_set;
mod error;
mod filesystem;
mod print;
mod math;

use args::Args;
use exit_status::*;
use filesystem::*;
use working_set::*;
use print::print_current_files;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use ctrlc;

fn main() -> ExitCode {
  let abort_flag = Arc::new(AtomicBool::new(false));
  let flag_clone = abort_flag.clone();

  // Signal Handling
  ctrlc::set_handler(move || {
      flag_clone.store(true, Ordering::SeqCst);
  }).expect("Failed setting up SIGINT handler");
  
  let mut working_set: WorkingSet = WorkingSet::new(Args::parse(), Some(abort_flag));

  if working_set.args.recursive { 
    working_set.loop_manager.init_loop_detection(); 
  }
  
  // Output arguments 
  working_set.process_argument_paths();
  working_set.sort_files();
  working_set.extract_dirs_from_files(None, true);
  
  let mut print_dir_name: bool = true;
  let n_files: usize = working_set.sorted_files.len(); 
  if n_files > 0 {
    print_current_files(&mut working_set);
    
    if working_set.pending_dirs.has_next() {
      println!()
    }
  } else if n_files <= 1 && working_set.pending_dirs.has_next() && !working_set.pending_dirs.peek() {
    print_dir_name = false;
  }

  // Output directories 
  let mut first: bool = true;

  while working_set.pending_dirs.has_next(){
    // Signals
    if let Some(abort_flag) = &working_set.abort_flag {
      if abort_flag.load(Ordering::SeqCst) {
          return working_set.exit_status.into();
      }
    }

    // dequeue_directory() can't be none, due to has_next() == true within this scope
    let this_pend = working_set.pending_dirs.dequeue_directory().unwrap();
    
    if working_set.args.recursive {
      if this_pend.name == None {
        working_set.loop_manager.dev_ino_pop();
        continue;
      }
    }
    
    // this_pend.name is some at this point!
    print_dir(&mut working_set, &this_pend, print_dir_name, first);
    print_dir_name = true;
    first = false;
  }

  // Cleanup
  working_set.exit_status.into()
}