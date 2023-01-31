use std::{convert::TryInto, usize};

use crate::float::F64;
use ndarray;
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
    F: Fn(ndarray::Array1<F64>) -> F64,
{
    let mut result = ndarray::Array1::<f64>::zeros(x.len());
    for i in 0..x.len() {
        result[i] = differential(&f, &x, i).dx;
    }
    result
}

// TODO it is not ideal to have coords passed as an int
pub fn differential<F>(f: F, x: &ndarray::Array1<f64>, coord: usize) -> F64
where
    F: Fn(ndarray::Array1<F64>) -> F64,
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
    f(k)
}

pub fn differential_n<F>(f: F, x: &ndarray::Array1<f64>, coord: usize) -> ndarray::Array1<F64>
where
    F: Fn(ndarray::Array1<F64>) -> ndarray::Array1<F64>,
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
    f(k)
}
