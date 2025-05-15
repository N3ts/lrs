use unicode_width::UnicodeWidthStr;

pub fn digit_width(n: u64) -> usize {
  /*let mut length = 1;
  
  while n >= 10 {
      n /= 10;
      length += 1;
  }

  length*/
  let out: String = format!("{}", n);
  UnicodeWidthStr::width(out.as_str())
}