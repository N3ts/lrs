#[derive(Debug)]
pub struct Pending {
  pub name: Option<String>, 
  pub real_name: Option<String>, 
  pub cli_arg: bool, 
  next: Option<Box<Pending>>,
}

impl Pending {
  fn new(name: Option<String>, real_name: Option<String>, cli_arg: bool) -> Self {
    Pending {
      name, 
      real_name, 
      cli_arg,
      next: None,
    }
  }

  /// Expects name to be Some()!
  pub fn get_name(&self) -> &str {
    self.name.as_ref().unwrap()
  }
}

#[derive(Debug)]
pub struct PendingList {
  head: Option<Box<Pending>>,
}

impl PendingList {
  pub fn new() -> PendingList {
    PendingList { 
      head: None 
    }
  }

  pub fn has_next(&self) -> bool {
    self.head.is_some()
  }

  pub fn peek(&self) -> bool {
    match &self.head {
      Some(head) => head.next.is_some(),
      None => false
    }
  }

  pub fn queue_directory(&mut self, name: Option<&str>, real_name: Option<&str>, cli_arg: bool) {
    let mut new_pending = Box::new(
      Pending::new(
        name.map(|s| s.to_owned()),
        real_name.map(|s| s.to_owned()),
        cli_arg
      )
    );

    new_pending.next = self.head.take();
    self.head = Some(new_pending);
  }

  pub fn dequeue_directory(&mut self) -> Option<Box<Pending>> {
    self.head.take().map(|mut out| {
        self.head = out.next.take(); 
        out
      }
    ) 
  }
}
