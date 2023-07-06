use num_traits::{CheckedMul, FromPrimitive, NumCast, ToPrimitive, Zero};
use rand::{thread_rng, Rng};
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;
use std::default::Default;
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

                println!(
                    "Error: Failed to Get Value for Index ({}, {}).",
                    $i, $j
                );
                continue; // We mainly use that in loops, so we continue to the next iteration
            }
        }
    };
}


pub struct Mat<T: ToPrimitive> {
    pub rows: T,
    pub cols: T,
    pub data_stream: *mut u64,
}

impl<T> Mat<T>
where
    T: CheckedMul
        + NumCast
        + Zero
        + PartialOrd
        + Mul<Output = T>
        + Copy,
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

        let num_elements_usize = match NumCast::from(num_elements) {
            Some(result) => result,
            None => panic!("Conversion to usize failed"),
        };

        let data_stream =
            Box::into_raw(vec![0u64; num_elements_usize].into_boxed_slice()) as *mut u64;

        Mat {
            rows,
            cols,
            data_stream,
        }
    }

    pub fn print(&self, overwrite_padding: Option<usize>) {
        let padding = overwrite_padding.unwrap_or(4);
        let rows_usize = match self.rows.to_usize() {
            Some(rows_usize) => rows_usize,
            _ => panic!("Conversion to usize failed for Rows"),
        };
        let cols_usize = match self.cols.to_usize() {
            Some(cols_usize) => cols_usize,
            _ => panic!("Conversion to usize failed for Columns"),
        };
        println!("Mat ({} x {}):", rows_usize, cols_usize);
        println!("Memory location: {:?}", self.data_stream);

        // Calculate the maximum number of digits in column indices
        let max_col_digits = self.cols.to_usize().unwrap().to_string().len();

        unsafe {
            println!("[");
            for i in 0..rows_usize {
                print!("{:padding$}", "", padding = padding);
                for j in 0..cols_usize {
                    let value = *self.data_stream.add(i * cols_usize + j);
                    print!("{:<width$}", value, width = max_col_digits + padding);
                }
                println!();
            }
            println!("]");
        }
    }

    pub fn rand(&self, low: u64, high: u64) {
        let rows_usize = match self.rows.to_usize() {
            Some(rows_usize) => rows_usize,
            _ => panic!("Conversion to usize failed for Rows"),
        };
        let cols_usize = match self.cols.to_usize() {
            Some(cols_usize) => cols_usize,
            _ => panic!("Conversion to usize failed for Columns"),
        };
        let mut rng = thread_rng();
        for i in 0..rows_usize {
            for j in 0..cols_usize {
                let random_value = rng.gen_range(low..=high);
                let index = i * self.cols.to_usize().unwrap() + j;
                unsafe {
                    *self.data_stream.add(index) = random_value;
                }
            }
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy + NumCast,
        usize: NumCast,
    {
        let num_elements = self.rows * self.cols;
        let num_elements_usize = NumCast::from(num_elements).expect("Conversion to usize failed");
        let value_u64 = NumCast::from(value).expect("Conversion to u64 failed");

        unsafe {
            let data_stream = std::slice::from_raw_parts_mut(self.data_stream, num_elements_usize);
            for i in 0..num_elements_usize {
                *data_stream.get_unchecked_mut(i) = value_u64;
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T>
    where
        T: ToPrimitive + FromPrimitive + Copy + PartialOrd + std::fmt::Display,
    {
        let rows_usize = match self.rows.to_usize() {
            Some(rows_usize) => rows_usize,
            _ => panic!("Conversion to usize failed for Rows"),
        };
        let cols_usize = match self.cols.to_usize() {
            Some(cols_usize) => cols_usize,
            _ => panic!("Conversion to usize failed for Columns"),
        };
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
            return Some(FromPrimitive::from_u64(value).unwrap());
        }
    }
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let rows_usize = match self.rows.to_usize() {
            Some(rows_usize) => rows_usize,
            _ => panic!("Conversion to usize failed for Rows"),
        };
        let cols_usize = match self.cols.to_usize() {
            Some(cols_usize) => cols_usize,
            _ => panic!("Conversion to usize failed for Columns"),
        };
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
            *self.data_stream.add(index) = value.to_u64().expect("Conversion to u64 failed");
        }
    }
}
impl<T> Default for Mat<T>
where
    T: ToPrimitive + Zero,
{
    fn default() -> Self {
        Mat {
            rows: T::zero(),
            cols: T::zero(),
            data_stream: std::ptr::null_mut(),
        }
    }
}
pub fn addition<T>(mat1: &Mat<T>, mat2: &Mat<T>) -> Mat<T>
where
    T: Copy
        + Add<Output = T>
        + PartialEq
        + NumCast
        + num_traits::Zero
        + num_traits::CheckedMul
        + PartialOrd
        + Display
        + num_traits::FromPrimitive,
{
    assert!(
        mat1.rows == mat2.rows && mat1.cols == mat2.cols,
        "Matrix dimensions must match. Got Mat1: ({}x{}) and Mat2: ({}x{})", mat1.rows, mat1.cols, mat2.rows, mat2.cols
    );
    let rows_usize = match mat1.rows.to_usize() {
        Some(rows_usize) => rows_usize,
        _ => panic!("Conversion to usize failed for Rows"),
    };
    let cols_usize = match mat1.cols.to_usize() {
        Some(cols_usize) => cols_usize,
        _ => panic!("Conversion to usize failed for Columns"),
    };

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
pub fn dot_product<T>(mat1: &Mat<T>, mat2: &Mat<T>) -> Mat<T>
where
    T: Copy
        + Add<Output = T>
        + PartialEq
        + NumCast
        + num_traits::Zero
        + num_traits::CheckedMul
        + PartialOrd
        + Display
        + num_traits::FromPrimitive
        +Default,
{
    assert!(
        mat1.rows == mat2.cols && mat1.cols == mat2.rows,
        // For Multiplications mat1 cols must match mat2 rows
        "For Multiplications Mat1 Cols:({}) must match Mat2 Rows: ({})! \n . Got Mat1: ({}x{}) and Mat2: ({}x{})", mat1.cols, mat2.rows, mat1.rows, mat1.cols, mat2.rows, mat2.cols
    );
    let result_rows_usize = match mat1.rows.to_usize() {
        Some(rows_usize) => rows_usize,
        _ => panic!("Conversion to usize failed for Rows"),
    };
    let mat1_cols_usize = match mat1.cols.to_usize() {
        Some(rows_usize) => rows_usize,
        _ => panic!("Conversion to usize failed for Rows"),
    };
    let result_cols_usize = match mat2.cols.to_usize() {
        Some(cols_usize) => cols_usize,
        _ => panic!("Conversion to usize failed for Columns"),
    };

    let rows = mat1.rows;
    let cols = mat2.cols;
    let mut result = Mat::new(rows, cols);

    for i in 0..result_rows_usize {
        for j in 0..result_cols_usize {
            let mut sum = T::default();
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
impl<T: ToPrimitive> Drop for Mat<T> {
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