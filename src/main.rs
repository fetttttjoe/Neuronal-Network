mod nn;

fn main() {
    let m =  nn::Mat::new(50, 20);
    // println!("Matrix Cols: {} Rows: {} Data Stream: {:?} !!", m.rows, m.cols, m.data_stream);
    m.print(None);
}
