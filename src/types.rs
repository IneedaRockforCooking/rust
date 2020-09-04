/*
pRIMITIVEC tYPES
Integeres: u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
            number ov bits they take for memory
Float f32 f64
boolean bool
character char
tuples
arrays
 */

pub fn run() {
    // Default is i32
    let x = 1;
    let y = 2.6;
    let j : i64 = 11;
    println!("x i32 1 {} y i64 2.6 {} j i64 11 {}",
             x,y,j);
    let u: i64 = 4545445454545;
    println!("u: i64 {}",u);
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i32::MAX);
    println!("Max i128: {}", std::i128::MAX);
}