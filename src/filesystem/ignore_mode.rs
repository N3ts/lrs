#[derive(Default, PartialEq, Debug)]
pub enum IgnoreMode {
  /// Ignore files whose names start with '.'
  #[default]
  IgnoreDefault = 0,

  /// Ignore '.', '..'
  IgnoreDotAndDotDot,

  /// Ignore no Files
  IgnoreMinimal
}