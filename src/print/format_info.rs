#[derive(Debug)]
pub struct FormatInfo {
  pub hard_link_length: usize, 
  pub user_length: usize,
  pub group_length: usize,  
  //pub block_length: usize,
  pub major_length: usize,
  pub minor_length: usize,
  pub file_size_length: usize
}

impl FormatInfo {
  pub fn new() -> Self {
    Self { 
      hard_link_length: 0,
      user_length: 0, 
      group_length: 0, 
      //block_length: 0,
      major_length: 0,
      minor_length: 0,
      file_size_length: 0 
    }
  }

  pub fn update_hard_link_length(&mut self, hard_link_length: usize) {
    if self.hard_link_length < hard_link_length {
      self.hard_link_length = hard_link_length;
    }
  }

    pub fn update_user_length(&mut self, user_length: usize) {
      if self.user_length < user_length {
        self.user_length = user_length;
      }
    }
  
    pub fn update_group_length(&mut self, group_length: usize) {
      if self.group_length < group_length {
        self.group_length = group_length;
      }
    }

  /*pub fn update_block_length(&mut self, block_length: usize) {
    if self.block_length < block_length {
      self.block_length = block_length;
    }
  }*/
  
  fn update_major_length(&mut self, major_length: usize) {
    if self.major_length < major_length {
      self.major_length = major_length;
    }
  }

  fn update_minor_length(&mut self, minor_length: usize) {
    if self.minor_length < minor_length {
      self.minor_length = minor_length;
    }
  }

  pub fn update_file_size_length(&mut self, file_size_length: usize) {
    if self.file_size_length < file_size_length {
      self.file_size_length = file_size_length;
    }
  }

  pub fn update_file_size_lengths(&mut self, major_length: usize, minor_length: usize) {
    self.update_major_length(major_length);
    self.update_minor_length(minor_length);
    self.update_file_size_length(major_length + minor_length + 2);
  }
}