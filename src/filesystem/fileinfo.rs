// Standard Libraries
use std::borrow::Cow;
use std::fs::Metadata;
use std::os::unix::fs::FileTypeExt;
use std::rc::{Rc, Weak};

// Cargo Modules 
use unicode_width::UnicodeWidthStr;

#[derive(Default, PartialEq, Debug)]
pub enum FileType {
  #[default]
  Unknown,
  FIFO,
  CharDev,
  Directory,
  BlockDev,
  Normal,
  SymbolicLink,
  Sock,
  ArgDirectory
}

impl FileType {
  pub fn new(file_type: std::fs::FileType, cli_arg: bool) -> Self {
    if file_type.is_file() { FileType::Normal }   
    else if file_type.is_dir() { 
      if cli_arg { FileType::ArgDirectory } 
      else { FileType::Directory }
    }
    else if file_type.is_symlink() { FileType::SymbolicLink }
    else if file_type.is_block_device() { FileType::BlockDev }
    else if file_type.is_char_device() { FileType::CharDev }
    else if file_type.is_fifo() { FileType::FIFO }
    else if file_type.is_socket() { FileType::Sock }
    else { FileType::default() }
  }

  pub fn determine(metadata: &Metadata, cli_arg: bool) -> Self {
    let file_type: std::fs::FileType = metadata.file_type();
    
    Self::new(file_type, cli_arg)
  }
}

#[derive(Debug)]
pub struct FileInfo {
  /// File Name
  pub name: String,
  /// Name of the File linked to, if symbolic link
  pub link_name: Option<String>,
  /// Contains Access to: FileType, Permissions, stat-Objects 
  pub metadata: Option<Metadata>,
  /// Inode of File - 0 when command line argument!
  #[allow(dead_code)]
  pub inode: u64,
  /// Cached FileType, differentiates between Directories and Directories passed as an argument
  pub file_type: FileType,
  /// Metadata of link target, for coloring
  #[allow(dead_code)]
  pub link_metadata: Option<Metadata>,
  /// Cached screen width (quotes included)
  pub width: usize,
  /// Whether the file name has to be quoted on output
  quoting: bool
}

impl FileInfo {
  pub fn new(name: String, link_name: Option<String>, metadata: Option<Metadata>, 
    link_metadata: Option<Metadata>, inode: u64, file_type: FileType) -> Self {
    let quoting = name.chars().any(|c| c.is_whitespace());
    let width = UnicodeWidthStr::width(name.as_str()) 
      + if quoting { UnicodeWidthStr::width("''") } else { 0 };

      FileInfo{
        name,
        link_name,
        metadata,
        inode,
        link_metadata,
        file_type,
        width,
        quoting
      }
  }

  pub fn is_directory(&self) -> bool{
    match &self.file_type {
      FileType::Directory | FileType::ArgDirectory => true,
      _ => false
    }
  }

  pub fn quoted_name(&self) -> String {
    if self.quoting {
      format!("'{}'", &self.name)
    } else {
      self.name.clone()
    }
  }
}

pub enum SortType {
  Name = 0,
  //Size,
  //Time,
  //None
}

trait SortFileInfoCollection {
  fn sort(&mut self, sort_type: SortType);
}

impl SortFileInfoCollection for Vec<Weak<FileInfo>> {
  fn sort(&mut self, sort_type: SortType) {
    match sort_type {
      SortType::Name => self.sort_by_key(|f| Cow::from(f.upgrade().unwrap().name.to_lowercase())),
      //SortType::Size => ,
      //SortType::Time => ,
      //SortType::None => 
    }
  }
}

pub trait SortedFileInfoCollection {
    fn sorted_files(&self, sort_type: SortType) -> Vec<Weak<FileInfo>>;
}

impl SortedFileInfoCollection for Vec<Rc<FileInfo>> {
  fn sorted_files(&self, sort_type: SortType) -> Vec<Weak<FileInfo>> {
    let mut out: Vec<Weak<FileInfo>> = Vec::with_capacity(self.len());
    
    self.iter().for_each(|f| out.push(Rc::downgrade(f)));
    out.sort(sort_type);

    out
  }
}