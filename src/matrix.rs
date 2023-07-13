#[path = "util/functions.rs"]
mod functions;
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

macro_rules! range {
  ($start:expr, $end:expr) => {
    ($start..$end)
  };
}

pub mod matrix {

  use num_traits::NumCast;
  use rand::{thread_rng, Rng};
  use std::ops::Sub;

  use super::functions;

  #[derive(Clone)]
  pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub stride: usize, // How far do want to jump in case we want to split the matrixes
    pub data_stream: *mut f64,
  }
  impl Mat {
    pub fn new(rows: usize, cols: usize) -> Mat {
      assert!(rows > 0, "Number of rows must be greater than 0.");
      assert!(cols > 0, "Number of columns must be greater than 0.");
      let num_elements = match rows.checked_mul(cols) {
        Some(result) => result,
        None => panic!("Multiplication overflow"),
      };

      let data_stream = Box::into_raw(vec![0u64; num_elements].into_boxed_slice()) as *mut f64;

      return Mat {
        rows,
        cols,
        stride: cols,
        data_stream,
      };
    }

    pub fn sigmoid(&mut self) {
      for i in range!(0, (self.rows)) {
        for j in range!(0, (self.cols)) {
          let value_f64: f64 = safe_get!(self, i, j);
          let sigmoid = functions::sigmoid(value_f64);
          self.set(i, j, sigmoid);
        }
      }
    }

    pub fn print(
      &self,
      name: &str,
      overwrite_padding: Option<usize>,
      overwrite_precision: Option<usize>,
    ) {
      let padding = overwrite_padding.unwrap_or(4);
      let precision = overwrite_precision.unwrap_or(4);

      println!(
        "{:padding$}Mat {} ({} x {}):",
        "",
        name,
        self.rows,
        self.cols,
        padding = padding
      );
      println!(
        "{:padding$}Memory location: {:?}",
        "",
        self.data_stream,
        padding = padding
      );

      // Calculate the maximum number of digits in column indices
      let max_col_digits = (self.cols).to_string().len();

      unsafe {
        println!("{:padding$}[", "", padding = padding);
        for i in range!(0, self.rows) {
          print!("{:padding$}", "", padding = padding * 2);
          for j in range!(0, self.cols) {
            let value = *self.data_stream.add(i * self.cols + j);
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
      let mut rng = thread_rng();
      for i in range!(0, self.rows) {
        for j in range!(0, self.cols) {
          let random_value = rng.gen_range(low..=high);
          let index = i * (self.cols) + j;
          unsafe {
            self.data_stream.add(index).write(random_value);
          }
        }
      }
    }

    pub fn fill<T>(&mut self, value: T)
    where
      T: Into<f64> + Copy,
    {
      let value_f64 = value.into();
      let data_stream =
        unsafe { std::slice::from_raw_parts_mut(self.data_stream, self.rows * self.cols) };

      for element in data_stream.iter_mut() {
        *element = value_f64;
      }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
      assert!(
        row < self.rows,
        "Get failed! Supplied row Index {} is out of bounds. Matrix has {} rows.",
        row,
        self.rows
      );
      assert!(
        col < self.cols,
        "Get failed! Supplied column Index {} is out of bounds. Matrix has {} columns.",
        col,
        self.cols
      );
      let index = row * self.stride + col;
      unsafe {
        return Some(self.data_stream.add(index).read().clone());
      }
    }

    pub fn set<T>(&mut self, row: usize, col: usize, value: T)
    where
      T: Into<f64> + Copy,
    {
      let value_f64 = value.into();
      let rows_usize = self.rows;
      let cols_usize = self.cols;

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
        self.data_stream.add(index).write(value_f64);
      }
    }
    fn drop(&mut self) {
      let num_elements = self.rows * self.cols;
      unsafe {
        Vec::from_raw_parts(self.data_stream, 0, num_elements);
      }
    }
  }

  pub fn addition(mat1: &Mat, mat2: &Mat) -> Mat {
    assert!(
      mat1.rows == mat2.rows && mat1.cols == mat2.cols,
      "Matrix dimensions must match. Got Mat1: ({}x{}) and Mat2: ({}x{})",
      mat1.rows,
      mat1.cols,
      mat2.rows,
      mat2.cols
    );
    let rows = mat1.rows;
    let cols = mat1.cols;
    let mut result = Mat::new(rows, cols);

    for i in 0..mat1.rows {
      for j in 0..mat1.cols {
        let value1 = safe_get!(mat1, i, j);
        let value2 = safe_get!(mat2, i, j);
        let sum = value1 + value2;
        // This might make problems, because we dont handle the error case
        result.set(i, j, sum);
      }
    }

    return result;
  }
  pub fn subtraction(mat1: &Mat, mat2: &Mat) -> Mat {
    assert!(
      mat1.rows == mat2.rows && mat1.cols == mat2.cols,
      "Matrix dimensions must match. Got Mat1: ({}x{}) and Mat2: ({}x{})",
      mat1.rows,
      mat1.cols,
      mat2.rows,
      mat2.cols
    );

    let rows = mat1.rows;
    let cols = mat1.cols;
    let mut result = Mat::new(rows, cols);

    for i in 0..rows {
      for j in 0..cols {
        let value1 = safe_get!(mat1, i, j);
        let value2 = safe_get!(mat2, i, j);
        let diff = value1.sub(value2);
        result.set(i, j, diff);
      }
    }

    result
  }

  pub fn dot_product(mat1: &Mat, mat2: &Mat) -> Mat {
    assert!(
        mat1.cols == mat2.rows,
        // For Multiplications mat1 cols must match mat2 rows
        "For Multiplications Mat1 Cols:({}) must match Mat2 Rows: ({})! \n . Got Mat1: ({}x{}) and Mat2: ({}x{})", mat1.cols, mat2.rows, mat1.rows, mat1.cols, mat2.rows, mat2.cols
    );
    let mut result: Mat = Mat::new(mat1.rows, mat2.cols);

    for i in 0..mat1.rows {
      for j in 0..mat2.cols {
        let mut sum = 0.0;
        for k in 0..mat1.cols {
          let value1 = safe_get!(mat1, i, k);
          let value2 = safe_get!(mat2, k, j);
          sum = sum + (value1 * value2);
          println!("{} {} {}", value1, value2, sum)
        }
        result.set(i, j, sum);
      }
    }

    return result;
  }
  pub fn mat_row(m: &Mat, row: usize) -> Mat {
    let index = row * m.stride;
    return Mat {
      rows: 1,
      cols: m.cols,
      stride: m.stride,
      data_stream: unsafe { m.data_stream.add(index) },
    };
  }

  pub fn mat_copy(m_dest: &mut Mat, m_src: &Mat) {
    assert!(
      m_dest.cols == m_src.cols && m_dest.rows == m_src.rows,
      "Matrices must have the same dimensions. Got Mat1: ({}x{}) and Mat2: ({}x{})",
      m_dest.rows,
      m_dest.cols,
      m_src.rows,
      m_src.cols
    );
    for i in range!(0, m_src.rows) {
      for j in range!(0, m_src.cols) {
        let value = safe_get!(m_src, i, j);
        m_dest.set(i, j, value);
      }
    }
  }
}
