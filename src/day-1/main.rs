use crate::InputType;
use std::fs;

pub fn part1(data_type: InputType) -> String {
    let input = match data_type {
        InputType::Example => fs::read_to_string("src/day-1/example.txt").unwrap(),
        InputType::Real => fs::read_to_string("src/day-1/data.txt").unwrap(),
    };

    input
}

pub fn part2(data_type: InputType) -> String {
    let input = match data_type {
        InputType::Example => fs::read_to_string("src/day-1/example.txt").unwrap(),
        InputType::Real => fs::read_to_string("src/day-1/data.txt").unwrap(),
    };

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(InputType::Example), "expected_result");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(InputType::Example), "expected_result");
    }
}