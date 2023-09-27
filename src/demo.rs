#![recursion_limit = "65535"]

mod varuint;

use varuint::VarUInt;

fn main() {
    let value: VarUInt<48> = 123u32.into();
    let x: u32 = (VarUInt::<32>::from(0xFFu8) << 31).into();
    println!("x: {x} {}", x == 2147483648);
    let mut value = value + (VarUInt::<48>::MAX - VarUInt::<31>::from(3u8).into());
    value += 1u8.into();
    println!("value: {value} {}", value == 120u32);
    value <<= 1;
    println!("value: {value} {} {}", !(value != 240u16), <VarUInt<48> as Into<u64>>::into(value) == 240u64);
}
