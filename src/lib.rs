pub mod derivative;
pub mod float;

#[cfg(test)]
mod tests {
    use crate::{
        derivative::{derivative, differential, gradient},
        float::F64,
    };
    use num::Float;

    fn simple_arr(a: &[F64; 2]) -> F64 {
        // x^3 * y + 5x.
        // dx is 3x^2y + 5; dy is x^3. - at (5,2) dx is 155, 125

        a[0].powi(3) * a[1] + F64::constant(5.0) * a[0]
    }

    fn simple_arr_n(a: &[F64; 2]) -> [F64; 2] {
        [a[0].powi(3) - a[1].powi(2), a[0].powi(3) + a[0].powi(2)]
    }

    #[test]
    fn test_derivative() {
        fn simple(a: F64) -> F64 {
            a * (a * a - F64::constant(2.0))
        }
        assert_eq!(derivative(simple, 5.0).dx, 73.0);
    }

    #[test]
    fn test_differential() {
        assert_eq!(differential(simple_arr, &[5.0, 2.0], 0).dx, 155.0)
    }

    #[test]
    fn test_gradient() {
        assert_eq!(gradient(simple_arr, &[5.0, 2.0]), [155.0, 125.0])
    }

    // #[test]
    // fn test_differential_n() {
    //     println!("{:?}", differential(simple_arr, &[5.0, 2.0], 0));
    //     // [3x^2, 3x^2] for dx and [-2y, 2y] for dy -> [3*5^2, 3 * 5^2] = [75, 75] for dx, [-10, 10] for dy
    //     assert_eq!(differential_n(simple_arr_n, &[5.0, 2.0], 1), [75.0, 75.0])
    // }
}
