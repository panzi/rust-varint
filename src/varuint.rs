use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign,
    Sub, SubAssign,
    BitOr, BitOrAssign, BitXor, BitXorAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Not,
    Rem, RemAssign,
    Shl, ShlAssign, Shr, ShrAssign,
};

use std::fmt::{
    Display,
    Debug,
};

use core::convert::{
    From,
    TryFrom,
    Into,
    TryInto,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    ValueTooBig,
}

pub trait UnsignedInteger:
    Sized + Copy + Clone + PartialEq + PartialOrd +
    Display + Debug +
    From<u8> +
    Add + AddAssign + BitAnd + BitAndAssign +
    Sub + SubAssign +
    BitOr + BitOrAssign + BitXor + BitXorAssign +
    Div + DivAssign +
    Mul + MulAssign +
    Not +
    Rem + RemAssign +
    Shl + ShlAssign + Shr + ShrAssign
{}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}

pub trait Intern {
    type UInt: UnsignedInteger;
    const MIN:  Self::UInt;
    const MAX:  Self::UInt;
    const ZERO: Self::UInt;
    const ONE:  Self::UInt;
    const BITS: u32;
}

pub struct InternHelper<const N: u8> {}

macro_rules! make_minint {
    (@impl_minint $type:ident, [ $($from_types:ident),* ], [ $($try_from_types:ident),* ], [ $($into_types:ident),* ], $bits:literal) => {
        impl Intern for InternHelper<$bits> {
            type UInt = $type;
            const MIN:  Self::UInt = $type::MIN;
            const MAX:  Self::UInt = $type::MAX >> ($type::BITS - $bits);
            const ZERO: Self::UInt = 0;
            const ONE:  Self::UInt = 1;
            const BITS: u32 = $bits;
        }

        make_minint!(@from $bits, $($from_types),*);
        make_minint!(@try_from $bits, $($try_from_types),*);
        make_minint!(@into $bits, $($into_types),*);

        impl Add for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self { value: (self.value + rhs.value) & Self::MAX.value }
            }
        }

        impl AddAssign for VarUInt<$bits> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.value = (self.value + rhs.value) & Self::MAX.value;
            }
        }

        impl Sub for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self { value: (self.value - rhs.value) & Self::MAX.value }
            }
        }

        impl SubAssign for VarUInt<$bits> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.value = (self.value - rhs.value) & Self::MAX.value;
            }
        }

        impl BitAnd for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self { value: self.value & rhs.value }
            }
        }

        impl BitAndAssign for VarUInt<$bits> {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.value &= rhs.value;
            }
        }

        impl BitOr for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self { value: self.value | rhs.value }
            }
        }

        impl BitOrAssign for VarUInt<$bits> {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.value |= rhs.value;
            }
        }

        impl BitXor for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self { value: self.value ^ rhs.value }
            }
        }

        impl BitXorAssign for VarUInt<$bits> {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.value ^= rhs.value;
            }
        }

        impl Not for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                Self { value: !self.value & Self::MAX.value }
            }
        }

        impl Mul for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                Self { value: (self.value * rhs.value) & Self::MAX.value }
            }
        }

        impl MulAssign for VarUInt<$bits> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.value = (self.value * rhs.value) & Self::MAX.value;
            }
        }

        impl Div for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                Self { value: self.value / rhs.value }
            }
        }

        impl DivAssign for VarUInt<$bits> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.value /= rhs.value;
            }
        }

        impl Rem for VarUInt<$bits> {
            type Output = Self;
        
            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                Self { value: self.value % rhs.value }
            }
        }

        impl RemAssign for VarUInt<$bits> {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.value %= rhs.value;
            }
        }

        impl Shl for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: Self) -> Self::Output {
                Self { value: (self.value << rhs.value) & Self::MAX.value }
            }
        }

        impl ShlAssign for VarUInt<$bits> {
            #[inline]
            fn shl_assign(&mut self, rhs: Self) {
                self.value = (self.value << rhs.value) & Self::MAX.value;
            }
        }

        impl Shr for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: Self) -> Self::Output {
                Self { value: self.value >> rhs.value }
            }
        }

        impl ShrAssign for VarUInt<$bits> {
            #[inline]
            fn shr_assign(&mut self, rhs: Self) {
                self.value >>= rhs.value;
            }
        }
    };

    (@minint $type:ident, [ $($from_types:ident),* ], [ $($try_from_types:ident),* ], [ $($into_types:ident),* ] $(,)?) => {};

    (@minint $type:ident, [ $($from_types:ident),* ], [ $($try_from_types:ident),* ], [ $($into_types:ident),* ], $bits:literal $(,)?) => {
        make_minint!(@impl_minint $type, [ $($from_types),* ], [ $($try_from_types),* ], [ $($into_types),* ], $bits);
    };

    (@minint $type:ident, [ $($from_types:ident),* ], [ $($try_from_types:ident),* ], [ $($into_types:ident),* ], $bits:literal, $($more:literal),+) => {
        make_minint!(@impl_minint $type, [ $($from_types),* ], [ $($try_from_types),* ], [ $($into_types),* ], $bits);
        make_minint!(@minint $type, [ $($from_types),* ], [ $($try_from_types),* ], [ $($into_types),* ], $($more),+);
    };

    // ==== from ===============================================================
    (@impl_from $bits:literal, $type:ident) => {
        impl From<$type> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn from(value: $type) -> Self {
                VarUInt { value: value.into() }
            }
        }

        /*
        impl TryInto<$type> for VarUInt<$bits> {
            type Error = Error;

            #[inline]
            fn try_into(self) -> Result<$type, Self::Error> {
                if self.value > $type::MAX.into() {
                    return Err(Error::ValueTooBig);
                }

                return Ok(self.value as $type);
            }
        }
         */
    };

    (@from $bits:literal $(,)?) => {};

    (@from $bits:literal, $type:ident $(,)?) => {
        make_minint!(@impl_from $bits, $type);
    };

    (@from $bits:literal, $type:ident, $($more:ident),*) => {
        make_minint!(@impl_from $bits, $type);
        make_minint!(@from $bits, $($more),*);
    };

    // ==== try from ===========================================================
    (@impl_try_from $bits:literal, $type:ident) => {
        impl TryFrom<$type> for VarUInt<$bits> {
            type Error = Error;

            #[inline]
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if let Ok(value) = value.try_into() {
                    if value > Self::MAX.value {
                        return Err(Error::ValueTooBig);
                    }
                    return Ok(VarUInt { value });
                }
                return Err(Error::ValueTooBig);
            }
        }
    };

    (@try_from $bits:literal $(,)?) => {};

    (@try_from $bits:literal, $type:ident $(,)?) => {
        make_minint!(@impl_try_from $bits, $type);
    };

    (@try_from $bits:literal, $type:ident, $($more:ident),*) => {
        make_minint!(@impl_try_from $bits, $type);
        make_minint!(@try_from $bits, $($more),*);
    };

    // ==== into ===============================================================
    (@impl_into $bits:literal, $type:ident) => {
        impl Into<$type> for VarUInt<$bits> {
            #[inline]
            fn into(self) -> $type {
                self.value as $type
            }
        }

        /*
        impl TryFrom<$type> for VarUInt<$bits> {
            type Error = Error;

            #[inline]
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if value > Self::MAX {
                    return Err(Error::ValueTooBig);
                }

                return Ok(VarUInt { value });
            }
        }
        */
    };

    (@into $bits:literal $(,)?) => {};

    (@into $bits:literal, $type:ident $(,)?) => {
        make_minint!(@impl_into $bits, $type);
    };

    (@into $bits:literal, $type:ident, $($more:ident),*) => {
        make_minint!(@impl_into $bits, $type);
        make_minint!(@into $bits, $($more),*);
    };

    // ==== start ==============================================================
    ($type:ident, [ $($from_types:ident),* ], [ $($try_from_types:ident),* ], [ $($into_types:ident),* ], [ $($bits:literal),* ]) => {
        make_minint!(@minint $type, [ $($from_types),* ], [ $($try_from_types),* ], [ $($into_types),* ], $($bits),*);
    };
}

make_minint!(u8, [], [u8, u16, u32, u64, u128], [u8, u16, u32, u64, u128],
    [1, 2, 3, 4, 6, 7]);
make_minint!(u8, [u8], [u16, u32, u64, u128], [u8, u16, u32, u64, u128], [8]);

make_minint!(u16, [u8], [u16, u32, u64, u128], [u16, u32, u64, u128],
    [9, 10, 11, 12, 13, 14, 15]);
make_minint!(u16, [u8, u16], [u32, u64, u128], [u16, u32, u64, u128], [16]);

make_minint!(u32, [u8, u16], [u32, u64, u128], [u32, u64, u128],
    [17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
make_minint!(u32, [u8, u16, u32], [u64, u128], [u32, u64, u128], [32]);

make_minint!(u64, [u8, u16, u32], [u64, u128], [u64, u128],
    [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
     49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]);
make_minint!(u64, [u8, u16, u32, u64], [u128], [u64, u128], [64]);

make_minint!(u128, [u8, u16, u32, u64], [], [u128],
    [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
     83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
     101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
     116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127]);
make_minint!(u128, [u8, u16, u32, u64, u128], [], [u128], [128]);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct VarUInt<const N: u8> where InternHelper::<{N}>: Intern {
    value: <InternHelper::<N> as Intern>::UInt
}

impl<const N: u8> VarUInt<N> where InternHelper::<{N}>: Intern {
    pub const MIN:  VarUInt<N> = VarUInt { value: InternHelper::<{N}>::MIN };
    pub const MAX:  VarUInt<N> = VarUInt { value: InternHelper::<{N}>::MAX };
    pub const ZERO: VarUInt<N> = VarUInt { value: InternHelper::<{N}>::ZERO };
    pub const ONE:  VarUInt<N> = VarUInt { value: InternHelper::<{N}>::ONE };
    pub const BITS: u32 = InternHelper::<{N}>::BITS;
}

impl<const N: u8> Default for VarUInt<N> where InternHelper::<{N}>: Intern {
    #[inline]
    fn default() -> Self {
        Self { value: InternHelper::<{N}>::ZERO }
    }
}

impl<const N: u8> Display for VarUInt<N> where InternHelper::<{N}>: Intern {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}
