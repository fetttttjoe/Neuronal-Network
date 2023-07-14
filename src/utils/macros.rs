pub mod macros {
  #[macro_export]
  macro_rules! safe_get {
    ($mat:expr, $i:expr, $j:expr) => {
      match $mat.get($i, $j) {
        Some(value) => value,
        None => {
          // Handle the error case (get() returned an error)
          println!("Error: Failed to Get Value for Index ({}, {}).", $i, $j);
          continue; // We mainly use that in loops, so we continue to the next iteration
        }
      }
    };
  }
  #[macro_export]
  macro_rules! range {
    ($start:expr, $end:expr) => {
      ($start..$end)
    };
  }
  #[macro_export]
  macro_rules! to_usize {
    ($value:expr) => {
      $value as usize
    };
  }
}
