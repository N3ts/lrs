use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct DevIno {
  st_dev: u64,
  st_ino: u64
}

impl DevIno {
  pub fn new(st_dev: u64, st_ino: u64) -> Self {
    DevIno { 
      st_dev, 
      st_ino 
    }
  }
}

impl Eq for DevIno { }

impl PartialEq for DevIno {
  fn eq(&self, other: &Self) -> bool {
    self.st_dev == other.st_dev && self.st_ino == other.st_ino
  }
}

impl Hash for DevIno {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u64(self.st_dev);
    state.write_u64(self.st_ino);
  }
}
