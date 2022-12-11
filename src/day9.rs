use std::collections::HashSet;

use crate::{DaySolution, FromInput};

enum Dir {
    Right,
    Left,
    Up,
    Down,
}

pub struct Day9(Vec<(Dir, usize)>);

impl FromInput for Day9 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| match line.split_once(' ').unwrap() {
                    ("R", steps) => (Dir::Right, steps.parse().unwrap()),
                    ("L", steps) => (Dir::Left, steps.parse().unwrap()),
                    ("U", steps) => (Dir::Up, steps.parse().unwrap()),
                    ("D", steps) => (Dir::Down, steps.parse().unwrap()),
                    _ => panic!("Invalid direction."),
                })
                .collect(),
        )
    }
}

impl DaySolution for Day9 {
    fn part_one(&self) -> String {
        self.get_visited(2)
    }

    fn part_two(&self) -> String {
        self.get_visited(10)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Day9 {
    fn get_visited<'d>(&'d self, knots_count: usize) -> String {
        let mut visited = HashSet::new();
        let mut knots = vec![Pos { x: 0, y: 0 }; knots_count];

        visited.insert(knots[knots_count - 1]);

        fn step(offset: isize) -> isize {
            if offset > 0 {
                1
            } else if offset < 0 {
                -1
            } else {
                offset
            }
        }

        for (direction, steps) in self.0.iter() {
            for _ in 0..*steps {
                match direction {
                    Dir::Right => knots[0].x += 1,
                    Dir::Left => knots[0].x -= 1,
                    Dir::Up => knots[0].y += 1,
                    Dir::Down => knots[0].y -= 1,
                }

                for i in 1..knots_count {
                    let diff = Pos {
                        x: knots[i - 1].x - knots[i].x,
                        y: knots[i - 1].y - knots[i].y,
                    };

                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        knots[i].x += step(diff.x);
                        knots[i].y += step(diff.y);
                    }
                }

                visited.insert(knots[knots_count - 1]);
            }
        }

        visited.len().to_string()
    }
}
