use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn run() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes (64 bits, machine arch)

    let p3 = *p2; // Unboxing

    println!("p3 takes up {} bytes", mem::size_of_val(&p3)); // 16 bytes
    println!("p3.x: {}", p3.x);
}
