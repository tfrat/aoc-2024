mod days;

use std::fs;
use std::process;
use clap::{command, Parser};

#[derive(clap::ValueEnum, Clone, Eq, PartialEq)]
enum Part {
    PartOne,
    PartTwo,
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day to execute, supports 1-25
    #[arg(short, long)]
    day: u8,
    /// Which part to execute
    #[arg(short, long, value_enum)]
    part: Part,
    /// Use the example input?
    #[arg(short, long, action)]
    example: bool,
}

fn get_input(day: &u8, part: &Part, example: &bool) -> String {
    let filename = if *example {
        "example.txt"
    } else {
        if part == &Part::PartOne {
            "part_one.txt"
        } else {
            "part_two.txt"
        }
    };
    let path = format!("inputs/day_{}/{}", day, &filename);

    match fs::exists(path) {
        Result::Ok(bool) => ,

    }

    match fs::read_to_string(&path) {
        Ok(contents) => {
            contents
        }
        Err(err) => {
            eprintln!("Error reading file '{}': {}", &filename, err);
            process::exit(1);
        }
    }
}

fn main() {

    let args: Args = Args::parse();
    let input = get_input(&args.day, &args.part, &args.example);
    let answer = match &args.part {
        &Part::PartOne => days::day_1::part_one(&input),
        &Part::PartTwo => days::day_1::part_two(&input),
    };
    println!("{}\n", answer)
}
