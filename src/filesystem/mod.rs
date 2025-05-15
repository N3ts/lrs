mod error;
mod dev_ino;
mod fileinfo;
mod loop_manager;
mod pending;
pub mod ignore_mode;

// Standard Libraries
use std::ffi::OsString;
use std::fs::{metadata, read_dir, read_link}; 
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::DirEntryExt;
pub use std::rc::Rc;
pub use std::fs::{Metadata, ReadDir, DirEntry};
pub use std::path::PathBuf;

// Project Modules 
use super::{ExitStatus, IntoExitStatus};
use super::math::*;
use super::working_set::*;
use super::print::format_info::*;
use self::dev_ino::*;
pub use self::error::*;
pub use self::pending::*;
pub use self::fileinfo::*;
pub use self::loop_manager::*;

pub fn open_dir(path: &PathBuf) -> Result<ReadDir> {
  read_dir(path).map_err(
    |e| Error::CannotOpenDirectoryError {  
      path: path.clone(),
      message: e.to_string() 
    }
  )
}

/*pub fn detect_loop() {

}*/

pub fn file_name_to_string(pending: &Pending, file_name: OsString) -> Result<String> {
  match file_name.into_string() {
    Ok(name) => Ok(name), 
    Err(_) => Err( Error::FileInvalidUTF8Error { 
      path: pending.name.as_ref().unwrap().clone() 
    })
  }
}

pub fn pathname_to_string(path: &PathBuf) -> Result<String> {
  match path.to_str() {
    Some(s) => Ok(s.to_string()),
    None => Err(
      Error::PathInvalidUTF8Error { 
        path: path.clone() 
      }
    )
  }
}

/// Resolves a relative path. Assumes directory is Some
fn resolve_path(rel_path: PathBuf, directory: Option<&PathBuf>) -> PathBuf {
  if directory.is_some() {
    directory.unwrap().join(rel_path)
  } else {
    rel_path
  }
}

fn last_component(path: &str) -> &str {
  path
    .rsplit('/')
    .next()
    .unwrap_or("")
}

pub fn dot_or_dot_dot(name: &str) -> bool {
  name == "." || name == ".."
}

pub fn basename_is_dot_or_dot_dot(dir_name: &str) -> bool {
  dot_or_dot_dot(last_component(dir_name))
}

pub fn file_name_concat(dirname: &str, filename: &str) -> String {
  if dirname.ends_with('/') {
      format!("{}{}", dirname, filename)
  } else {
      format!("{}/{}", dirname, filename)
  }
}

pub fn gobble_file (working_set: &mut WorkingSet, name: &str, mut file_type: FileType, 
                    inode: u64, cli_arg: bool, dir_name: Option<&PathBuf>) -> u64 {
  let blocks: u64; 
  let original_path = PathBuf::from(name);
  let mut resolved_path: PathBuf;
  let mut full_name: String = name.to_string();
  let mut link_name: Option<String> = None;
  let metadata: Metadata; 
  let mut link_metadata: Option<Metadata> = None;

  debug_assert!(! cli_arg || inode == 0);

  // Technically stat always needs to be checked, due to color properties always being enabled
  let check_stat = cli_arg 
    || working_set.args.long
    || matches!(file_type, FileType::Unknown | FileType::Directory | FileType::Normal)   
    || (matches!(file_type, FileType::SymbolicLink | FileType::Unknown) && working_set.args.dereference); 

  if check_stat || original_path.is_relative() && dir_name.is_some() {
    resolved_path = resolve_path(original_path.clone(), dir_name);
    match pathname_to_string(&resolved_path) {
      Ok(n) => full_name = n,
      Err(e) => { 
        working_set.exit_status.update(e.into(), cli_arg);
        return 0;
      }
    };
  } else {
    resolved_path = original_path.clone();
  }

  let metadata_result: Result<Metadata>;
  if working_set.args.dereference {
    metadata_result = resolved_path.metadata().map_err(
      |e| Error::CannotAccessFileError {
        name: full_name.clone(), 
        message: e.to_string() 
      }
    );
  } else {
    metadata_result = resolved_path.symlink_metadata().map_err(
      |e| Error::CannotAccessFileError {
        name: full_name.clone(), 
        message: e.to_string()
      }
    );
  }
  match metadata_result { 
    Ok(m) => metadata = m, 
    Err(e) => {
      working_set.exit_status.update(e.into(), cli_arg);
      if cli_arg { 
        return 0
      } else {
        working_set.cwd_files.push(Rc::new(FileInfo::new(
          name.to_string(), 
          None,
          None, 
          None, 
          inode, 
          file_type)));
        return 0;
      }
    }
  }

  file_type = FileType::determine(&metadata, cli_arg);

  if file_type == FileType::SymbolicLink && working_set.args.long {
    match read_link(&full_name) {
      Ok(target_path) => {
        let dir_name = PathBuf::from(dir_name.unwrap_or(&PathBuf::from(".")));
        let abs_target = dir_name.join(&target_path);
        
        match abs_target.metadata() {
          Ok(m) => {
            resolved_path = target_path;
            link_metadata = Some(m);
            link_name = Some(pathname_to_string(&resolved_path).unwrap_or_default());
          }
          Err(e) => {
            resolved_path = target_path;
            link_name = Some(pathname_to_string(&resolved_path).unwrap_or_default());
            working_set.exit_status.update(
            Error::CannotReadSymbolicLinkError {
              path: resolved_path,
              message: e.to_string()
            }.into(),
            cli_arg
          );
          }
        }
      }
      Err(e) => {
        working_set.exit_status.update(
          Error::CannotReadSymbolicLinkError {
            path: original_path.clone(),
            message: e.to_string()
          }.into(),
          cli_arg
        );
      }
    }
  } 

  // Standard Blocksize is 512 bytes
  blocks = metadata.st_blocks();

  if working_set.args.long {
    let info: &mut FormatInfo = working_set.format_info.as_mut().unwrap();
    
    info.update_hard_link_length(digit_width(metadata.st_nlink()));

    info.update_user_length(digit_width(metadata.st_uid() as u64));
    info.update_group_length(digit_width(metadata.st_gid() as u64));
    // Not necessary for pure "-l"-Argument 
    //info.update_block_length(digit_count(blocks));
    
    // Char- and block devices don't have block sizes, but major- and minor dev-numbers instead
    if matches!(file_type, FileType::CharDev | FileType::BlockDev) {
      let st_rdev = metadata.st_rdev();
      let maj_len: usize = digit_width(major(st_rdev));
      let min_len: usize = digit_width(minor(st_rdev));

      // Major and minor device numbers are separated by ", "
      info.update_file_size_lengths(maj_len, min_len);
    }
    else {
      info.update_file_size_length(digit_width(metadata.st_size()));
    }
  }

  // inode won't be printed in pure -l or -C
  let file_info = FileInfo::new(
    name.to_string(), 
    link_name, 
    Some(metadata), 
    link_metadata,
    inode, 
    file_type
  );
  working_set.cwd_files.push(Rc::new(file_info));

  blocks
}

fn major(dev: u64) -> u64 {
  (dev >> 8) & 0xfff
}

fn minor(dev: u64) -> u64 {
  (dev & 0xff) | ((dev >> 12) & 0xffffff00)
}

pub fn get_major_and_minor(dev: u64) -> (u64, u64) {
  (major(dev), minor(dev))
}

pub fn print_dir(working_set: &mut WorkingSet, this_pend: &Box<Pending>, print_dir_name: bool, first: bool) { 
  let dir: ReadDir;
  let mut dir_entry: DirEntry;
  let mut total_blocks: u64 = 0;
  
  let path: PathBuf = PathBuf::from(this_pend.get_name()); 
  match open_dir(&path) {
    Ok(d) => dir = d,
    Err(e) => { 
      working_set.exit_status.update(e.into(), this_pend.cli_arg); 
      return;
    }
  }

  if working_set.args.recursive {
    // fstat on open directories is not safe, therefore perform safe stat through metadata()
    let metadata: Metadata = match metadata(&path) {
      Ok(m) => m,
      Err(e) => {
        working_set.exit_status.update(
          Error::CannotDetermineDevInoError { path, message: e.to_string() }.into(), 
          this_pend.cli_arg
        );
        return;
      },
    }; 
    let (dev, ino): (u64, u64) = (metadata.st_dev(), metadata.st_ino());

    // If the directory has been visited before, the entry will be skipped
    if !working_set.loop_manager.visit_dir(dev, ino) {
      working_set.exit_status.update(
        Error::DirectoryAlreadyListedError { name: this_pend.name.clone().unwrap() }.into(), 
        this_pend.cli_arg
      );
      return;
    }
    
    working_set.loop_manager.dev_ino_push(dev, ino)
  } 

  working_set.clear_files();

  // directory name output 
  if working_set.args.recursive || print_dir_name {
    if !first { println!(); }

    match &this_pend.real_name {
      Some(real_name) => print!("{}", real_name),
      None => print!("{}", this_pend.get_name())
    }; 
    println!(":");
  } 

  // Process directory entries
  for res in dir {
    match res {
      Ok(entry) => dir_entry = entry ,
      Err(e) => { working_set.exit_status.update(
    Error::CannotReadFileInDirectoryError { 
            name: this_pend.name.as_ref().unwrap().into(), 
            message: e.to_string() 
          }.into(), 
          this_pend.cli_arg
        ); 
        continue;
      }
    }
    
    let file_name = match file_name_to_string(&this_pend, dir_entry.file_name()) {
      Ok(name) => name, 
      Err(e) => { 
        working_set.exit_status.update(e.into(), this_pend.cli_arg); 
        continue;
      }
    }; 

    if ! working_set.file_ignored(&file_name) {
      let file_type: FileType = match dir_entry.file_type() {
        Ok(file_type) => FileType::new(file_type, false),
        Err(_) => FileType::Unknown
      };
      
      total_blocks += gobble_file(working_set, &file_name, file_type, dir_entry.ino(), false, Some(&path));
    } 
  }
  // Directory closes automatically 

  working_set.sort_files();

  if working_set.args.recursive {
      working_set.extract_dirs_from_files(this_pend.name.as_deref(), false);
  }
  
  if working_set.args.long {
    println!("total {}", total_blocks);
  }
  
  if working_set.sorted_files.len() > 0 {
    super::print::print_current_files(working_set);
  }
}