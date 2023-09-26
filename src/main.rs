// #![feature(generic_const_exprs)]
// #![allow(incomplete_features)]

#![recursion_limit = "65535"]

mod u48;
mod varuint;

use u48::U48;
use varuint::VarUInt;

fn main() {
    let value: U48 = 123u32.into();
    let mut value = value + (U48::MAX - U48::from(3u8));
    value += 1u8.into();
    println!("value: {value}");
    value <<= 1;
    println!("value: {value}");

    let value: VarUInt<48> = 123u32.into();
    let x: u32 = (VarUInt::<32>::from(0xFFu8) << 31).into();
    println!("x: {x}");
    let mut value = value + (VarUInt::<48>::MAX - VarUInt::<31>::from(3u8).into());
    value += 1u8.into();
    println!("value: {value}");
    value <<= 1;
    println!("value: {value}");
}
