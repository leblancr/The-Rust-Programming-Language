fn main() {
    let names = ["Beau", "Rich", "Mary"];

    // match on the return value of a function, an Option type
    // the function returns an Option type
    // Option type variants are Some() and None
    // match returns a string
    for name in names {
        println!("{}'s occupation is {}", name,
        match get_occupation(&name) {
            Some(occupation) => occupation,
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

//fn main() {
//    let x: i8 = 5;
//    let y: Option<i8> =
//}