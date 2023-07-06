mod matrix;

fn main() {
    let mut n =  matrix::Mat::new(15, 20);
    let mut m =  matrix::Mat::new(15, 20);
    n.fill(10);
    m.fill(10);
    
    // n.rand(1, 10);
    // m.rand(1, 10);
    let add = matrix::addition(&n, &m);
    add.print(None);

    // Multiply
    let mut x = matrix::Mat::new(20, 3);
    let mut y = matrix::Mat::new(3, 20);
    x.fill(1);
    y.fill(2);
    let multiply = matrix::dot_product(&x, &y);
    multiply.print(None);

}
