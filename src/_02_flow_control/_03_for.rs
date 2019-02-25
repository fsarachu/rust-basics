pub fn run() {
    for x in 1..10 {
        println!("X: {}", x);
    }

    for (pos, y) in (30..40).enumerate() {
        println!("pos: {}, y: {}", pos, y);
    }
}