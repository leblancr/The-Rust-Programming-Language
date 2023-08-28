use common_collections::{run_vector_code, run_string_code, run_hash_map_code};

mod exercizes;

fn main() {
    //run_vector_code();
    //run_string_code();
    // run_hash_map_code()

    // exercises
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful
    // here) of the list. If even number of items avg of middle two.
    
    // Get list of integers
    let count = 11; // Number of random numbers
    let min = 1;    // Minimum value
    let max = 100;  // Maximum value

    let integers = exercizes::get_list_of_integers(count, min, max);
    //integers.sort();
    println!("List of integers: {:?}", integers);
    
    // get median
    let median = exercizes::get_median(&integers);
    println!("median: {:?}", median);
    
    // get mode
    let mode = exercizes::get_mode(&integers);
    println!("mode: {:?}", mode);
}
