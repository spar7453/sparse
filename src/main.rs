pub mod structs;
use structs::sc::*;

fn main() {
    let v = vec![4.5, 0.0, 3.2, 0.0,
                 3.1, 2.9, 0.0, 0.9,
                 0.0, 1.7, 3.0, 0.0,
                 3.5, 0.4, 0., 1.0];

    let reorder = CcsMatrix::to_column_order(&v, 4, 4);
    let mat = CcsMatrix::new(&v, 4, 4);

    println!("mat = {:#?}", mat);
}
