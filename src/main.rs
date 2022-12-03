use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

mod day1;

use day1::Day1;

const INPUT_DIR: &str = "input";

trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;
}

trait DaySolution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

fn load_input(day: usize, test: bool) -> impl Iterator<Item = String> {
    let file_name = if test {
        format!("{day}-test")
    } else {
        day.to_string()
    };
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!("{INPUT_DIR}/{file_name}.txt"))
        .expect(format!("Failed to access input data for day {day}.").as_str());
    let buffered_file = BufReader::new(file);

    buffered_file.lines().map(|line| {
        String::from(
            line.expect("Failed to read a line from the input file")
                .trim(),
        )
    })
}

fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        1 => Box::new(Day1::from_lines(lines)),
        _ => panic!("Day has not been solved yet."),
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
        .parse::<usize>()
        .expect("The provided day is not a valid integer.");
    let test = env::args().nth(2) == Some(String::from("test"));

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