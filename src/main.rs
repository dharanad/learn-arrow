use arrow::array::Int32Array;

fn main() {
    println!("Hello, world!");
    let array = Int32Array::from_iter_values(vec![1,2,3]);
    println!("{}", array.len())
}
