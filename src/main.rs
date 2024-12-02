mod days;

use crate::days::get_day;
use clap::{command, Parser};
use std::fmt::Display;
use std::fs;
use std::process;

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
    #[arg(short, long, value_enum)]
    filename: String,
}

fn get_input(day: &u8, filename: &str) -> String {
    let path = format!("inputs/day_{}/{}", day, filename);

    match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", &filename, err);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Args = Args::parse();

    let input = get_input(&args.day, &args.filename);
    let day = match get_day(&args.day, &args.part) {
        Ok(day) => day,
        Err(message) => {
            println!("{}", message);
            process::exit(1)
        }
    };
    let answer: Box<dyn Display> = match args.part {
        Part::PartOne => Box::new(day.part_one(&input)),
        Part::PartTwo => Box::new(day.part_two(&input)),
    };
    println!("{}", answer)
}
