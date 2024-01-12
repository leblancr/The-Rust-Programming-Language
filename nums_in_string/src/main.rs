// concat the digits until non-digit found

struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let txt1 = String::from("abc1def12ghi123jkl");
    let mut num_str = String::from("");
    let mut sum = 0;
    
    for char in txt1.chars() {
        if char.is_digit(10) {
            num_str.push(char);  // concat the digits until non-digit found
        } else {
            if !num_str.is_empty() {
                // Attempt to parse the num_str string into an integer
                match num_str.parse::<i32>() {
                    Ok(parsed_num) => {
                        // Parsing was successful
                        sum += parsed_num;
                        println!("sum: {}", sum);
                    }
                    Err(e) => {
                        // Parsing failed
                        println!("Failed to parse: {}", e);
                    }
                }
                num_str.clear()
            }
        }
    }
}

