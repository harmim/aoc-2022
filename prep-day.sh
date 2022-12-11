#!/usr/bin/env bash

set -e

YEAR=2022
SESSION_FILE=.session
INPUT_DIR=input
SRC_DIR=src

if [ -z "$1" ]; then
  echo "Must provide a day of the month as the first argument."
  exit 1
fi

DAY=$(echo "$1" | bc)
if [[ $DAY -lt 1 || $DAY -gt 25 ]]; then
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

DAY_FILE=$DAY
if [[ $DAY -ge 1 && $DAY -le 9 ]]; then
  DAY_FILE="0$DAY"
fi

INPUT_FILE="$INPUT_DIR/$DAY_FILE.txt"
if [ -f "$INPUT_FILE" ]; then
  echo "Input data already exists for day $DAY, skipping download..."
else
  echo "Downloading input data for day $DAY to $INPUT_FILE..."
  mkdir -p "$INPUT_DIR"
  curl "https://adventofcode.com/$YEAR/day/$DAY/input" -s -m 10 \
    -b "session=$SESSION" > "$INPUT_FILE"
fi

SRC_FILE="$SRC_DIR/day$DAY_FILE.rs"
if [ -f "$SRC_FILE" ]; then
  echo "$SRC_FILE already exists, skipping..."
else
  echo "Creating a boilerplate module for day $DAY at $SRC_FILE..."
  echo "Remember to update $SRC_DIR/main.rs:"
  echo "  - Add 'mod day$DAY_FILE;'."
  echo "  - Add 'use day$DAY_FILE::Day$DAY_FILE;'."
  echo "  - Update 'get_day_solution' to use 'Day$DAY_FILE'."
  cat <<-EOF > "$SRC_FILE"
use crate::{DaySolution, FromInput};

pub struct Day$DAY_FILE;

impl FromInput for Day$DAY_FILE {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        Self
    }
}

impl DaySolution for Day$DAY_FILE {
    fn part_one(&self) -> String {
        String::from("")
    }

    fn part_two(&self) -> String {
        String::from("")
    }
}
EOF
fi

echo "Happy coding!"
