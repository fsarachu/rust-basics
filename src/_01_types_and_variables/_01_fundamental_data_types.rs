use std::mem;

pub fn run() {
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
