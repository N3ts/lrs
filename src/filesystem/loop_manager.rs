// Standard Libraries 
use std::collections::HashSet;

// Project Modules
use super::DevIno
;

const TABLE_INIT_SIZE: usize = 32;

#[derive(Debug)]
pub struct LoopManager {
  /// Hash set containing visited Directories during recursion
  active_dir_set: HashSet<DevIno>,
  /// Open Directories Stack
  dev_ino_stack: Vec<DevIno>,  
}

impl LoopManager {
  pub fn new() -> Self {
    LoopManager { 
      active_dir_set: HashSet::new(), 
      dev_ino_stack: Vec::new()
    }
  }

  pub fn init_loop_detection(&mut self) {
    self.active_dir_set.reserve(TABLE_INIT_SIZE);
    self.dev_ino_stack.reserve(TABLE_INIT_SIZE);
  }

  /// Returns false if directory has been visited before, else true
  pub fn visit_dir(&mut self, dev: u64, ino: u64) -> bool {
    let ent: DevIno = DevIno::new(dev, ino);
    //let ent_from_table: DevIno = hash_insert(ent);
    self.active_dir_set.insert(ent)
  }

  pub fn dev_ino_push(&mut self, dev: u64, ino: u64) {
    let dev_ino: DevIno = DevIno::new(dev, ino);
    
    self.dev_ino_stack.push(dev_ino);
  } 

  pub fn dev_ino_pop(&mut self) {
    // Unwrap is safe to do: Every pop has a corresponding push
    let dev_ino: DevIno = self.dev_ino_stack.pop().unwrap();
    
    self.active_dir_set.remove(&dev_ino);
  } 
}
