use crate::InputType;

pub fn part1(input_type: InputType) -> String {
    println!("Day n part 1");
    let input_path = match input_type {
        InputType::Example => "src/day-n/example.txt",
        InputType::Real => "src/day-n/data.txt",
    };

    input_path.to_string()
}

pub fn part2(input_type: InputType) -> String {
    println!("Day n part 2");
    let input_path = match input_type {
        InputType::Example => "src/day-n/example.txt",
        InputType::Real => "src/day-n/data.txt",
    };

    input_path.to_string()
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