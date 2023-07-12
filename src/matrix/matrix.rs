#[path = "../util/functions.rs"]
mod functions;
use num_traits::{CheckedMul, FromPrimitive, NumCast, PrimInt, Zero};
use rand::{thread_rng, Rng};
use std::cmp::PartialOrd;
use std::default::Default;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
macro_rules! ternary {
  ($condition:expr, $true_expr:expr, $false_expr:expr) => {
    if $condition {
      $true_expr
    } else {
      $false_expr
    }
  };
}

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
macro_rules! to_usize {
  ($value:expr) => {
    match $value.to_usize() {
      Some(result) => result,
      _ => panic!("Conversion to usize failed"),
    }
  };
}

macro_rules! range {
  ($start:expr, $end:expr) => {
    ($start..$end)
  };
}

pub struct Mat<T> {
  pub rows: T,
  pub cols: T,
  pub data_stream: *mut f64,
}
impl<T> Default for Mat<T>
where
  T: Zero,
{
  fn default() -> Self {
    Mat {
      rows: T::zero(),
      cols: T::zero(),
      data_stream: std::ptr::null_mut(),
    }
  }
}
impl<T> Mat<T>
where
  T: CheckedMul + PrimInt + NumCast + Zero + PartialOrd + Mul<Output = T>,
{
  pub fn new(rows: T, cols: T) -> Mat<T> {
    assert!(rows > T::zero(), "Number of rows must be greater than 0.");
    assert!(
      cols > T::zero(),
      "Number of columns must be greater than 0."
    );
    let num_elements = match rows.checked_mul(&cols) {
      Some(result) => result,
      None => panic!("Multiplication overflow"),
    };

    let num_elements_usize = to_usize!(num_elements);
    let data_stream = Box::into_raw(vec![0u64; num_elements_usize].into_boxed_slice()) as *mut f64;

    return Mat {
      rows,
      cols,
      data_stream,
    };
  }

  pub fn sigmoid(&mut self) {
    for i in range!(0, to_usize!(self.rows)) {
      for j in range!(0,to_usize!(self.cols)) {
        let value = safe_get!(self, i, j);
        let value_f64: f64 = NumCast::from(value).expect("Conversion to f64 failed");
        let sigmoid = functions::sigmoid(value_f64);
        self.set(i, j, sigmoid);
      }
    }
  }

  pub fn print(&self, overwrite_padding: Option<usize>, overwrite_precision: Option<usize>) {
    let padding = overwrite_padding.unwrap_or(4);
    let precision = overwrite_precision.unwrap_or(4);

    let rows_usize = to_usize!(self.rows);
    let cols_usize = to_usize!(self.cols);

    println!(
      "{:padding$}Mat ({} x {}):",
      "",
      rows_usize,
      cols_usize,
      padding = padding
    );
    println!(
      "{:padding$}Memory location: {:?}",
      "",
      self.data_stream,
      padding = padding
    );

    // Calculate the maximum number of digits in column indices
    let max_col_digits = self.cols.to_usize().unwrap().to_string().len();

    unsafe {
      println!("{:padding$}[", "", padding = padding);
      for i in range!(0, rows_usize) {
        print!("{:padding$}", "", padding = padding + padding);
        for j in range!(0, cols_usize) {
          let value = *self.data_stream.add(i * cols_usize + j);
          let value_f64: f64 = NumCast::from(value).expect("Conversion to f64 failed");
          print!(
            "{:<width$.precision$}",
            value_f64,
            width = (max_col_digits + padding + precision),
            precision = precision
          );
        }
        println!();
      }
      println!("{:padding$}]", "", padding = padding);
    }
  }

  pub fn rand(&self, low: f64, high: f64) {
    let rows_usize = to_usize!(self.rows);
    let cols_usize = to_usize!(self.cols);

    let mut rng = thread_rng();
    for i in range!(0, rows_usize) {
      for j in range!(0, cols_usize) {
        let random_value = rng.gen_range(low..=high);
        let index = i * to_usize!(self.cols) + j;
        unsafe {
          *self.data_stream.add(index) = random_value;
        }
      }
    }
  }

  pub fn fill(&mut self, value: T)
  where
    T: NumCast,
    usize: NumCast,
  {
    let num_elements = self.rows * self.cols;
    let num_elements_usize = to_usize!(num_elements);
    let value_u64 = NumCast::from(value).expect("Conversion to u64 failed");

    unsafe {
      let data_stream = std::slice::from_raw_parts_mut(self.data_stream, num_elements_usize);
      for i in range!(0, num_elements_usize) {
        *data_stream.get_unchecked_mut(i) = value_u64;
      }
    }
  }

  pub fn get(&self, row: usize, col: usize) -> Option<f64>
  where
    T: PrimInt + Copy + PartialOrd,
  {
    let rows_usize = to_usize!(self.rows);
    let cols_usize = to_usize!(self.cols);
    assert!(
      row < rows_usize,
      "Get failed! Supplied row Index {} is out of bounds. Matrix has {} rows.",
      row,
      rows_usize
    );
    assert!(
      col < cols_usize,
      "Get failed! Supplied column Index {} is out of bounds. Matrix has {} columns.",
      col,
      cols_usize
    );
    let index = row * cols_usize + col;
    unsafe {
      let value = *self.data_stream.add(index);
      return Some(FromPrimitive::from_f64(value).unwrap());
    }
  }
  pub fn set(&mut self, row: usize, col: usize, value: f64) 
  {
    let rows_usize = to_usize!(self.rows);
    let cols_usize = to_usize!(self.cols);

    assert!(
      row < rows_usize,
      "Set failed! Supplied row Index {} is out of bounds. Matrix has {} rows.",
      row,
      rows_usize
    );
    assert!(
      col < cols_usize,
      "Set failed! Supplied column Index {} is out of bounds. Matrix has {} columns.",
      col,
      cols_usize
    );

    let index = row * cols_usize + col;
    unsafe {
      *self.data_stream.add(index) = value;
    }
  }
  fn drop(&mut self) {
    let num_elements_usize = match (self.rows.to_usize(), self.cols.to_usize()) {
      (Some(rows_usize), Some(cols_usize)) => rows_usize * cols_usize,
      _ => panic!("Conversion to usize failed"),
    };

    unsafe {
      Vec::from_raw_parts(self.data_stream, 0, num_elements_usize);
    }
  }
}
pub fn addition<T>(mat1: &Mat<T>, mat2: &Mat<T>) -> Mat<T>
where
  T: Add<Output = T>
    + PartialEq
    + NumCast
    + Zero
    + CheckedMul
    + PartialOrd
    + Display
    + PrimInt,
{
  assert!(
    mat1.rows == mat2.rows && mat1.cols == mat2.cols,
    "Matrix dimensions must match. Got Mat1: ({}x{}) and Mat2: ({}x{})",
    mat1.rows,
    mat1.cols,
    mat2.rows,
    mat2.cols
  );
  let rows_usize = to_usize!(mat1.rows);
  let cols_usize = to_usize!(mat1.cols);

  let rows = mat1.rows;
  let cols = mat1.cols;
  let mut result = Mat::new(rows, cols);

  for i in 0..rows_usize {
    for j in 0..cols_usize {
      let value1 = safe_get!(mat1, i, j);
      let value2 = safe_get!(mat2, i, j);
      let sum = value1 + value2;
      // This might make problems, because we dont handle the error case
      result.set(i, j, sum);
    }
  }

  return result;
}
pub fn subtraction<T>(mat1: &Mat<T>, mat2: &Mat<T>) -> Mat<T>
where
  T: Sub<Output = T>
    + PartialEq
    + NumCast
    + Zero
    + CheckedMul
    + PartialOrd
    + Display
    + PrimInt
{
  assert!(
    mat1.rows == mat2.rows && mat1.cols == mat2.cols,
    "Matrix dimensions must match. Got Mat1: ({}x{}) and Mat2: ({}x{})",
    mat1.rows,
    mat1.cols,
    mat2.rows,
    mat2.cols
  );

  let rows_usize = to_usize!(mat1.rows);
  let cols_usize = to_usize!(mat1.cols);

  let rows = mat1.rows;
  let cols = mat1.cols;
  let mut result = Mat::new(rows, cols);

  for i in 0..rows_usize {
    for j in 0..cols_usize {
      let value1 = safe_get!(mat1, i, j);
      let value2 = safe_get!(mat2, i, j);
      let diff = value1.sub(value2);
      result.set(i, j, diff);
    }
  }

  result
}

pub fn dot_product<T>(mat1: &Mat<T>, mat2: &Mat<T>) -> Mat<T>
where
  T: Add<Output = T>
    + PartialEq
    + NumCast
    + Zero
    + CheckedMul
    + PartialOrd
    + Display
    + PrimInt
    + Default,
{
  assert!(
        mat1.cols == mat2.rows,
        // For Multiplications mat1 cols must match mat2 rows
        "For Multiplications Mat1 Cols:({}) must match Mat2 Rows: ({})! \n . Got Mat1: ({}x{}) and Mat2: ({}x{})", mat1.cols, mat2.rows, mat1.rows, mat1.cols, mat2.rows, mat2.cols
    );
  let result_rows_usize = to_usize!(mat1.rows);
  let mat1_cols_usize = to_usize!(mat1.cols);
  let result_cols_usize = to_usize!(mat2.cols);
  let rows = mat1.rows;
  let cols = mat2.cols;
  let mut result = Mat::new(rows, cols);

  for i in 0..result_rows_usize {
    for j in 0..result_cols_usize {
      let mut sum = 0.0;
      for k in 0..mat1_cols_usize {
        let value1 = safe_get!(mat1, i, k);
        let value2 = safe_get!(mat2, k, j);
        sum = sum + (value1 * value2);
      }
      result.set(i, j, sum);
    }
  }

  return result;
}
