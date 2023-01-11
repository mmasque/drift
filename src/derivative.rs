use crate::float::F64;

// TODO multivariable support
// TODO do we want to return a derivative function for f instead?
// TODO borrowed/owned args

pub fn derivative<F>(f: F, x: f64) -> F64
where
    F: FnOnce(F64) -> F64,
{
    f(F64::variable(x))
}
