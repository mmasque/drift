// primitive functions aside from +, -, *, /

pub trait Primitives<T> {
    fn exp(x: T) -> T;
    fn log(x: T) -> T;
    fn sin(x: T) -> T;
}
// TODO pow?
