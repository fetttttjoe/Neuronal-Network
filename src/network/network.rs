#[path = "../matrix/matrix.rs"]
mod matrix;

use matrix::Mat;
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
  // ... existing code

  pub fn get_weights(&self) -> &[Mat<usize>] {
    unsafe { std::slice::from_raw_parts(self.weights, self.count) }
  }

  pub fn get_bias(&self) -> &[Mat<usize>] {
    unsafe { std::slice::from_raw_parts(self.bias, self.count) }
  }

  pub fn get_activations(&self) -> &[Mat<usize>] {
    unsafe { std::slice::from_raw_parts(self.activations, self.count + 1) }
  }

  pub fn print(&self, overwrite_padding: Option<usize>) {
    let padding = overwrite_padding.unwrap_or(4);

    println!("Network: ");
    println!("[",);
    for i in 0..self.count {
        print!("{:padding$}", "", padding = padding);
        unsafe {
            println!("{:padding$}weights[{}]: ", "", i, padding = padding / 2);
            println!("{:padding$}[", "", padding=padding / 2);
            (*self.weights.add(i)).print(Some(padding));
            println!("{:padding$}]", "", padding=padding / 2);
        }
        unsafe {
            println!("{:padding$}bias[{}]: ", "", i, padding = padding / 2);
            println!("{:padding$}[", "", padding=padding / 2);
            (*self.bias.add(i)).print(Some(padding));
            println!("{:padding$}]", "", padding=padding / 2);
        }
    }
    println!("]");
}


  fn drop(&mut self) {
    let mut weights = unsafe { Vec::from_raw_parts(self.weights, self.count, self.count) };
    let mut bias = unsafe { Vec::from_raw_parts(self.bias, self.count, self.count) };
    let mut activations =
      unsafe { Vec::from_raw_parts(self.activations, self.count + 1, self.count + 1) };

    // Explicitly drop each matrix in reverse order
    for i in (0..self.count).rev() {
      let _ = std::mem::replace(&mut weights[i], Mat::new(0, 0));
      let _ = std::mem::replace(&mut bias[i], Mat::new(0, 0));
      let _ = std::mem::replace(&mut activations[i], Mat::new(0, 0));
    }
    let _ = std::mem::replace(&mut activations[self.count], Mat::new(0, 0));

    // Deallocate the memory
    std::mem::drop(weights);
    std::mem::drop(bias);
    std::mem::drop(activations);
  }
}
