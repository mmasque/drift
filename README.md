# drift

A rudimentary implementation of automatic differentiation written in Rust. Currently supports forward mode 
automatic differentiation using Operator Overloading. Currently supports functions of the form R^n -> R. 
Can compute individual partial derivatives via `differential`, gradient via `gradient`. `derivative` can be used when
dealing with R -> R functions. 

The idea is to use a custom float type, `F64` which implements the `num::Float` trait and the usual binary operations
(+,-,*,/). Then derivatives can be computed next to function outputs. 

## Example

```rust
fn simple_arr(a: &[F64; 2]) -> F64 {
    a[0].powi(3) * a[1] + F64::constant(5.0) * a[0]
}
    
assert_eq!(gradient(simple_arr, &[5.0, 2.0]), [155.0, 125.0])


```
