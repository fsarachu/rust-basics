const THE_CONST:u8 = 8; // No fixed address, occurrences are replaced at compilation time

pub fn run() {
    println!("Constant: {}", THE_CONST);
}