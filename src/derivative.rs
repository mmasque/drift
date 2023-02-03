use std::usize;

use crate::float::F64;
use ndarray::{self, Array2};
// TODO R^n -> R^m support
// TODO reverse mode support

pub fn derivative<F>(f: F, x: f64) -> F64
where
    F: Fn(F64) -> F64,
{
    f(F64::variable(x))
}

pub fn gradient<F>(f: F, x: &ndarray::Array1<f64>) -> ndarray::Array1<f64>
where
    F: Fn(&ndarray::Array1<F64>) -> F64,
{
    let mut result = ndarray::Array1::<f64>::zeros(x.len());
    for i in 0..x.len() {
        result[i] = differential(&f, x, i).dx;
    }
    result
}

// TODO it is not ideal to have coords passed as an int
pub fn differential<F>(f: F, x: &ndarray::Array1<f64>, coord: usize) -> F64
where
    F: Fn(&ndarray::Array1<F64>) -> F64,
{
    let k: ndarray::Array1<F64> = x
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i == coord {
                F64::variable(*a)
            } else {
                F64::constant(*a)
            }
        })
        .collect();
    f(&k)
}

pub fn differential_n<F>(f: F, x: &ndarray::Array1<f64>, coord: usize) -> ndarray::Array1<F64>
where
    F: Fn(&ndarray::Array1<F64>) -> ndarray::Array1<F64>,
{
    let k: ndarray::Array1<F64> = x
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i == coord {
                F64::variable(*a)
            } else {
                F64::constant(*a)
            }
        })
        .collect();
    f(&k)
}

/// Jacobian matrix calculation. Convention used: each input variable in its own row (i.e. row 0 is partials of x0, etc).  
pub fn jacobian<F>(f: F, x: &ndarray::Array1<f64>) -> ndarray::Array2<f64>
where
    F: Fn(&ndarray::Array1<F64>) -> ndarray::Array1<F64>,
{
    // have to run the computation for each input dimension
    // TODO right now we cannot know the size of the output array of f,
    // which is why I might move this to nalgebra.
    let nrows = x.len();
    let mut nested = Vec::new();
    for i in 0..x.len() {
        // this is the recommended way: https://docs.rs/ndarray/0.15.1/ndarray/struct.ArrayBase.html#conversions-from-nested-vecsarrays
        let col: Vec<f64> = differential_n(&f, x, i).iter().map(|x| x.dx).collect();
        nested.extend_from_slice(&col);
    }
    // TODO proper error handling
    Array2::from_shape_vec((nrows, nested.len() / nrows), nested).unwrap()
}
