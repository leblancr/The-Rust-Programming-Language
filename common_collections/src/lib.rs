use rand::Rng;

mod common_collections {
    pub mod vectors {
        pub fn vector_code() {
            let mut v = vec![100, 32, 57];
            println!("{:?}", v);

            for i in &mut v {
                *i += 50;
            }
            println!("{:?}", v);

            #[derive(Debug)]
            enum SpreadsheetCell {
                Int(i32),
                Float(f64),
                Text(String),
            }

            // vector with different types by using enum type
            let v = vec![
                SpreadsheetCell::Int(3),
                SpreadsheetCell::Text(String::from("blue")),
                SpreadsheetCell::Float(10.12),
            ];
            println!("{:?}", v);
        }
    }

    pub mod strings {
        pub fn string_code() {
            let mut s1 = String::from("foo");
            let s2 = "bar";
            s1.push_str(s2);
            println!("s2 is {s2}");
            
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");
            let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
            println!("s2 is {s2}");
            println!("s3 is {s3}");
            
            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            let s = format!("{s1}-{s2}-{s3}");
            println!("s is {s}");
        }
    }

    pub mod hash_maps {
        use std::collections::HashMap;
        
         pub fn hash_map_code() {
             let mut scores = HashMap::new();
        
             scores.insert(String::from("Blue"), 10);

             scores.entry(String::from("Yellow")).or_insert(50);
             scores.entry(String::from("Blue")).or_insert(50);
        
             println!("{:?}", scores);
             
             let text = "hello world wonderful world";
             let mut map = HashMap::new();

             println!("{:?}", text.split_whitespace());
             
             for word in text.split_whitespace() {
                 let count = map.entry(word).or_insert(0);  // count is a ref to the value
                 *count += 1;  // increment data in count
             }

             println!("{:?}", map);
        }
    }
}

pub fn run_vector_code() {
    // Absolute path
    // crate::common_collections::vectors::vector_code();

    // Relative path
    common_collections::vectors::vector_code();
}

pub fn run_string_code() {
    // Absolute path
    // crate::common_collections::strings::string_code();

    // Relative path
    common_collections::strings::string_code();
}

pub fn run_hash_map_code() {
    // Absolute path
    // crate::common_collections::hash_maps::hash_map_code();

    // Relative path
    common_collections::hash_maps::hash_map_code();
}

// generate list of random numbers that appear random number of times
pub fn generate_random_numbers(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng(); // Create a random number generator for the current thread
    let mut numbers = Vec::new();

    for _ in 0..count {
        let random_number = rng.gen_range(min..=max);
        let random_inner_number = rng.gen_range(0..max);
        println!("{}", random_inner_number);
        for _ in 0..random_inner_number {
            numbers.push(random_number);
        }
    }

    numbers
}