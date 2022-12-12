use crate::{DaySolution, FromInput};

pub struct Day04(Vec<((usize, usize), (usize, usize))>);

impl FromInput for Day04 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        fn parse_range(range: &str) -> (usize, usize) {
            let bounds = range.split_once('-').unwrap();

            (bounds.0.parse().unwrap(), bounds.1.parse().unwrap())
        }

        Self(
            lines
                .map(|line| {
                    let ranges = line.split_once(',').unwrap();

                    (parse_range(ranges.0), parse_range(ranges.1))
                })
                .collect(),
        )
    }
}

impl DaySolution for Day04 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|ranges| match ranges {
                ((a1, a2), (b1, b2)) if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) => 1,
                _ => 0,
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .map(|ranges| match ranges {
                ((a1, a2), (b1, b2)) if a2 >= b1 && a1 <= b2 => 1,
                _ => 0,
            })
            .sum::<usize>()
            .to_string()
    }
}
