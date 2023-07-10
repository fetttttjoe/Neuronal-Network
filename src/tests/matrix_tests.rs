#[path = "../matrix/matrix.rs"]
mod matrix;
use matrix::Mat;
// I dont understand why sharing is broken now.
// So I had to copy the macro code from matrix.rs
macro_rules! to_usize {
  ($value:expr) => {
    match $value.to_usize() {
      Some(result) => result,
      _ => panic!("Conversion to usize failed"),
    }
  };
}
#[cfg(test)]
mod tests {
  use num_traits::ToPrimitive;

  use super::*;

  #[test]
  fn test_new_matrix() {
    let mat: Mat<u32> = Mat::new(2, 3);
    assert_eq!(mat.rows, 2);
    assert_eq!(mat.cols, 3);
  }

  #[test]
  fn test_set_and_get() {
    let mut mat: Mat<u32> = Mat::new(2, 2);
    mat.set(0, 0, 1);
    mat.set(0, 1, 2);
    mat.set(1, 0, 3);
    mat.set(1, 1, 4);

    assert_eq!(mat.get(0, 0), Some(1));
    assert_eq!(mat.get(0, 1), Some(2));
    assert_eq!(mat.get(1, 0), Some(3));
    assert_eq!(mat.get(1, 1), Some(4));
  }

  #[test]
  fn test_mat_get_in_bound() {
    let mut mat: Mat<i32> = Mat::new(3, 3);
    mat.set(1, 1, 42);
    let value = mat.get(1, 1);
    assert_eq!(value, Some(42));
  }

  #[test]
  fn test_mat_set_in_bound() {
    let mut mat: Mat<i32> = Mat::new(3, 3);
    mat.set(1, 1, 42);
    let value = mat.get(1, 1);
    assert_eq!(value, Some(42));
  }

  #[test]
  #[should_panic(
    expected = "Set failed! Supplied row Index 5 is out of bounds. Matrix has 3 rows."
  )]
  fn test_mat_set_out_of_bounds_row() {
    let mut mat: Mat<i32> = Mat::new(3, 3);
    mat.set(5, 2, 42);
  }

  #[test]
  #[should_panic(
    expected = "Set failed! Supplied column Index 5 is out of bounds. Matrix has 3 columns."
  )]
  fn test_mat_set_out_of_bounds_column() {
    let mut mat: Mat<i32> = Mat::new(3, 3);
    mat.set(2, 5, 42);
  }

  #[test]
  #[should_panic(
    expected = "Get failed! Supplied row Index 5 is out of bounds. Matrix has 3 rows."
  )]
  fn test_mat_get_out_of_bounds_row() {
    let mat: Mat<i32> = Mat::new(3, 3);
    let _value = mat.get(5, 2);
  }

  #[test]
  #[should_panic(
    expected = "Get failed! Supplied column Index 5 is out of bounds. Matrix has 3 columns."
  )]
  fn test_mat_get_out_of_bounds_column() {
    let mat: Mat<i32> = Mat::new(3, 3);
    let _value = mat.get(2, 5);
  }

  #[test]
  fn test_fill() {
    let mut mat: Mat<u32> = Mat::new(2, 2);
    mat.fill(5);

    assert_eq!(mat.get(0, 0), Some(5));
    assert_eq!(mat.get(0, 1), Some(5));
    assert_eq!(mat.get(1, 0), Some(5));
    assert_eq!(mat.get(1, 1), Some(5));
  }

  #[test]
  fn test_matrix_multiplication_dimensions() {
    let mat1: Mat<u32> = Mat::new(2, 3);
    let mat2: Mat<u32> = Mat::new(3, 2);
    let result = matrix::dot_product(&mat1, &mat2);

    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 2);
  }
  #[test]
  fn test_addition() {
    // Create two matrices
    let mut mat1 = Mat::new(2, 2);
    mat1.fill(2);
    let mut mat2 = Mat::new(2, 2);
    mat2.fill(3);

    // Perform addition
    let result = matrix::addition(&mat1, &mat2);

    // Expected result
    let mut expected = Mat::new(2, 2);
    expected.fill(5);

    // Compare the actual result with the expected result
    assert_eq!(result.rows, expected.rows);
    assert_eq!(result.cols, expected.cols);

    for i in 0..to_usize!(result.rows) {
      for j in 0..to_usize!(result.cols) {
        assert_eq!(result.get(i, j), expected.get(i, j));
      }
    }
  }

  #[test]
  fn test_subtraction() {
    // Create two matrices
    let mut mat1 = Mat::new(2, 2);
    mat1.fill(5);
    let mut mat2 = Mat::new(2, 2);
    mat2.fill(3);

    // Perform subtraction
    let result = matrix::subtraction(&mat1, &mat2);

    // Expected result
    let mut expected = Mat::new(2, 2);
    expected.fill(2);

    // Compare the actual result with the expected result
    assert_eq!(result.rows, expected.rows);
    assert_eq!(result.cols, expected.cols);

    for i in 0..to_usize!(result.rows) {
      for j in 0..to_usize!(result.cols) {
        assert_eq!(result.get(i, j), expected.get(i, j));
      }
    }
  }

  #[test]
  fn test_dot_product() {
    // Create two matrices
    let mut mat1 = Mat::new(2, 3);
    mat1.fill(2);
    let mut mat2 = Mat::new(3, 2);
    mat2.fill(3);

    // Perform dot product
    let result = matrix::dot_product(&mat1, &mat2);

    // Expected result
    let mut expected = Mat::new(2, 2);
    expected.set(0, 0, 18);
    expected.set(0, 1, 18);
    expected.set(1, 0, 18);
    expected.set(1, 1, 18);

    // Compare the actual result with the expected result
    assert_eq!(result.rows, expected.rows);
    assert_eq!(result.cols, expected.cols);

    for i in 0..to_usize!(result.rows) {
      for j in 0..to_usize!(result.cols) {
        assert_eq!(result.get(i, j), expected.get(i, j));
      }
    }
  }
  #[test]
  fn test_matrix_addition() {
    let mat1: Mat<u32> = Mat::new(2, 2);
    let mat2: Mat<u32> = Mat::new(2, 2);
    let result = matrix::addition(&mat1, &mat2);

    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 2);
  }

  #[test]
  #[should_panic]
  fn test_matrix_addition_incompatible_dimensions() {
    let mat1: Mat<u32> = Mat::new(2, 2);
    let mat2: Mat<u32> = Mat::new(2, 3); // Incompatible dimensions for addition
    let _result = matrix::addition(&mat1, &mat2); // This should panic
  }

  #[test]
  fn test_rand() {
    let rows = 3;
    let cols = 3;
    // Create a matrix
    let mat = Mat::new(rows, cols);

    let low = 0.0;
    let high = 9.0;
    // Call the rand() function
    mat.rand(low, high);

    // Check that all elements are within the specified range
    for i in 0..rows {
      for j in 0..cols {
        let value = mat.get(i, j).unwrap();
        assert!(
          value >= low.to_usize().unwrap() && value <= high.to_usize().unwrap(),
          "Element at ({}, {}) is out of range",
          i,
          j
        );
      }
    }
  }

  #[test]
  #[should_panic(expected = "Multiplication overflow")]
  fn test_mat_creation_overflow() {
    // This should panic with "Multiplication overflow"
    let _mat: Mat<u32> = Mat::new(u32::MAX, 2);
  }
}
