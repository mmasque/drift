use crate::float::F64;

mod float;
mod primitives;

fn main() {
    println!("Hello, world!");
    let test = F64 { x: 1.0, dx: 1.0 };
    let test2 = F64 { x: 2.0, dx: 1.0 };
    let sum = test + &test2;
    println!("{:?}", sum);
}
