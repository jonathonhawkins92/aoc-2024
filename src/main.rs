use clap::{Parser, ValueEnum};
use std::error::Error;
#[path = "day-1/main.rs"]
mod day1;

#[derive(Clone, ValueEnum, Debug)]
enum InputType {
    Example,
    Real,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run (1-25)
    #[arg(short, long)]
    day: u8,

    /// Which part of the day to run (1-2)
    #[arg(short, long)]
    part: u8,

    /// Whether to use example data or real input
    #[arg(short, long, value_enum, default_value_t = InputType::Real)]
    input_type: InputType,
}



macro_rules! run_day {
    ($day:expr, $part:expr, $input_type:expr) => {
        match ($day, $part) {
            (1, 1) => day1::part1($input_type),
            (1, 2) => day1::part2($input_type),
            // Add more days as needed
            _ => "Day not implemented".to_string(),
        }
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Validate input ranges
    if args.day < 1 || args.day > 25 {
        return Err("Day must be between 1 and 25".into());
    }
    if args.part < 1 || args.part > 2 {
        return Err("Part must be either 1 or 2".into());
    }

    let result = run_day!(args.day, args.part, args.input_type);
    println!("Result: {}", result);

    Ok(())
}
