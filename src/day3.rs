use std::collections::HashSet;

use crate::{DaySolution, FromInput};

pub struct Day3(Vec<Vec<char>>);

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|line| line.chars().collect()).collect())
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|bag| {
                let mid = bag.len() / 2;
                let compartment1 = HashSet::<char>::from_iter(bag.iter().take(mid).cloned());
                let compartment2 = HashSet::<char>::from_iter(bag.iter().skip(mid).cloned());
                *compartment1.intersection(&compartment2).next().unwrap()
            })
            .map(type_to_priority)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .chunks(3)
            .map(|group| {
                let bag1 = HashSet::<char>::from_iter(group.get(0).unwrap().clone());
                let bag2 = HashSet::<char>::from_iter(group.get(1).unwrap().clone());
                let bag3 = HashSet::<char>::from_iter(group.get(2).unwrap().clone());
                *bag1
                    .intersection(&bag2)
                    .cloned()
                    .collect::<HashSet<char>>()
                    .intersection(&bag3)
                    .next()
                    .unwrap()
            })
            .map(type_to_priority)
            .sum::<usize>()
            .to_string()
    }
}

fn type_to_priority(t: char) -> usize {
    if t.is_ascii_lowercase() {
        t as usize - 'a' as usize + 1
    } else if t.is_ascii_uppercase() {
        const DIFF: usize = 'Z' as usize - 'A' as usize + 1;
        t as usize - 'A' as usize + 1 + DIFF
    } else {
        panic!("Invalid type.")
    }
}
