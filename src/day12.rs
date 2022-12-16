use std::collections::{HashSet, VecDeque};
use std::usize;

use crate::{DaySolution, FromInput};

pub struct Day12 {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<char>>,
}

impl FromInput for Day12 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = vec![];

        for (x, line) in lines.enumerate() {
            let mut row = vec![];

            for (y, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (x, y);
                        row.push('a');
                    }
                    'E' => {
                        end = (x, y);
                        row.push('z');
                    }
                    _ => row.push(c),
                }
            }

            map.push(row);
        }

        Self { start, end, map }
    }
}

impl DaySolution for Day12 {
    fn part_one(&self) -> String {
        self.find_path(self.start).to_string()
    }

    fn part_two(&self) -> String {
        let mut paths = vec![];

        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                if self.map[x][y] == 'a' {
                    paths.push(self.find_path((x, y)));
                }
            }
        }

        paths.iter().min().unwrap().to_string()
    }
}

impl Day12 {
    fn find_path<'d>(&'d self, start: (usize, usize)) -> usize {
        let mut open = VecDeque::from([(start.0, start.1, 0)]);
        let mut closed = HashSet::new();
        const MOVES: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while open.len() > 0 {
            let square = open.pop_front().unwrap();

            for mov in MOVES {
                let new = (square.0 as isize + mov.0, square.1 as isize + mov.1);

                if new.0 < 0
                    || new.1 < 0
                    || new.0 as usize >= self.map.len()
                    || new.1 as usize >= self.map[0].len()
                {
                    continue;
                }
                let new = (new.0 as usize, new.1 as usize);

                if closed.contains(&new)
                    || open.iter().any(|(x, y, _)| *x == new.0 && *y == new.1)
                    || self.map[new.0][new.1] as u8 - 1 > self.map[square.0][square.1] as u8
                {
                    continue;
                }

                if new.0 == self.end.0 && new.1 == self.end.1 {
                    return square.2 + 1;
                }

                open.push_back((new.0, new.1, square.2 + 1));
            }

            closed.insert((square.0, square.1));
        }

        usize::MAX
    }
}
