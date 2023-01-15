use num::Float;

use crate::derivative::{derivative, differential, gradient};
use crate::float::F64;
mod derivative;
mod float;
fn main() {
    println!("Expect derivative at 5: 3 * a - 2 * a = 3*5^2 - 2 = 73");
    println!("Calculated derivative at 5: {:?}", derivative(&simple, 5.0));
    println!("Expect differential in first coord at (5,2) to be: 155");
    println!(
        "Calculated differential in first coord is: {:?}",
        differential(&simple_arr, &[5.0, 2.0], 0)
    );
    println!("Expect gradient at (5,2) to be: (155, 125)");
    println!(
        "Calculated gradient is: {:?}",
        gradient(&simple_arr, &[5.0, 2.0])
    );
}

fn simple(a: F64) -> F64 {
    a * (a * a - F64::new(2.0))
}

fn simple_arr(a: &[F64; 2]) -> F64 {
    // x^3 * y + 5x.
    // dx is 3x^2y + 5; dy is x^3. - at (5,2) dx is 155, 125

    a[0].powi(3) * a[1] + F64::new(5.0) * a[0]
}
