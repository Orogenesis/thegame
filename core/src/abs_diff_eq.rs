/// Equality that is defined using the absolute difference of two numbers.
pub trait AbsDiffEq<Rhs = Self>: PartialEq<Rhs> where Rhs: ?Sized {
    /// Used for specifying relative comparisons.
    type Epsilon;

    /// A test for equality that uses the absolute difference to compute the approximate
    /// equality of two numbers.
    fn abs_diff_eq(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool;
}

macro_rules! impl_unsigned_abs_diff_eq {
    ($T:ident) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                (if self > other { self - other } else { other - self }) == epsilon
            }
        }
    };
}

impl_unsigned_abs_diff_eq!(u8);
