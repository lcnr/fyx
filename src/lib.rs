#![feature(min_const_generics)]
use core::ops::{Add, Div, Mul, Shl, Shr, Sub};

pub trait Integer:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Sized
{
}

macro_rules! integer {
    ($s:ty, $b: ty $(, $t:ty)*) => {
        impl Integer for $s {}

        impl Widen for $s {
            type Assoc = $b;

            #[inline]
            fn widen(self) -> $b {
                self as $b
            }
        }

        impl Shrink for $b {
            type Assoc = $s;

            #[inline]
            fn shrink(self) -> $s {
                // FIXME: Use fallible conversion here.
                self as $s
            }
        }
        integer!($b $(, $t)*);
    };
    ($s:ty) => {
        impl Integer for $s {}
    };
}

integer!(u8, u16, u32, u64, u128);
integer!(i8, i16, i32, i64, i128);

pub trait Widen: Integer {
    type Assoc: Integer + Shrink<Assoc = Self>;

    fn widen(self) -> Self::Assoc;
}

pub trait Shrink: Integer {
    type Assoc: Integer + Widen<Assoc = Self>;

    fn shrink(self) -> Self::Assoc;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Fyx<T: Integer, const Q: usize>(T);

impl<T: Integer, const Q: usize> Fyx<T, Q> {
    #[inline]
    pub fn new(v: T) -> Fyx<T, Q> {
        Fyx(v << Q)
    }
}

impl<T: Integer, const Q: usize> Add for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn add(self, other: Fyx<T, Q>) -> Fyx<T, Q> {
        Fyx(self.0 + other.0)
    }
}

impl<T: Integer, const Q: usize> Add<T> for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn add(self, other: T) -> Fyx<T, Q> {
        self + Fyx::new(other)
    }
}

impl<T: Integer, const Q: usize> Sub for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn sub(self, other: Fyx<T, Q>) -> Fyx<T, Q> {
        Fyx(self.0 - other.0)
    }
}

impl<T: Integer, const Q: usize> Sub<T> for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn sub(self, other: T) -> Fyx<T, Q> {
        self - Fyx::new(other)
    }
}

impl<T: Integer + Widen, const Q: usize> Mul for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn mul(self, other: Fyx<T, Q>) -> Fyx<T, Q> {
        Fyx((self.0.widen() * other.0.widen() >> Q * 2).shrink())
    }
}

impl<T: Integer + Widen, const Q: usize> Mul<T> for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn mul(self, other: T) -> Fyx<T, Q> {
        self * Fyx::new(other)
    }
}

impl<T: Integer + Widen, const Q: usize> Div for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn div(self, other: Fyx<T, Q>) -> Fyx<T, Q> {
        Fyx(((self.0.widen() << Q) / (other.0.widen() << Q)).shrink())
    }
}

impl<T: Integer + Widen, const Q: usize> Div<T> for Fyx<T, Q> {
    type Output = Fyx<T, Q>;

    #[inline]
    fn div(self, other: T) -> Fyx<T, Q> {
        self / Fyx::new(other)
    }
}
