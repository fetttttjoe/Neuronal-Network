#[path = "../network.rs"]
mod network;
#[cfg(test)]
mod tests {
  use crate::network::network::Network as NN;

  #[test]
  fn test_network_drop() {
    // Create a mock Network instance
    let network = NN::new(&[2, 3, 1]);

    // Call the drop function explicitly
    drop(&network);

    // No assertions are required as the drop function is responsible for resource cleanup
  }
  #[test]
  fn test_network_new() {
    let arch = [2, 3, 1];
    let network = NN::new(&arch);

    // Assert that the network count is set correctly
    assert_eq!(network.count, 2);

    // Assert that the network weights, bias, and activations are not null
    assert!(!network.weights.is_null());
    assert!(!network.bias.is_null());
    assert!(!network.activations.is_null());
  }

  #[test]
  fn test_network_structure() {
    let arch = [2, 3, 1];
    let network = NN::new(&arch);

    // Assert that the network count is set correctly
    assert_eq!(network.count, arch.len() - 1);

    // Assert that the weights, bias, and activations pointers are not null
    assert!(!network.weights.is_null());
    assert!(!network.bias.is_null());
    assert!(!network.activations.is_null());

    // Convert the pointers back to Vec to perform further assertions
    let weights_mat_array = network.get_weights();
    let bias_mat_array = network.get_bias();
    let activations_mat_array = network.get_activations();

    // Assert that the lengths of the Vecs match the expected count
    assert_eq!(weights_mat_array.len(), network.count);
    assert_eq!(bias_mat_array.len(), network.count);
    assert_eq!(activations_mat_array.len(), network.count + 1);

    // Assert that each matrix in weights, bias, and activations has the expected dimensions
    for i in 0..network.count {
      let weights_matrix = &weights_mat_array[i];
      assert_eq!(weights_matrix.rows, arch[i]);
      assert_eq!(weights_matrix.cols, arch[i + 1]);

      let bias_matrix = &bias_mat_array[i];
      assert_eq!(bias_matrix.rows, 1);
      assert_eq!(bias_matrix.cols, arch[i + 1]);

      let activation_matrix = &activations_mat_array[i];
      assert_eq!(activation_matrix.rows, 1);
      assert_eq!(activation_matrix.cols, arch[i + 1]);
    }
  }
}
