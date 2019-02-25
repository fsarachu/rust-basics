pub fn run() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("X: {}", x);
    }

    let mut y = 1;

    loop {
        y *= 2;

        if y > 100 {
            break;
        }

        println!("Y: {}", y);
    }
}