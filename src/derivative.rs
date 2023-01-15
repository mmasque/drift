use std::{convert::TryInto, usize};

use crate::float::F64;

// TODO R^n -> R^m support
// TODO reverse mode support
// TODO borrowed/owned args

pub fn derivative<F>(f: F, x: f64) -> F64
where
    F: Fn(F64) -> F64,
{
    f(F64::variable(x))
}

// TODO it is not ideal to have coords passed as an int
pub fn gradient<F, const S: usize>(f: F, x: &[f64; S]) -> [f64; S]
where
    F: Fn(&[F64; S]) -> F64,
{
    let mut result = [0.0; S];
    for i in 0..S {
        result[i] = differential(&f, x, i).dx;
    }
    result
}

pub fn differential<F, const S: usize>(f: F, x: &[f64; S], coord: usize) -> F64
where
    F: Fn(&[F64; S]) -> F64,
{
    assert!(coord < S, "Coordinate not in input range");
    let k: &[F64; S] = &x
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i == coord {
                F64::variable(*a)
            } else {
                F64::constant(*a)
            }
        })
        .collect::<Vec<F64>>()
        .try_into()
        .unwrap();
    f(k)
}
