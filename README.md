# drift
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
[![codecov](https://codecov.io/gh/mmasque/drift/branch/main/graph/badge.svg?token=YNBO0EP77J)](https://codecov.io/gh/mmasque/drift)
[![build](https://github.com/mmasque/drift/actions/workflows/CI.yml/badge.svg)
A rudimentary implementation of automatic differentiation written in Rust. Currently supports forward mode 
automatic differentiation using Operator Overloading. Currently supports functions of the form R^n -> R. 
Can compute individual partial derivatives via `differential`, gradient via `gradient`. `derivative` can be used when
dealing with R -> R functions. 

The idea is to use a custom float type, `F64` which implements the `num::Float` trait and the usual binary operations
(+,-,*,/). Then derivatives can be computed next to function outputs. 

## Example

```rust
    fn simple_arr(a: ndarray::Array1<F64>) -> F64 {
        // x^3 * y + 5x.
        // dx is 3x^2y + 5; dy is x^3. - at (5,2) dx is 155, 125
        a[0].powi(3) * a[1] + F64::constant(5.0) * a[0]
    }
    assert_eq!(
        differential(simple_arr, &ndarray::array![5.0, 2.0], 0).dx,
        155.0
    )
```

## Todos
1. Reverse mode support. This is useful because I'm currently using `drift` in `mmasque/translator` where I'm implementing some basic neural networks. 
Backpropagation there ends up being a special case of reverse mode autodiff so it will likely simplify the code. 

## References
The [autodiff](https://github.com/elrnv/autodiff) crate was used to cross-check definitions and API setup. 
