#[path = "matrix/matrix.rs"]
mod matrix;
#[path = "network/network.rs"]
mod network;
use crate::matrix::addition;
use crate::matrix::dot_product;
use crate::matrix::subtraction;
use crate::network::Network;
use matrix::Mat;
fn main() {
  // let mut n =  Mat::new(15, 20);
  // let mut m =  Mat::new(15, 20);
  // n.fill(10);
  // m.fill(10);

  // // subsctraction
  // let sub = subtraction(&n, &m);
  // println!("All 0");
  // sub.print(None);
  // println!("----------------------------------------------");
  // // addition
  // println!("All 20");
  // let add = addition(&n, &m);
  // add.print(None);
  // println!("----------------------------------------------");
  // // Multiply
  // let mut x = Mat::new(20, 3);
  // let mut y = Mat::new(3, 20);
  // x.fill(1);
  // y.fill(2);
  // let multiply = dot_product(&x, &y);
  // println!("All 6");
  // multiply.print(None);
  // println!("----------------------------------------------");
  let mut network = network::Network::new(&[2, 2, 1]);
  network.rand(0.0, 1.0);
  network.print(Some(5), Some(10));
}
