#!/usr/bin/env bash

set -e

YEAR=2022
SESSION_FILE=.session
INPUT_DIR=input
SRC_DIR=src

if [ -z "$1" ]; then
  echo "Must provide a day of the month (not zero-padded) as the first argument."
  exit 1
fi

if [[ "$1" -lt 1 || "$1" -gt 25 ]]; then
  echo "The day must be between 1 and 25, inclusive."
  exit 1
fi

if [ ! -f "$SESSION_FILE" ]; then
  echo "File '$SESSION_FILE' with the user's session key from the Advent of" \
    "Code website does not exist."
  exit 1
fi

SESSION=$(cat "$SESSION_FILE")
if [ -z "$SESSION" ]; then
  echo "Must set the session from the Advent of Code website."
  exit 1
fi

if [ -f "$INPUT_DIR/$1.txt" ]; then
  echo "Input data already exists for day $1, skipping download..."
else
  echo "Downloading input data for day $1 to $INPUT_DIR/$1.txt..."
  mkdir -p "$INPUT_DIR"
  curl "https://adventofcode.com/$YEAR/day/$1/input" -s -m 10 \
    -b "session=$SESSION" > "$INPUT_DIR/$1.txt"
fi

if [ -f "$SRC_DIR/day$1.rs" ]; then
  echo "$SRC_DIR/day$1.rs already exists, skipping..."
else
  echo "Creating a boilerplate module for day $1 at $SRC_DIR/day$1.rs..."
  echo "Remember to update $SRC_DIR/main.rs:"
  echo "  - Add 'mod day$1;'."
  echo "  - Add 'use day$1::Day$1;'."
  echo "  - Update 'get_day_solution' to use 'Day$1'."
  cat <<-EOF > "$SRC_DIR/day$1.rs"
use crate::{FromInput, DaySolution};

pub struct Day$1;

impl FromInput for Day$1 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        // Self
        todo!("Parse your input from the input file.")
    }
}

impl DaySolution for Day$1 {
    fn part_one(&self) -> String {
        // String::from("")
        todo!("Solve part one of day $1 using your parsed input.")
    }

    fn part_two(&self) -> String {
        // String::from("")
        todo!("Solve part two of day $1 using your parsed input.")
    }
}
EOF
fi

echo "Happy coding!"
