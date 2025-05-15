// Standard Libraries
use std::rc::{Rc, Weak};

// Project Modules
use super::Args; 
use super::ExitStatus;
use super::filesystem::*;
use crate::print::{print_info::*, format_info::*};
use crate::ignore_mode::*;

use super::{Arc, AtomicBool};

#[derive(Debug)]
pub struct WorkingSet {
  /// Arguments Parsed from the CLI
  pub args: Args,
  /// The list of pending directories to be printed
  pub pending_dirs: PendingList,
  /// The files passed by the CLI or contained in the current directory to be printed
  pub cwd_files: Vec<Rc<FileInfo>>,
  /// Sorted references to the files about to be printed 
  pub sorted_files: Vec<Weak<FileInfo>>,
  /// Manager for directory loop detection 
  pub loop_manager: LoopManager,
  /// Data for output setup
  pub print_info: PrintInfo,
  /// Contains Calculations for column-style output.
  /// Is recalculated for each directory, unlike in the original coreutils-version, which uses
  /// a static Array that is assigned more memory when needed. Therefore performance heavy.
  pub column_state: Option<ColumnState>,
  /// Exit Status Value: 0 => Success, 1 => Minor Problem, 2 => Critical Problem
  pub exit_status: ExitStatus, 
  /// Mode set, to ignore certain Files
  ignore_mode: IgnoreMode,
  /// Contains max width of metadata to be printed
  pub format_info: Option<FormatInfo>,
  /// SIGINT flag
  pub abort_flag: Option<Arc<AtomicBool>>
}

impl WorkingSet {
  pub fn new(args: Args, abort_flag: Option<Arc<AtomicBool>>) -> Self {
    let ignore_mode: IgnoreMode = {
      if args.all { IgnoreMode::IgnoreMinimal }
      else if args.almost_all { IgnoreMode::IgnoreDotAndDotDot }
      else { IgnoreMode::IgnoreDefault }
    };
    let format_info: Option<FormatInfo> = {
      if args.long { Some(FormatInfo::new()) }
      else { None }
    };

    let mut cwd_files:Vec<Rc<FileInfo>> = Vec::new();
    cwd_files.reserve(128);

    Self {
      args, 
      pending_dirs: PendingList::new(),
      cwd_files,
      sorted_files: Vec::new(),
      loop_manager: LoopManager::new(),
      print_info: PrintInfo::new(),
      column_state: None,
      exit_status: ExitStatus::default(),
      ignore_mode,
      format_info,
      abort_flag
    }
  }

  pub fn process_argument_paths(&mut self) {
    let cli_arg: bool = true;
    let paths: Vec<String> = self.args.paths.clone();

    if paths.len() == 0 {
      self.pending_dirs.queue_directory(Some("."), None, cli_arg);
    } 
    for i in 0..paths.len() {
      gobble_file(self, &paths[i], FileType::Unknown, 0, cli_arg, None);
    }
  }

  pub fn clear_files(&mut self) {
    // Not necessarry, as it will be replaced through cwd_files.sorted_files()
    // self.sorted_files.clear(); 
    self.cwd_files.clear();
    
    if self.args.long { 
      self.format_info = Some(FormatInfo::new());
    } else {
      self.format_info = None;
    }
  }

  pub fn sort_files(&mut self) {
    // Todo: make this dependent of sort arguments!
    self.sorted_files = self.cwd_files.sorted_files(SortType::Name);
  }

  pub fn extract_dirs_from_files(&mut self, dir_name: Option<&str>, command_line_arg: bool) {
    let ignore_dot_and_dot_dot = dir_name.is_some();
    
    // Marker Entry
    if dir_name.is_some() && self.args.recursive {
      self.pending_dirs.queue_directory(None, dir_name.clone(), false);
    }
  
    for file_info in self.sorted_files.iter().rev() {
      let f: Rc<FileInfo> = file_info.upgrade().unwrap();
      
      if f.is_directory() && (!ignore_dot_and_dot_dot || !basename_is_dot_or_dot_dot(&f.name)) {
        /* File names cannot be empty, which is why chars.nth(0).unwrap() is safe to do,
        else there is a problem in the name allocation of the FileInfo object. */
        if dir_name.is_none() || f.name.chars().nth(0).unwrap() == '/' {
          self.pending_dirs.queue_directory(Some(&f.name), f.link_name.as_deref(), command_line_arg);
        } else {
          let name: String = file_name_concat(dir_name.unwrap(), &f.name); 
          self.pending_dirs.queue_directory(Some(&name), f.link_name.as_deref(), command_line_arg);
        }
      }
    }

    self.sorted_files.retain(|f| f.upgrade().unwrap().file_type != FileType::ArgDirectory);
  }

  pub fn file_ignored(&self, file_name: &str) -> bool {
    self.ignore_mode != IgnoreMode::IgnoreMinimal 
    && file_name.starts_with('.')
    && (self.ignore_mode == IgnoreMode::IgnoreDefault || dot_or_dot_dot(file_name))
  }

  pub fn calculate_columns(&mut self, by_columns: bool) -> usize {
    let print_info: &PrintInfo = &self.print_info;
    
    let file_count: usize  = self.sorted_files.len();
    let max_idx = print_info.max_idx;
    // max_idx is of type usize: therefore always >= 0
    let max_cols: usize = if max_idx < file_count { max_idx } else { file_count }; 

    let mut column_state: ColumnState = ColumnState::init_column_info(max_cols);
    let columns: &mut Vec<ColumnInfo> = &mut column_state.columns;
    
    for (file_i, weak_file) in self.sorted_files.iter().enumerate() {
      let file = weak_file.upgrade().unwrap(); 
      let name_length = file.width;
      
      for col in 0..max_cols {
        if columns[col].valid_len {
          let rows: usize = (file_count + col) / (col + 1);
          let idx: usize = if by_columns {
            file_i / rows
          } else {
              file_i % (col + 1)
          };

          let real_length: usize = if idx == col { name_length } else { name_length + 2};
          
          if columns[col].col_arr[idx] < real_length{
            columns[col].line_len += real_length - columns[col].col_arr[idx];
            columns[col].col_arr[idx] = real_length; 
            columns[col].valid_len = columns[col].line_len < print_info.line_length;
          }
        }
      }
    }
    
    let mut cols = max_cols;
    while cols > 1 {
      cols -= 1;
      if columns[cols].valid_len {
        break
      }
    }
    
    self.column_state = Some(column_state);
    return cols;
  }
}
