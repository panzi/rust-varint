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

use std::cmp::PartialEq;

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

macro_rules! impl_varuint {
    ////////////////////////////////////////////////////////////////////////////

    (@loop
        smaller_types: [ $($smaller_types:ident)* ],
        fewer_bits: [ $($fewer_bits:literal)* ],
        tail: []
    ) => {};

    (@loop
        smaller_types: [ $($smaller_types:ident)* ],
        fewer_bits: [ $($fewer_bits:literal)* ],
        tail: [ $type:ident [ $($bits:literal)* ], $($tail:tt)* ]
    ) => {
        impl_varuint!(@loop_impl $type, [ $($bits)* ],
            smaller_types: [ $($smaller_types)* ],
            fewer_bits: [ $($fewer_bits)* ]);

        impl_varuint!(@loop_into_bigger_types $type, [ $($fewer_bits)* $($bits)* ]);

        impl_varuint!(@loop
            smaller_types: [ $($smaller_types)* $type ],
            fewer_bits: [ $($fewer_bits)* $($bits)* ],
            tail: [ $($tail)* ]
        );
    };

    ////////////////////////////////////////////////////////////////////////////

    (@loop_into_bigger_types $type:ident, []) => {};

    (@loop_into_bigger_types $type:ident, [ $bits:literal $($more_bits:literal)* ]) => {
        impl Into<$type> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn into(self: Self) -> $type {
                self.value.into()
            }
        }

        impl_varuint!(@loop_into_bigger_types $type, [ $($more_bits)* ]);
    };

    ////////////////////////////////////////////////////////////////////////////

    (@loop_impl $type:ident, [],
        smaller_types: [ $($smaller_types:ident)* ],
        fewer_bits: [ $($fewer_bits:literal)* ]
    ) => {};

    (@loop_impl $type:ident, [ $bits:literal ],
        smaller_types: [ $($smaller_types:ident)* ],
        fewer_bits: [ $($fewer_bits:literal)* ]
    ) => {
        impl_varuint!(@impl $type, $bits);
        impl_varuint!(@from_type $type, $bits, $type);
        impl_varuint!(@loop_smaller_types $type, $bits, [ $($smaller_types)* ]);
        impl_varuint!(@loop_fewer_bits $type, $bits, [ $($fewer_bits)* ]);
    };

    (@loop_impl $type:ident, [ $bits:literal $($more_bits:literal)+ ],
        smaller_types: [ $($smaller_types:ident)* ],
        fewer_bits: [ $($fewer_bits:literal)* ]
    ) => {
        impl_varuint!(@impl $type, $bits);
        impl_varuint!(@try_from_type $type, $bits);
        impl_varuint!(@loop_smaller_types $type, $bits, [ $($smaller_types)* ]);
        impl_varuint!(@loop_fewer_bits $type, $bits, [ $($fewer_bits)* ]);

        impl_varuint!(@loop_impl $type, [ $($more_bits)* ],
            smaller_types: [ $($smaller_types)* ],
            fewer_bits: [ $($fewer_bits)* $bits ]);
    };

    ////////////////////////////////////////////////////////////////////////////

    (@loop_smaller_types $type:ident, $bits:literal, []) => {};

    (@loop_smaller_types $type:ident, $bits:literal, [ $smaller_type:ident $($smaller_types:ident)* ]) => {
        impl_varuint!(@from_type $type, $bits, $smaller_type);
        impl_varuint!(@try_into_type $type, $bits, $smaller_type);

        impl_varuint!(@loop_smaller_types $type, $bits, [ $($smaller_types)* ]);
    };

    ////////////////////////////////////////////////////////////////////////////

    (@loop_fewer_bits $type:ident, $bits:literal, []) => {};

    (@loop_fewer_bits $type:ident, $bits:literal, [ $fewer_bits:literal $($more_fewer_bits:literal)* ]) => {
        impl_varuint!(@from_bits $type, $bits, $fewer_bits);

        impl_varuint!(@loop_fewer_bits $type, $bits, [ $($more_fewer_bits)* ]);
    };

    ////////////////////////////////////////////////////////////////////////////

    (@impl $type:ident, $bits:literal) => {
        impl Intern for InternHelper<$bits> {
            type UInt = $type;
            const MIN:  Self::UInt = $type::MIN;
            const MAX:  Self::UInt = $type::MAX >> ($type::BITS - $bits);
            const ZERO: Self::UInt = 0;
            const ONE:  Self::UInt = 1;
            const BITS: u32 = $bits;
        }

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

        impl Shl<usize> for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: usize) -> Self::Output {
                Self { value: (self.value << rhs) & Self::MAX.value }
            }
        }

        impl ShlAssign<usize> for VarUInt<$bits> {
            #[inline]
            fn shl_assign(&mut self, rhs: usize) {
                self.value = (self.value << rhs) & Self::MAX.value;
            }
        }

        impl Shr<usize> for VarUInt<$bits> {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: usize) -> Self::Output {
                Self { value: self.value >> rhs }
            }
        }

        impl ShrAssign<usize> for VarUInt<$bits> {
            #[inline]
            fn shr_assign(&mut self, rhs: usize) {
                self.value >>= rhs;
            }
        }
    };

    (@from_type $type:ident, $bits:literal, $smaller_type:ident) => {
        impl From<$smaller_type> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn from(value: $smaller_type) -> Self {
                VarUInt { value: value.into() }
            }
        }

        impl PartialEq<$smaller_type> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn eq(&self, rhs: &$smaller_type) -> bool {
                let rhs: <InternHelper::<$bits> as Intern>::UInt = (*rhs).into();
                self.value == rhs
            }
        }

        impl PartialEq<VarUInt<$bits>> for $smaller_type
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn eq(&self, rhs: &VarUInt<$bits>) -> bool {
                let lhs: <InternHelper::<$bits> as Intern>::UInt = (*self).into();
                lhs == rhs.value
            }
        }
    };

    (@from_bits $type:ident, $bits:literal, $other_bits:literal) => {
        impl From<VarUInt<$other_bits>> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            #[inline]
            fn from(other: VarUInt<$other_bits>) -> Self {
                VarUInt { value: other.value.into() }
            }
        }
    };

    (@try_from_type $type:ident, $bits:literal) => {
        impl TryFrom<$type> for VarUInt<$bits>
        where InternHelper::<{$bits}>: Intern {
            type Error = Error;

            #[inline]
            fn try_from(value: $type) -> Result<VarUInt<$bits>, Self::Error> {
                if let Ok(value) = value.try_into() {
                    if value > InternHelper::<{$bits}>::MAX {
                        return Err(Error::ValueTooBig);
                    }
                    return Ok(VarUInt { value });
                }
                return Err(Error::ValueTooBig);
            }
        }
    };

    (@try_into_type $type:ident, $bits:literal, $smaller_type:ident) => {
        impl TryInto<$smaller_type> for VarUInt<$bits> {
            type Error = Error;

            #[inline]
            fn try_into(self) -> Result<$smaller_type, Self::Error> {
                if let Ok(value) = self.value.try_into() {
                    return Ok(value);
                }
                return Err(Error::ValueTooBig);
            }
        }
    };

    ////////////////////////////////////////////////////////////////////////////

    () => {};

    ($type:ident [ $($bits:literal)* ] $($tail:tt)*) => {
        impl_varuint!(@loop smaller_types: [], fewer_bits: [], tail: [ $type [ $($bits)* ] $($tail)* ]);
    };
}

impl_varuint! {
    u8   [1 2 3 4 5 6 7 8],
    u16  [9 10 11 12 13 14 15 16],
    u32  [17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32],
    u64  [33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64],
    u128 [65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128],
}

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
