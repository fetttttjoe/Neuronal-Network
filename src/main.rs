mod network;
use nn::matrix::*; 
use std::mem;
fn main() {
  let stride = 3;
  let training_data: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0];
  let n = size_of_element(&training_data, stride);
  println!("n: {}", n);
  let training_inputs = Mat::new(n, 2);
  let training_outputs = Mat::new(n, 1);
  // training_inputs.print(None, None);
  // training_outputs.print(None, None);
  // let training_inputs = Mat::new()
  let mut network = network::Network::new(&[2, 2, 1]);
  network.print(Some(5), Some(10));
  let mut network_activations_vec = network.get_activations();
  let mut network_activations = network_activations_vec.as_mut_slice();
  training_inputs.print("training_inputs",None, None);
  let training_row = mat_row(&training_inputs, 1);
  network_activations[0].print("activations",None, None);
  training_row.print("training_row",None, None);
  mat_copy(
     &mut network_activations[0],
    &training_row
  );
  network_activations[0].print("activations",None, None);
  return;
  let mat1 = Mat::new(2, 2);
  let mat2 = Mat::new(2, 2);
  let mat = addition(&mat1, &mat2);
  network.print(Some(5), Some(10));
  network.print(Some(5), Some(10));
  network.print(Some(5), Some(10));
  network.print(Some(5), Some(10));
  // network.forward();
  // network.print(None, Some(30))
}

fn size_of_element<T>(vector: &[T], stride: usize) -> usize {
  let element_size = mem::size_of_val(&vector[0]);
  let vector_size = mem::size_of_val(&vector);
  return vector_size / element_size / stride;
}
