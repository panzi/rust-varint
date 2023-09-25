// #![feature(generic_const_exprs)]
// #![allow(incomplete_features)]

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
    let x: u32 = VarUInt::<31>::from(3u8).into();
    let mut value = value + (VarUInt::<48>::MAX - x.into());
    value += 1u8.into();
    println!("value: {value}");
    value <<= 1;
    println!("value: {value}");
}
