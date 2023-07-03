mod nn;

fn main() {
    let mut n =  nn::Mat::new(15, 20);
    let mut m =  nn::Mat::new(15, 20);
    n.fill(10);
    m.fill(10);
    
    n.rand(1, 10);
    m.rand(1, 10);
    let add = nn::add_matrices(&n, &m);
    add.print(None);
}
