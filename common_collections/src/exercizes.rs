use common_collections::generate_random_numbers;

pub fn get_list_of_integers(count: usize, min: i32, max: i32) -> Vec<i32> {
    let random_numbers = generate_random_numbers(count, min, max);
    // println!("Random numbers: {:?}", random_numbers);
    random_numbers
}

pub fn get_median(numbers: &Vec<i32>) -> i32 {
    // Step 1: Sort the list in ascending order
    let mut sorted_numbers = numbers.clone(); // Clone the original vector
    sorted_numbers.sort();

    // Step 2: Determine if the list has an odd or even number of elements
    let length = sorted_numbers.len();
    let middle = length / 2;

    // Step 3: Calculate the median based on the number of elements
    if length % 2 == 0 {
        // Even number of elements, take the average of the two middle values
        let mid1 = sorted_numbers[middle - 1] as i32;
        let mid2 = sorted_numbers[middle] as i32;
        (mid1 + mid2) / 2
    } else {
        // Odd number of elements, the middle element is the median
        sorted_numbers[middle] as i32
    }
}

// mode (the value that occurs most often;
// hash map will be helpful here) of the list.
pub fn get_mode(numbers: &Vec<i32>) -> i32 {
    numbers[0]
}
