use std::collections::HashSet;

use crate::{DaySolution, FromInput};

pub struct Day6(Vec<char>);

impl FromInput for Day6 {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        Self(lines.next().unwrap().chars().collect())
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        self.find_marker_pos(4)
    }

    fn part_two(&self) -> String {
        self.find_marker_pos(14)
    }
}

impl Day6 {
    fn find_marker_pos<'d>(&'d self, len: usize) -> String {
        for i in len - 1..self.0.len() {
            if HashSet::<char>::from_iter(self.0[i + 1 - len..i + 1].to_vec()).len() == len {
                return (i + 1).to_string();
            }
        }

        panic!("This should not happen if there is a marker.")
    }
}
