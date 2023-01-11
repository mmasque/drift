use std::ops::Rem;

use auto_ops::{impl_op, impl_op_ex};
use num::{traits::Float, Num, One, Zero};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct F64 {
    pub x: f64,
    dx: f64,
}

impl F64 {
    fn new_internal(x: f64, dx: f64) -> Self {
        F64 { x: x, dx: dx }
    }
    pub fn new(x: f64) -> Self {
        F64 { x: x, dx: 0.0 }
    }

    pub fn variable(x: f64) -> Self {
        F64 { x: x, dx: 1.0 }
    }
}

impl Rem for F64 {
    type Output = F64;
    fn rem(self, rhs: Self) -> Self::Output {
        F64 {
            x: self.x % rhs.x,
            dx: self.dx,
        }
    }
}

impl One for F64 {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.x.is_one()
    }
    fn one() -> Self {
        F64::new(f64::one())
    }

    fn set_one(&mut self) {
        // sets dx to 0, I guess TODO what is best?
        self.x.set_one();
        self.dx.set_zero();
    }
}

impl Zero for F64 {
    fn is_zero(&self) -> bool {
        self.x.is_zero()
    }
    fn zero() -> Self {
        F64::new(f64::zero())
    }
    fn set_zero(&mut self) {
        self.x.set_zero();
        self.dx.set_zero();
    }
}

impl_op!(-|a: F64| -> F64 { F64::new_internal(-a.x, -a.dx) });
impl_op_ex!(+ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x + b.x, a.dx + b.dx)} );
impl_op_ex!(-|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x - b.x, a.dx - b.dx) });
impl_op_ex!(*|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x * b.x, a.dx * b.x + a.x * b.dx) });
impl_op_ex!(/ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x / b.x, (a.dx * b.x - a.x * b.dx) / (b.x * b.x)) });

//TODO impl Float for F64 {}
