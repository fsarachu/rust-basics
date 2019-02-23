use std::mem;
/* 01 - Types and Variables */

fn fundamental_data_types() {
    // u - unsigned - 0...255
    // 8  - 8 bits
    let a: u8 = 123;
    println!("a: {}", a);

    // Following won't work because bindings are immutable by default
    // a = 423;

    // i - integer
    // 8  - 8 bits
    let mut b: i8 = -123;
    println!("b: {}", b);
    b = 0;
    println!("b: {}", b);

    // Can omit amount of memory/type
    let c = 123456789;
    println!("c: {}, size: {} bytes", c, mem::size_of_val(&c));

    // Chars
    let d: char = 'D';
    println!("d: {}, size: {} bytes", d, mem::size_of_val(&d));

    // Floats
    let e = 2.5; // double-precision, 64 bit, same as specifying 'f64'
    println!("e: {}, size: {} bytes", e, mem::size_of_val(&e));

    // Bool
    let g = false;
    let h = true;
    println!("g: {}, size: {} bytes", g, mem::size_of_val(&g));
    println!("h: {}, size: {} bytes", h, mem::size_of_val(&h));
}

fn operators() {
    // Arithmetic
    let mut a = 2 - 3 * 4;
    println!("a: {}", a);
    a = a + 1; // No -- and ++ operators
    println!("a: {}", a);
    a += 1; // But we got those += -= *= /= %=
    println!("a: {}", a);

    // No power operator, use built in functions
    let a_cubed = i32::pow(a, 3); // should use f64 or any corresponding type for 'a'
    println!("a_cubed: {}", a_cubed);

    // Bitwise
    let b = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
    println!("1|2: {}", b); // 3

    // Logical
    // < <= == >= >
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("PI < 4.0: {}", pi_less_4); // true
}

// Constants
const THE_CONST:u8 = 8; // No fixed address, occurrences are replaced at compilation time

fn main() {
    //fundamental_data_types();
    //operators();
    println!("Constant: {}", THE_CONST);
}
