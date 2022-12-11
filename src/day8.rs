use std::cmp;

use crate::{DaySolution, FromInput};

pub struct Day8(Vec<Vec<usize>>);

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        )
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> String {
        let mut grid: Vec<Vec<(isize, isize, isize, isize, isize)>> = self
            .0
            .iter()
            .map(|row| row.iter().map(|&h| (h as isize, -1, -1, -1, -1)).collect())
            .collect();
        let x_size = self.0.len();
        let y_size = self.0[0].len();

        for x in 0..x_size {
            for y in 0..y_size {
                if y == 0 {
                    continue;
                }
                grid[x][y].1 = cmp::max(grid[x][y - 1].0, grid[x][y - 1].1);
            }
        }

        for x in 0..x_size {
            for y in (0..y_size).rev() {
                if y == y_size - 1 {
                    continue;
                }
                grid[x][y].2 = cmp::max(grid[x][y + 1].0, grid[x][y + 1].2);
            }
        }

        for y in 0..y_size {
            for x in 0..x_size {
                if x == 0 {
                    continue;
                }
                grid[x][y].3 = cmp::max(grid[x - 1][y].0, grid[x - 1][y].3);
            }
        }

        for y in 0..y_size {
            for x in (0..x_size).rev() {
                if x == x_size - 1 {
                    continue;
                }
                grid[x][y].4 = cmp::max(grid[x + 1][y].0, grid[x + 1][y].4);
            }
        }

        grid.iter()
            .map(|row| {
                row.iter()
                    .filter(|(h, l, r, t, b)| h > l || h > r || h > t || h > b)
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let x_size = self.0.len();
        let y_size = self.0[0].len();
        let mut all_scores: Vec<usize> = vec![];

        for x in 0..x_size {
            for y in 0..y_size {
                let mut scores = vec![];
                let mut score = 0;

                for yr in y + 1..y_size {
                    score += 1;
                    if self.0[x][yr] >= self.0[x][y] {
                        break;
                    }
                }
                scores.push(score);

                score = 0;
                for yl in (0..y).rev() {
                    score += 1;
                    if self.0[x][yl] >= self.0[x][y] {
                        break;
                    }
                }
                scores.push(score);

                score = 0;
                for xb in x + 1..x_size {
                    score += 1;
                    if self.0[xb][y] >= self.0[x][y] {
                        break;
                    }
                }
                scores.push(score);

                score = 0;
                for xt in (0..x).rev() {
                    score += 1;
                    if self.0[xt][y] >= self.0[x][y] {
                        break;
                    }
                }
                scores.push(score);

                all_scores.push(scores.iter().product());
            }
        }

        all_scores.iter().max().unwrap().to_string()
    }
}
