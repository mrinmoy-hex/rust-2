use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 2, 3, 3, 4, 5, 3];

    // Sort the numbers
    numbers.sort();
    println!("Sorted numbers: {:?}", numbers);

    let num_len = numbers.len();

    // Handle an empty vector
    if num_len == 0 {
        println!("The vector is empty.");
        return;
    }

    // Calculate the median
    let median: f64;

    if num_len % 2 == 0 {
        let lower = numbers[num_len / 2 - 1];
        let upper = numbers[num_len / 2];

        median = (lower as f64 + upper as f64) / 2.0;
    } else {
        median = numbers[num_len / 2] as f64;
    }

    println!("Median = {}", median);

    // Count the frequency of each number
    let mut frequencies = HashMap::new();

    for number in &numbers {
        *frequencies.entry(*number).or_insert(0) += 1;
    }

    println!("Frequencies: {:#?}", frequencies);

    // Find the mode
    let mut mode = numbers[0];
    let mut max_count = 0;

    for (number, count) in &frequencies {
        if *count > max_count {
            max_count = *count;
            mode = *number;
        }
    }

    println!("Mode = {}", mode);
    println!("Frequency = {}", max_count);
}
