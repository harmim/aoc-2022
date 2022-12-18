use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::{DaySolution, FromInput};

pub struct Day14(HashSet<(usize, usize)>);

const SOURCE: (usize, usize) = (500, 0);

impl FromInput for Day14 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut map = HashSet::from([SOURCE]);

        for line in lines {
            let mut prev = None;

            for shape in line.split(" -> ") {
                let (x, y) = shape
                    .split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap();

                match prev {
                    Some((prev_x, prev_y)) => {
                        if x != prev_x {
                            for xi in cmp::min(x, prev_x)..cmp::max(x, prev_x) + 1 {
                                map.insert((xi, y));
                            }
                        } else {
                            for yi in cmp::min(y, prev_y)..cmp::max(y, prev_y) + 1 {
                                map.insert((x, yi));
                            }
                        }
                    }
                    None => (),
                }

                prev = Some((x, y));
            }
        }

        Self(map)
    }
}

impl DaySolution for Day14 {
    fn part_one(&self) -> String {
        let mut max_ys = HashMap::new();

        for (x, y) in self.0.iter() {
            if max_ys.contains_key(x) {
                *max_ys.get_mut(x).unwrap() = cmp::max(*y, max_ys[x]);
            } else {
                max_ys.insert(*x, *y);
            }
        }

        self.get_sand_units(self.0.clone(), Some(max_ys))
    }

    fn part_two(&self) -> String {
        let mut map = self.0.clone();
        let max_y = map.iter().map(|(_, y)| y).max().unwrap() + 2;

        for x in SOURCE.0 - max_y..SOURCE.0 + max_y + 1 {
            map.insert((x, max_y));
        }

        self.get_sand_units(map, None)
    }
}

impl Day14 {
    fn get_sand_units<'d>(
        &'d self,
        mut map: HashSet<(usize, usize)>,
        max_ys: Option<HashMap<usize, usize>>,
    ) -> String {
        let mut sand_units = 0;

        loop {
            let (mut x, mut y) = SOURCE;
            let mut end = false;

            loop {
                if !map.contains(&(x, y + 1)) {
                    ();
                } else if !map.contains(&(x - 1, y + 1)) {
                    x -= 1;
                } else if !map.contains(&(x + 1, y + 1)) {
                    x += 1;
                } else {
                    if (x, y) == SOURCE {
                        end = true;
                    }

                    map.insert((x, y));
                    sand_units += 1;
                    break;
                }

                y += 1;

                match &max_ys {
                    Some(max_ys) => {
                        if !max_ys.contains_key(&x) || max_ys[&x] < y {
                            end = true;
                            break;
                        }
                    }
                    None => (),
                }
            }

            if end {
                break;
            }
        }

        sand_units.to_string()
    }
}
