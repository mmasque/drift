pub mod derivative;
pub mod float;

#[cfg(test)]
mod tests {
    use crate::{
        derivative::{derivative, differential, differential_n, gradient, jacobian},
        float::F64,
    };
    use ndarray::array;
    use num::Float;

    fn simple_arr(a: &ndarray::Array1<F64>) -> F64 {
        // x^3 * y + 5x.
        // dx is 3x^2y + 5; dy is x^3. - at (5,2) dx is 155, 125

        a[0].powi(3) * a[1] + F64::constant(5.0) * a[0]
    }

    fn simple_arr_n(a: &ndarray::Array1<F64>) -> ndarray::Array1<F64> {
        // x^3 - y^2, x^3 + y^2
        ndarray::array![a[0].powi(3) - a[1].powi(2), a[0].powi(3) + a[1].powi(2)]
    }

    fn simple_arr_n_m(a: &ndarray::Array1<F64>) -> ndarray::Array1<F64> {
        // x^3 - y^2, x^3 + y^2 + z^2
        ndarray::array![
            a[0].powi(3) - a[1].powi(2),
            a[0].powi(3) + a[1].powi(2) + a[2].powi(2)
        ]
    }
    #[test]
    fn test_sum() {
        let a = [
            F64 { x: 1.0, dx: -1.0 },
            F64 { x: 2.0, dx: -2.0 },
            F64 { x: 3.0, dx: -3.0 },
        ];
        assert_eq!(a.iter().sum::<F64>(), F64 { x: 6.0, dx: -6.0 });
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
        assert_eq!(
            differential(simple_arr, &ndarray::array![5.0, 2.0], 0).dx,
            155.0
        )
    }

    #[test]
    fn test_gradient() {
        assert_eq!(
            gradient(simple_arr, &ndarray::array![5.0, 2.0]),
            ndarray::array![155.0, 125.0]
        )
    }

    #[test]
    fn test_differential_n() {
        // [3x^2, 3x^2] for dx and [-2y, 2y] for dy -> [3*5^2, 3 * 5^2] = [75, 75] for dx, [-4, 4] for dy
        assert_eq!(
            differential_n(simple_arr_n, &ndarray::array![5.0, 2.0], 1).map(|x| x.dx),
            ndarray::array![-4.0, 4.0]
        );
    }

    #[test]
    fn test_jacobian_square() {
        // simple_array_n has [3x^2, 3x^2] for dx and [-2y, 2y] for dy -> [3*5^2, 3 * 5^2] = [75, 75] for dx, [-4, 4] for dy
        // at [5, 2]. jacobian rows are dfi/dx, i.e. variable per row, function component per col
        assert_eq!(
            jacobian(simple_arr_n, &ndarray::array![5.0, 2.0]),
            array![[75.0, 75.0], [-4.0, 4.0]]
        )
    }
    fn test_jacobian_3_2() {
        // jacobian test with 3 input dims, 2 output dims
        assert_eq!(
            jacobian(simple_arr_n_m, &ndarray::array![5.0, 2.0, 3.0]),
            array![[75.0, 75.0], [-4.0, 4.0], [0.0, 6.0]]
        )
    }
}
