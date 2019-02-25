pub fn run() {
    let country_code = 598;
    let country = match country_code {
        44 => "United Kingdom",
        7 => "Rusia",
        598 => "Uruguay",
        1...999 => "Unknown", // Weird range matcher
        _ => "Invalid" // 'Default' or 'catch all'
    };

    println!("{}: {}", country_code, country);
}