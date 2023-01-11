use crate::derivative::derivative;
use crate::float::F64;
mod derivative;
mod float;
mod primitives;
fn main() {
    println!("Hello, world!");
    println!("Expect derivative at 5: 3 * a - 2 * a = 3*5^2 - 2 = 73");
    println!("{:?}", derivative(simple, 5.0));
}
// highlights our first interesting issue: a real number r has to be lifted to (r,0) while a variable x to (x,1)
// thankfully given a function f we know (through which its params are) about which are variables.

fn simple(a: F64) -> F64 {
    a * (a * a - F64::new(2.0))
}
