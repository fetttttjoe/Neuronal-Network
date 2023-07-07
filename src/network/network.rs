#[path = "../matrix/matrix.rs"]
mod matrix;

use matrix::Mat;
use num_traits::PrimInt;
use std::ptr;

pub struct Network {
    pub count: usize,
    pub weights: *mut Mat<usize>,
    pub bias: *mut Mat<usize>,
    pub activations: *mut Mat<usize>,
}

impl Network {
    pub fn new(arch: &[usize]) -> Network {
        let arch_count = arch.len();
        assert!(arch_count > 0, "Architecture must have at least one layer");

        let mut nn = Network {
            count: arch_count - 1,
            weights: ptr::null_mut(),
            bias: ptr::null_mut(),
            activations: ptr::null_mut(),
        };

        let mut weights: Vec<Mat<usize>> = Vec::with_capacity(nn.count);
        let mut bias: Vec<Mat<usize>> = Vec::with_capacity(nn.count);
        let mut activations: Vec<Mat<usize>> = Vec::with_capacity(nn.count + 1);

        for i in 0..nn.count {
            let weights_mat = Mat::new(arch[i], arch[i + 1]);
            let bias_mat = Mat::new(1, arch[i + 1]);
            let activations_mat = Mat::new(1, arch[i + 1]);

            weights.push(weights_mat);
            bias.push(bias_mat);
            activations.push(activations_mat);
        }

        nn.weights = weights.as_mut_ptr();
        nn.bias = bias.as_mut_ptr();
        nn.activations = activations.as_mut_ptr();

        // Prevent `Vec` from deallocating the memory when it goes out of scope
        std::mem::forget(weights);
        std::mem::forget(bias);
        std::mem::forget(activations);

        nn
    }

    pub fn print(&self) {
        println!("[");
        for i in 0..self.count {
            unsafe {
                (*self.weights.add(i)).print(None);
                (*self.bias.add(i)).print(None);
            }
        }
        println!("]");
    }
}
