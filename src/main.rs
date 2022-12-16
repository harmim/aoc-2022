use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;

const INPUT_DIR: &str = "input";

trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;
}

trait DaySolution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

fn load_input(day: usize, test: bool) -> impl Iterator<Item = String> {
    let mut file_name = if day >= 1 && day <= 9 {
        format!("0{day}")
    } else {
        day.to_string()
    };

    if test {
        file_name += "-test";
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!("{INPUT_DIR}/{file_name}.txt"))
        .expect(format!("Failed to access input data for day {day}.").as_str());
    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Failed to read a line from the input file."))
}

fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        1 => Box::new(Day01::from_lines(lines)),
        2 => Box::new(Day02::from_lines(lines)),
        3 => Box::new(Day03::from_lines(lines)),
        4 => Box::new(Day04::from_lines(lines)),
        5 => Box::new(Day05::from_lines(lines)),
        6 => Box::new(Day06::from_lines(lines)),
        7 => Box::new(Day07::from_lines(lines)),
        8 => Box::new(Day08::from_lines(lines)),
        9 => Box::new(Day09::from_lines(lines)),
        10 => Box::new(Day10::from_lines(lines)),
        11 => Box::new(Day11::from_lines(lines)),
        12 => Box::new(Day12::from_lines(lines)),
        13 => Box::new(Day13::from_lines(lines)),
        _ => panic!("Day has not been solved yet, or it is invalid."),
    }
}

fn time_execution(work: impl FnOnce() -> String) -> (String, Duration) {
    let start = Instant::now();
    let result = work();
    let duration = start.elapsed();

    (result, duration)
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Must provide a day to solve.")
        .parse()
        .expect("The provided day is not a valid integer.");
    let test = env::args().nth(2) == Some("test".to_string());

    let input = load_input(day, test);
    let solution = get_day_solution(day, input);

    println!("Solving day {day}...");
    if test {
        println!("Using a test input file.")
    }

    let (part_one, duration) = time_execution(|| solution.part_one());
    println!("Part 1: {part_one} ({} seconds)", duration.as_secs_f32());

    let (part_two, duration) = time_execution(|| solution.part_two());
    println!("Part 2: {part_two} ({} seconds)", duration.as_secs_f32());
}
