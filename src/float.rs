use auto_ops::impl_op_ex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct F64 {
    pub x: f64,
    pub dx: f64,
}

impl F64 {
    fn new_internal(x: f64, dx: f64) -> Self {
        F64 { x: x, dx: dx }
    }
    pub fn new(x: f64) -> Self {
        F64 { x: x, dx: 0.0 }
    }
}

impl_op_ex!(+ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x + b.x, a.dx + b.dx)} );
impl_op_ex!(-|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x - b.x, a.dx - b.dx) });
impl_op_ex!(*|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x * b.x, a.dx * b.x + a.x * b.dx) });
impl_op_ex!(/ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x / b.x, (a.dx * b.x - a.x * b.dx) / (b.x * b.x)) });

// TODO impl for primitives.rs functions
