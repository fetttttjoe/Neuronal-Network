use num_traits::{CheckedMul, NumCast, ToPrimitive, Zero};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::Mul;
macro_rules! ternary {
    ($condition:expr, $true_expr:expr, $false_expr:expr) => {
        if $condition {
            $true_expr
        } else {
            $false_expr
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
        + std::fmt::Debug
        + std::fmt::Display,
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
        println!("Mat ({:?} x {:?}):", self.rows, self.cols);
        println!("Memory location: {:?}", self.data_stream);

        let num_elements_usize = match (self.rows.to_usize(), self.cols.to_usize()) {
            (Some(rows_usize), Some(cols_usize)) => rows_usize * cols_usize,
            _ => panic!("Conversion to usize failed"),
        };

        // Calculate the maximum number of digits in column indices
        let max_col_digits = self.cols.to_usize().unwrap().to_string().len();

        // Print column indicators
        print!("{:<width$}", "", width = max_col_digits + padding + 4); // Add padding + 2 for spacing
        for j in 0..self.cols.to_usize().unwrap() {
            print!("{:<width$}", j, width = max_col_digits + padding);
        }
        println!();
        println!();

        let mut current_row = 0;
        unsafe {
            for i in 0..num_elements_usize {
                let value = *self.data_stream.add(i);

                // Check if a new row starts
                if i % self.cols.to_usize().unwrap() == 0 {
                    current_row += 1;
                    print!(
                        "Row {:<width$}",
                        current_row,
                        width = max_col_digits + padding
                    );
                }

                print!("{:<width$}", value, width = max_col_digits + padding); // Adjust the width as per your preference

                // Check if a new row starts
                if (i + 1) % self.cols.to_usize().unwrap() == 0 {
                    println!(); // Print a new line
                }
            }
        }
    }
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
