extern crate core;

mod days;
mod utils;

use crate::days::get_day;
use clap::{command, Parser};
use std::fmt::Display;
use std::fs;
use std::process;
use std::time::Instant;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day to execute, supports 1-25
    #[arg(short, long)]
    day: u8,
    /// Which part to execute
    #[arg(short, long)]
    part: u8,
}

fn get_input(day: &u8) -> String {
    let path = format!("inputs/day_{}/input.txt", day);

    match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Args = Args::parse();

    let input = get_input(&args.day);
    let day = match get_day(&args.day) {
        Ok(day) => day,
        Err(message) => {
            println!("{}", message);
            process::exit(1)
        }
    };
    let start = Instant::now();
    let answer: Box<dyn Display> = match args.part {
        1 => Box::new(day.part_one(&input)),
        2 => Box::new(day.part_two(&input)),
        _ => panic!("Only parts 1 or 2 are supported."),
    };
    println!("Duration: {:?}, Answer: {}", start.elapsed(), answer);
}
