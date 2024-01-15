fn main() {
    let config_max = Some(3u8);
    match config_max {
       Some(max) => println!("The maximum is configured to be {}", max),
       _ => (),
    }

    // if config_max let Some() return a variable called max
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let names = ["Beau", "Rich", "Mary"];

    // match on the return value of a function, an Option type
    // the function returns an Option type
    // Option type variants are Some() and None
    // match returns a string
    for name in names {
        println!("{}'s occupation is {}", name,
        match get_occupation(&name) {
            Some(occupation) => occupation,  // if some value return a varible called occupation
            None => "no occupation"
        })
    }
}

// match on the &str ref to a string literal return Some() or None
// match returns a Option<&str>, Option variant with string literal data
fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Beau" => Some("Dog"),
        "Rich" => Some("Software dev"),
        _ => None
    }
}

