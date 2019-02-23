pub fn run() {
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