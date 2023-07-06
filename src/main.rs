mod matrix;
use matrix::Mat as Mat;
fn main() {
    let mut n =  Mat::new(15, 20);
    let mut m =  Mat::new(15, 20);
    n.fill(10);
    m.fill(10);
    
    // n.rand(1, 10);
    // m.rand(1, 10);
    // subsctraction
    let add = matrix::subtraction(&n, &m);
    println!("All 0");
    add.print(None);
    println!("----------------------------------------------");
    // addition
    println!("All 20");
    let add = matrix::addition(&n, &m);
    add.print(None);
    println!("----------------------------------------------");

    // Multiply
    let mut x = Mat::new(20, 3);
    let mut y = Mat::new(3, 20);
    x.fill(1);
    y.fill(2);
    let multiply = matrix::dot_product(&x, &y);
    println!("All 6");
    multiply.print(None);
    println!("----------------------------------------------");


}
