// Cargo Modules
use terminal_size::{terminal_size, Width};

const MIN_COLUMN_WIDTH: usize = 3;

#[derive(Debug)]
pub struct PrintInfo {
  pub max_idx: usize,
  pub line_length: usize,
}

impl PrintInfo {
  pub fn new() -> Self {
    let line_length: usize = match Self::terminal_width() {
      Some(len) => len,
      None => 80
    };
    let max_idx: usize = line_length / MIN_COLUMN_WIDTH;
    
    Self {
      max_idx,
      line_length
    }
  }

  fn terminal_width() -> Option<usize> {
    if let Some((Width(w), _)) = terminal_size() {
      Some(w as usize)
  } else {
      None
  }
  }
}

#[derive(Debug)]
pub struct ColumnInfo {
  pub valid_len: bool, 
  pub line_len: usize,
  pub col_arr: Vec<usize>
}

#[derive(Debug)]
pub struct ColumnState {
  pub columns: Vec<ColumnInfo>
}

impl ColumnState {
  /// Does not implement dynamic allocation for changing terminal size!
  pub fn init_column_info(max_cols: usize) -> Self {
    let mut columns: Vec<ColumnInfo> = Vec::with_capacity(max_cols);

    for i in 0..max_cols {
      // i + 1 Spalten
      let col_count: usize = i + 1;
      let col_arr: Vec<usize> = vec![MIN_COLUMN_WIDTH; col_count];
      let line_len: usize = col_count * MIN_COLUMN_WIDTH;

      columns.push(ColumnInfo {
        col_arr,
        line_len,
        valid_len: true,
      });
    }

    ColumnState { columns }
  }
}
