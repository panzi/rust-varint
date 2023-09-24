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

use std::fmt::Display;

use core::convert::{
    From,
    TryFrom,
    Into,
    TryInto,
};

// const MASK_U47: u64 = 0x7F_FF_FF_FF_FF;
const MASK_U48: u64 = 0xFF_FF_FF_FF_FF;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    ValueTooBig,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct U48 {
    value: u64
}

impl U48 {
    pub const MIN: U48 = U48 { value: 0 };
    pub const MAX: U48 = U48 { value: MASK_U48 };
}

impl Default for U48 {
    #[inline]
    fn default() -> Self {
        U48 { value: 0 }
    }
}

impl Display for U48 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl From<u8> for U48 {
    #[inline]
    fn from(value: u8) -> Self {
        U48 { value: value as u64 }
    }
}

impl From<u16> for U48 {
    #[inline]
    fn from(value: u16) -> Self {
        U48 { value: value as u64 }
    }
}

impl From<u32> for U48 {
    #[inline]
    fn from(value: u32) -> Self {
        U48 { value: value as u64 }
    }
}

impl TryFrom<u64> for U48 {
    type Error = Error;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > MASK_U48 {
            return Err(Error::ValueTooBig);
        }

        return Ok(U48 { value });
    }
}

impl TryInto<u8> for U48 {
    type Error = Error;

    #[inline]
    fn try_into(self) -> Result<u8, Self::Error> {
        if self.value > u8::MAX as u64 {
            return Err(Error::ValueTooBig);
        }

        return Ok(self.value as u8);
    }
}

impl TryInto<u16> for U48 {
    type Error = Error;

    #[inline]
    fn try_into(self) -> Result<u16, Self::Error> {
        if self.value > u16::MAX as u64 {
            return Err(Error::ValueTooBig);
        }

        return Ok(self.value as u16);
    }
}

impl TryInto<u32> for U48 {
    type Error = Error;

    #[inline]
    fn try_into(self) -> Result<u32, Self::Error> {
        if self.value > u32::MAX as u64 {
            return Err(Error::ValueTooBig);
        }

        return Ok(self.value as u32);
    }
}

impl Into<u64> for U48 {
    #[inline]
    fn into(self) -> u64 {
        self.value
    }
}

impl Add for U48 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        U48 { value: (self.value + rhs.value) & MASK_U48 }
    }
}

impl AddAssign for U48 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) & MASK_U48;
    }
}

impl Sub for U48 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        U48 { value: (self.value - rhs.value) & MASK_U48 }
    }
}

impl SubAssign for U48 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value - rhs.value) & MASK_U48;
    }
}

impl BitAnd for U48 {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        U48 { value: self.value & rhs.value }
    }
}

impl BitAndAssign for U48 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr for U48 {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        U48 { value: self.value | rhs.value }
    }
}

impl BitOrAssign for U48 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor for U48 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        U48 { value: self.value ^ rhs.value }
    }
}

impl BitXorAssign for U48 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl Not for U48 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        U48 { value: !self.value & MASK_U48 }
    }
}

impl Mul for U48 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        U48 { value: (self.value * rhs.value) & MASK_U48 }
    }
}

impl MulAssign for U48 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value) & MASK_U48;
    }
}

impl Div for U48 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        U48 { value: self.value / rhs.value }
    }
}

impl DivAssign for U48 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for U48 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        U48 { value: self.value % rhs.value }
    }
}

impl RemAssign for U48 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

impl Shl for U48 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        U48 { value: (self.value << rhs.value) & MASK_U48 }
    }
}

impl ShlAssign for U48 {
    #[inline]
    fn shl_assign(&mut self, rhs: Self) {
        self.value = (self.value << rhs.value) & MASK_U48;
    }
}

impl Shr for U48 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        U48 { value: self.value >> rhs.value }
    }
}

impl ShrAssign for U48 {
    #[inline]
    fn shr_assign(&mut self, rhs: Self) {
        self.value >>= rhs.value;
    }
}
