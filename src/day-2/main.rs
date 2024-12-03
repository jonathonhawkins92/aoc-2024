use crate::InputType;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input_type: InputType) -> String {
    println!("Day 2 part 1");
    let input_path = match input_type {
        InputType::Example => "src/day-2/example.txt",
        InputType::Real => "src/day-2/data.txt",
    };

    let file = File::open(input_path).unwrap();
    // Customize buffer size if needed (default is 8KB)
    let mut reader = BufReader::with_capacity(32 * 1024, file);
    let mut line = String::with_capacity(1024); // Pre-allocate line buffer
    let mut result = 0;
    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        // Process line here
        let entries: Vec<&str> = line.split_whitespace().collect();
        // decrease = -1
        // unset = 0
        // increase = 1
        let mut direction = 0;
        let mut is_safe = true;
        println!("{}", entries.join(","));
        for (index, raw_value) in entries.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let last_raw_value = entries[index - 1];
            let value: i32 = raw_value.parse().unwrap();
            let last_value: i32 = last_raw_value.parse().unwrap();
            println!(
                "   last_value {}, value {}",
                last_value.to_string(),
                value.to_string(),
            );
            let diff = value - last_value;

            if direction == 0 {
                if diff < 0 {
                    direction = -1
                } else if diff > 0 {
                    direction = 1
                }
            }
            println!(
                "   direction {}, diff {}",
                direction.to_string(),
                diff.to_string()
            );

            if diff == 0 {
                is_safe = false;
                break;
            } else if direction == 1 && (diff < 1 || 3 < diff) {
                is_safe = false;
                break;
            } else if direction == -1 && (diff > -1 || -3 > diff) {
                is_safe = false;
                break;
            }
        }
        println!("  is safe? {}", is_safe);
        if is_safe {
            result += 1;
        }
        line.clear(); // Reuse the allocated String
    }

    result.to_string()
}

pub fn part2(input_type: InputType) -> String {
    println!("Day 2 part 2");
    let input_path = match input_type {
        InputType::Example => "src/day-2/example.txt",
        InputType::Real => "src/day-2/data.txt",
    };

    let file = File::open(input_path).unwrap();
    // Customize buffer size if needed (default is 8KB)
    let mut reader = BufReader::with_capacity(32 * 1024, file);
    let mut line = String::with_capacity(1024); // Pre-allocate line buffer
    let mut result = 0;
    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        // Process line here
        let entries: Vec<&str> = line.split_whitespace().collect();
        // decrease = -1
        // unset = 0
        // increase = 1
        let mut direction = 0;
        let mut is_safe = true;
        let mut has_direction_changed = false;
        println!("{}", entries.join(","));
        for (index, raw_value) in entries.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let last_raw_value = entries[index - 1];
            let value: i32 = raw_value.parse().unwrap();
            let last_value: i32 = last_raw_value.parse().unwrap();
            println!(
                "   last_value {}, value {}",
                last_value.to_string(),
                value.to_string(),
            );
            let diff = value - last_value;

            if direction == 0 {
                if diff < 0 {
                    direction = -1
                } else if diff > 0 {
                    direction = 1
                }
            }
            println!(
                "   direction {}, diff {}",
                direction.to_string(),
                diff.to_string()
            );

            if direction == 1 {
                if diff <= 0 {
                    if has_direction_changed == false {
                        has_direction_changed = true
                    } else {
                        is_safe = false;
                        break;
                    }
                } else if diff < 1 || 3 < diff {
                    is_safe = false;
                    break;
                }
            } else if direction == -1 {
                if diff >= 0 {
                    if has_direction_changed == false {
                        has_direction_changed = true
                    } else {
                        is_safe = false;
                        break;
                    }
                } else if diff > -1 || -3 > diff {
                    is_safe = false;
                    break;
                }
            } else if diff == 0 {
                is_safe = false;
                break;
            }
        }
        println!("  is safe? {}", is_safe);
        if is_safe {
            result += 1;
        }
        line.clear(); // Reuse the allocated String
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(InputType::Example), "2");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(InputType::Example), "4");
    }
}
