use std::ops::Rem;

use auto_ops::{impl_op, impl_op_ex};
use num::{traits::Float, Num, NumCast, One, ToPrimitive, Zero};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

impl Num for F64 {
    type FromStrRadixErr = <f64 as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f64::from_str_radix(str, radix).map(|x| F64::new(x))
    }
}

impl ToPrimitive for F64 {
    fn to_i64(&self) -> Option<i64> {
        self.x.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.x.to_u64()
    }
    fn to_f64(&self) -> Option<f64> {
        self.x.to_f64()
    }
}

impl NumCast for F64 {
    fn from<T: num::ToPrimitive>(n: T) -> Option<Self> {
        <f64 as NumCast>::from(n).map(|x| F64::new(x))
    }
}

impl_op!(-|a: F64| -> F64 { F64::new_internal(-a.x, -a.dx) });
impl_op_ex!(+ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x + b.x, a.dx + b.dx)} );
impl_op_ex!(-|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x - b.x, a.dx - b.dx) });
impl_op_ex!(*|a: &F64, b: &F64| -> F64 { F64::new_internal(a.x * b.x, a.dx * b.x + a.x * b.dx) });
impl_op_ex!(/ |a: &F64, b: &F64| -> F64 { F64::new_internal(a.x / b.x, (a.dx * b.x - a.x * b.dx) / (b.x * b.x)) });

impl Float for F64 {
    fn nan() -> Self {
        F64::new(f64::nan())
    }
    fn abs(self) -> Self {
        F64 {
            x: self.x.abs(),
            dx: match self.x.is_sign_negative() {
                true => -self.dx,
                false => self.dx,
            },
        }
    }
    fn infinity() -> Self {
        F64::new(f64::infinity())
    }
    fn neg_infinity() -> Self {
        F64::new(f64::neg_infinity())
    }
    fn neg_zero() -> Self {
        F64::new(f64::neg_zero())
    }
    fn min_value() -> Self {
        F64::new(f64::min_value())
    }
    fn abs_sub(self, other: Self) -> Self {
        (self - other).abs()
    }
    fn signum(self) -> Self {
        F64::new(self.x.signum())
    }
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.dx.is_nan()
    }
    fn acos(self) -> Self {
        F64 {
            x: self.x.acos(),
            dx: -self.dx / (1.0 - self.x * self.x).sqrt(),
        }
    }
    fn acosh(self) -> Self {
        F64 {
            x: self.x.acosh(),
            dx: self.dx / (self.x * self.x - 1.0).sqrt(),
        }
    }
    fn asin(self) -> Self {
        F64 {
            x: self.x.asin(),
            dx: self.dx / (1.0 - self.x * self.x).sqrt(),
        }
    }
    fn asinh(self) -> Self {
        F64 {
            x: self.x.asinh(),
            dx: self.dx / (1.0 + self.x * self.x).sqrt(),
        }
    }
    fn atan(self) -> Self {
        F64 {
            x: self.x.atan(),
            dx: self.dx / (1.0 + self.x * self.x),
        }
    }
    fn atan2(self, other: Self) -> Self {
        F64 {
            x: self.x.atan2(other.x),
            dx: (self.dx * other.x - self.x * other.dx) / (self.x * self.x + other.x * other.x),
        }
    }
    fn atanh(self) -> Self {
        F64 {
            x: self.x.atanh(),
            dx: self.dx / (1.0 - self.x * self.x),
        }
    }
    fn cbrt(self) -> Self {
        F64 {
            x: self.x.cbrt(),
            dx: self.dx / (3.0 * self.x.powf(2.0 / 3.0)),
        }
    }
    fn ceil(self) -> Self {
        F64 {
            x: self.x.ceil(),
            dx: self.dx,
        }
    }
    fn classify(self) -> std::num::FpCategory {
        self.x.classify()
    }
    fn copysign(self, sign: Self) -> Self {
        F64 {
            x: self.x.copysign(sign.x),
            dx: self.dx,
        }
    }
    fn cos(self) -> Self {
        F64 {
            x: self.x.cos(),
            dx: -self.dx * self.x.sin(),
        }
    }
    fn cosh(self) -> Self {
        F64 {
            x: self.x.cosh(),
            dx: self.dx * self.x.sinh(),
        }
    }
    fn epsilon() -> Self {
        F64::new(f64::epsilon())
    }
    fn exp(self) -> Self {
        F64 {
            x: self.x.exp(),
            dx: self.dx * self.x.exp(),
        }
    }
    fn exp2(self) -> Self {
        F64 {
            x: self.x.exp2(),
            dx: self.dx * self.x.exp2() * f64::ln(2.0),
        }
    }
    fn exp_m1(self) -> Self {
        F64 {
            x: self.x.exp_m1(),
            dx: self.dx * self.x.exp(),
        }
    }
    fn floor(self) -> Self {
        F64 {
            x: self.x.floor(),
            dx: self.dx,
        }
    }
    fn fract(self) -> Self {
        F64 {
            x: self.x.fract(),
            dx: self.dx,
        }
    }
    fn hypot(self, other: Self) -> Self {
        F64 {
            x: self.x.hypot(other.x),
            dx: (self.dx * other.x + self.x * other.dx) / self.x.hypot(other.x),
        }
    }
    fn is_infinite(self) -> bool {
        self.x.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.x.is_finite()
    }
    fn is_normal(self) -> bool {
        self.x.is_normal()
    }
    fn is_sign_negative(self) -> bool {
        self.x.is_sign_negative()
    }
    fn is_sign_positive(self) -> bool {
        self.x.is_sign_positive()
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        self.x.integer_decode()
    }
    fn ln(self) -> Self {
        F64 {
            x: self.x.ln(),
            dx: self.dx / self.x,
        }
    }
    fn log(self, base: Self) -> Self {
        F64 {
            x: self.x.log(base.x),
            dx: self.dx / (self.x * base.x.ln()),
        }
    }
    fn log10(self) -> Self {
        F64 {
            x: self.x.log10(),
            dx: self.dx / (self.x * f64::ln(10.0)),
        }
    }
    fn log2(self) -> Self {
        F64 {
            x: self.x.log2(),
            dx: self.dx / (self.x * f64::ln(2.0)),
        }
    }
    fn max(self, other: Self) -> Self {
        F64 {
            x: self.x.max(other.x),
            dx: if self.x > other.x { self.dx } else { other.dx },
        }
    }
    fn ln_1p(self) -> Self {
        F64 {
            x: self.x.ln_1p(),
            dx: self.dx / (1.0 + self.x),
        }
    }
    fn min(self, other: Self) -> Self {
        F64 {
            x: self.x.min(other.x),
            dx: if self.x < other.x { self.dx } else { other.dx },
        }
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        F64 {
            x: self.x.mul_add(a.x, b.x),
            dx: self.dx * a.x + self.x * a.dx + b.dx,
        }
    }
    fn max_value() -> Self {
        F64::new(f64::max_value())
    }
    fn min_positive_value() -> Self {
        F64::new(f64::min_positive_value())
    }
    fn powf(self, n: Self) -> Self {
        F64 {
            x: self.x.powf(n.x),
            dx: self.dx * n.x * self.x.powf(n.x - 1.0),
        }
    }
    fn powi(self, n: i32) -> Self {
        F64 {
            x: self.x.powi(n),
            dx: self.dx * n as f64 * self.x.powi(n - 1),
        }
    }
    fn recip(self) -> Self {
        F64 {
            x: self.x.recip(),
            dx: -self.dx * self.x.recip().powi(2),
        }
    }
    fn round(self) -> Self {
        F64 {
            x: self.x.round(),
            dx: self.dx,
        }
    }
    fn sin(self) -> Self {
        F64 {
            x: self.x.sin(),
            dx: self.dx * self.x.cos(),
        }
    }
    fn sinh(self) -> Self {
        F64 {
            x: self.x.sinh(),
            dx: self.dx * self.x.cosh(),
        }
    }
    fn sqrt(self) -> Self {
        F64 {
            x: self.x.sqrt(),
            dx: self.dx / (2.0 * self.x.sqrt()),
        }
    }
    fn tan(self) -> Self {
        F64 {
            x: self.x.tan(),
            dx: self.dx / self.x.cos().powi(2),
        }
    }
    fn tanh(self) -> Self {
        F64 {
            x: self.x.tanh(),
            dx: self.dx / self.x.cosh().powi(2),
        }
    }
    fn to_degrees(self) -> Self {
        F64 {
            x: self.x.to_degrees(),
            dx: self.dx.to_degrees(),
        }
    }
    fn trunc(self) -> Self {
        F64 {
            x: self.x.trunc(),
            dx: self.dx,
        }
    }
    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }
}
