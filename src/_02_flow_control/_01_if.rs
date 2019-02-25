pub fn run(temperature:i32) {
    if temperature > 25 {
        println!("It's hot!");
    } else if temperature < 14 {
        println!("It's cold!");
    } else {
        println!("It's a nice day!");
    }

    let top_wear = if temperature > 20 {"t-shirt"} else { "hoodie" };
    println!("Top wear: {}", top_wear);
}