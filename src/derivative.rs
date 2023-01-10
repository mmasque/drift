pub trait Derivative {
    type Type;
    fn derivative(f: Type) -> Type
    where
        F: Fn(Type) -> Type;
}

impl Derivative for F64 {
    type Type = F64;
    fn derivative(f: Self) -> Self {}
}
