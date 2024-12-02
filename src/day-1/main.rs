use crate::InputType;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(data_type: InputType) -> String {
    println!("Day 1 part 1");
    let input_path = match data_type {
        InputType::Example => "src/day-1/example.txt",
        InputType::Real => "src/day-1/data.txt",
    };

    let file = File::open(input_path).unwrap();
    // Customize buffer size if needed (default is 8KB)
    let mut reader = BufReader::with_capacity(32 * 1024, file);
    let mut line = String::with_capacity(1024); // Pre-allocate line buffer

    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];

    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        // Process line here
        let entries: Vec<&str> = line.split_whitespace().collect();
        let left: i32 = entries[0].parse().unwrap();
        left_numbers.push(left);
        let right: i32 = entries[1].parse().unwrap();
        right_numbers.push(right);

        line.clear(); // Reuse the allocated String
    }
    
    left_numbers.sort();
    right_numbers.sort();

    assert!(left_numbers.len() == right_numbers.len());

    let mut result: i32 = 0;

    for (index, left) in left_numbers.iter().enumerate() {
        let right = right_numbers[index];
        let diff = left - right;
        result += diff.abs();
    }

    return result.to_string()
}

pub fn part2(data_type: InputType) -> String {
    println!("Day 1 part 2");
    let input_path = match data_type {
        InputType::Example => "src/day-1/example.txt",
        InputType::Real => "src/day-1/data.txt",
    };

    let file = File::open(input_path).unwrap();
    // Customize buffer size if needed (default is 8KB)
    let mut reader = BufReader::with_capacity(32 * 1024, file);
    let mut line = String::with_capacity(1024); // Pre-allocate line buffer

    let mut left_numbers: Vec<i32> = vec![];
    let mut right_occurances: HashMap<i32, i32> = HashMap::new();
    
    let mut result: i32 = 0;
    
    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        // Process line here
        let entries: Vec<&str> = line.split_whitespace().collect();
        let left: i32 = entries[0].parse().unwrap();
        left_numbers.push(left);

        let right: i32 = entries[1].parse().unwrap();
        right_occurances.entry(right).and_modify(|occurance| *occurance += 1).or_insert(1);

        line.clear();
    }

    for left in left_numbers {
        result += left * *right_occurances.entry(left).or_insert(0);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(InputType::Example), "11");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(InputType::Example), "31");
    }
}