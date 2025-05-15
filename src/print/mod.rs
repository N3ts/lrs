pub mod print_info;
pub mod format_info;

// Standard Libraries
use std::fs::{FileType, Metadata};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;

// Cargo Modules
use chrono::{Local, TimeZone, Utc};

// Project Modules
use crate::filesystem::get_major_and_minor;

use super::{Rc, FileInfo, WorkingSet};
use self::format_info::*;
use self::print_info::*;

const GREEN: &str = "\x1b[1;32m";
const BLUE:  &str = "\x1b[1;34m";
const CYAN:  &str = "\x1b[1;36m";
const RESET: &str = "\x1b[0m";


// Executable Permissions for owner, group and other
//const S_IXUSR: u32 = 0o100;
//const S_IXGRP: u32 = 0o010;
//const S_IXOTH: u32 = 0o001;
// S_IXUGO = (S_IXUSR | S_IXGRP | S_IXOTH)
const S_IXUGO: u32 = 0o111;  

fn indent(mut from: usize, to: usize) {
  while from < to {
    print!(" ");
    from += 1;
  }
}

pub fn print_current_files(working_set: &mut WorkingSet) {
  if working_set.args.long {
    print_long_format(working_set);
  } else {
    print_many_per_line(working_set);
  }
}

// Any panic in this function implies an error in regards to sorted_files or column_state.
// In that case, its a flaw in the logical execution beforehand and not in this function!
fn print_many_per_line(working_set: &mut WorkingSet) {
  let file_count: usize = working_set.sorted_files.len();
  let columns: usize = working_set.calculate_columns(true);
  let rows: usize = file_count / columns + ((file_count % columns != 0) as usize);
  
  // Panics if column_state is None, which should never happen, as it is only initialized, 
  // when working_set.sorted_files are supposed to be printed  
  let line_fmt: &ColumnInfo = &working_set.column_state.as_ref().unwrap().columns[columns-1];

  for row in 0..rows {
    let mut column: usize = 0; 
    let mut filesno: usize = row; 
    let mut pos: usize = 0; 

    loop {
      // If upgrade() or unwrap() fail, then because the referenced object fell out of scope,
      // which shouldn't have happened in the first place at this point
      let f: &Rc<FileInfo> = &working_set.sorted_files[filesno].upgrade().unwrap();
      let name_length: usize = f.width;
      let max_name_length: usize = line_fmt.col_arr[column];
      column += 1;

      print!("{}", quoted_name_to_string(f.quoted_name(), f.metadata.as_ref(), &f.file_type));

      if file_count - rows <= filesno {
        break;
      }
      filesno += rows;

      indent(pos + name_length, pos + max_name_length);
      pos += max_name_length
    }
    println!();
  }
}

fn print_long_format(working_set: &WorkingSet) {
  for entry in working_set.sorted_files.iter() {
    let mut output = String::new();
    
    // FileInfo should have to be unwrapable!
    let f: Rc<FileInfo> = entry.upgrade().unwrap();
    // So should format_info, due to -l set
    let format_info: &FormatInfo = working_set.format_info.as_ref().unwrap();
    // Metadata can also be None due to broken symlinks
    let metadata: Option<&Metadata> = f.metadata.as_ref();
    let file_type: &super::FileType = &f.file_type; 

    // Type and Permissions
    let mode = permissions_to_string(metadata, file_type);
    output.push_str(&mode);
    output.push(' ');

    // Hardlinks
    let nlink = n_link_to_string(metadata, format_info);
    output.push_str(&nlink);
    output.push(' ');

    // Owner
    let owner = owner_to_string(metadata, format_info);
    output.push_str(&owner);
    output.push(' ');

    // Group
    let group = group_to_string(metadata, format_info);
    output.push_str(&group);
    output.push(' ');

    // Size or dev/inode
    let size = file_size_to_string(metadata, format_info);
    output.push_str(&size);
    output.push(' ');

    // Timestamp
    let timestamp = time_stamp_to_string(metadata);
    output.push_str(&timestamp);
    output.push(' ');

    // File name
    output.push_str(&quoted_name_to_string(f.quoted_name(), metadata, file_type));
    output.push(' ');

    // Target if symbolic link
    if f.file_type == super::FileType::SymbolicLink {
      output.push_str(&symlink_path_to_string(&f));
    }

    println!("{}", output);
  }
}

/// Writes file types into a string (Unix only)
fn file_type_to_string(file_type: &super::FileType) -> char {    
  match file_type {
    &super::FileType::Normal => '-',
    &super::FileType::Directory |
    &super::FileType::ArgDirectory  => 'd',
    &super::FileType::SymbolicLink => 'l',
    &super::FileType::BlockDev => 'b',
    &super::FileType::CharDev => 'c',
    &super::FileType::FIFO => 'p',
    &super::FileType::Sock => 's',
    _ => '?'
  }
}

/// Writes permissions into a string
fn permissions_to_string(metadata:  Option<&Metadata>, file_type: &super::FileType) -> String {
  let mut out = String::from("");
  // perm_masks = rwx
  let perm_masks: [u32; 3] = [0o400, 0o200, 0o100];
  
  out.push(file_type_to_string(file_type));
  match metadata{
    Some(metadata) => {
      let mut mode = metadata.st_mode();
      for _ in 0..3 {
        // i = 0: user, i = 1: group, i = 2: other
        out.push(if mode & perm_masks[0] != 0 {'r'} else {'-'}); 
        out.push(if mode & perm_masks[1] != 0 {'w'} else {'-'}); 
        out.push(if mode & perm_masks[2] != 0 {'x'} else {'-'});
        mode <<= 3; 
      }
    },
    None => out.push_str("?????????"),
  }

  out
}

fn n_link_to_string(metadata: Option<&Metadata>, format_info: &FormatInfo) -> String {
  let out: String = match metadata {
    Some(metadata) => metadata.st_nlink().to_string(),
    None => String::from("?")
  };

  format!(
    "{:>1$}", 
    out, 
    format_info.hard_link_length
  )
}

fn owner_to_string(metadata:  Option<&Metadata>, format_info: &FormatInfo) -> String {
  let out: String = match metadata {
    Some(metadata) => metadata.st_uid().to_string(),
    None => String::from("?")
  };

  format!(
    "{:>1$}", 
    out, 
    format_info.user_length
  )
}

fn group_to_string(metadata:  Option<&Metadata>, format_info: &FormatInfo) -> String {
  let out: String = match metadata {
    Some(metadata) => metadata.st_gid().to_string(),
    None => String::from("?")
  };

  format!(
    "{:>1$}", 
    out, 
    format_info.group_length
  )
}

/// Writes file size into a string
fn file_size_to_string(metadata: Option<&Metadata>, format_info: &FormatInfo) -> String {
  match metadata {
    Some(metadata) => {
      let file_type: FileType = metadata.file_type();
      
      if file_type.is_block_device() || file_type.is_char_device() { 
        let (major, minor): (u64, u64) = get_major_and_minor(metadata.st_dev());
        
        format!("{:>major_length$}, {:>minor_length$}", major, minor, 
          major_length = format_info.major_length, minor_length = format_info.minor_length)
      } else {
        format!("{:>1$}", metadata.st_size(), format_info.file_size_length)
      }
    },
    None =>  format!("{:>1$}", "?", format_info.file_size_length)
  }
}

/// Writes time stamp into a string
fn time_stamp_to_string(metadata: Option<&Metadata>) -> String {
  const EMPTY: &str = "                  ?";

  match metadata {
    Some(metadata) => {
      let mtime: i64 = metadata.st_mtime();

      // Calculate utc, unwrap it, convert to local time and format it. Else return "?"
      Utc.timestamp_opt(mtime, 0)
      .single()
      .map(|dt_utc| dt_utc.with_timezone(&Local))
      .map(|dt_local| dt_local.format("%Y-%m-%d %H:%M:%S").to_string())
      .unwrap_or_else(|| EMPTY.into())
    },
    None => EMPTY.into()
  }
}

fn quoted_name_to_string(name: String, metadata:  Option<&Metadata>, file_type: &super::FileType) -> String {
    match file_type {
    super::FileType::Directory | 
    super::FileType::ArgDirectory => format!("{}{}{}", BLUE, name, RESET),
    super::FileType::SymbolicLink => format!("{}{}{}", CYAN, name, RESET),
    _ => if is_executable(metadata) { format!("{}{}{}", GREEN, name, RESET) } else { name }
  }
}

fn is_executable(metadata: Option<&Metadata>) -> bool{
  match metadata {
    //Some(metadata) => metadata.st_mode() & (S_IXUSR | S_IXGRP | S_IXOTH) != 0
    Some(metadata) => metadata.st_mode() & S_IXUGO != 0,
    None => false
  }
}

/// Writes path of symlink target into a string
fn symlink_path_to_string(file_info: &FileInfo) -> String {
  match file_info.link_name.as_ref() {
    Some(target) => format!("-> {}", target),
    None => "".into()
  }
}
