use crate::float::F64;

mod float;
mod primitives;

fn main() {
    println!("Hello, world!");
    let test = F64 { x: 5.0, dx: 1.0 };
    let sum = simple(&test);
    println!("Expect derivative at 5: 3 * a - 2 * a = 3*5^2 - 2 = 73");
    println!("{:?}", sum);
}

fn simple(a: &F64) -> F64 {
    let a1 = a * a;
    println!("{:?}", a1);
    let a2 = a1 - F64::new(2.0);
    println!("{:?}", a2);
    let a3 = a2 * a;
    println!("{:?}", a3);
    a3
}
