mod matrix;
use matrix::Mat;

#[cfg(test)]
mod tests {
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
    fn test_matrix_multiplication() {
        let mat1: Mat<u32> = Mat::new(2, 3);
        let mat2: Mat<u32> = Mat::new(3, 2);
        let result = matrix::dot_product(&mat1, &mat2);

        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
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
        // Create a matrix
        let mat = Mat::new(3, 3);

        // Call the rand() function
        mat.rand(0, 9);

        // Check that all elements are within the specified range
        for i in 0..3 {
            for j in 0..3 {
                let value = mat.get(i, j).unwrap();
                assert!(
                    value >= 0 && value <= 9,
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
