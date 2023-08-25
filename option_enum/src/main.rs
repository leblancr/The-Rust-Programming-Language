fn main() {
    let names = ["Beau", "Rich"];
    
    for name in names {
        println!("{}'s occupation is {}", name,
        match get_occupation(&name) {
            Some(occupation) => occupation,
            None => "no occupation"
        })
    }
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Beau" => Some("Dog"),
        "Rich" => Some("Software dev"),
        _ => None
    }
}