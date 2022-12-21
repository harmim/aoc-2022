use gcollections::ops::{Bounded, Cardinality, Difference, Union};
use interval::interval_set::IntervalSet;
use interval::ops::Range;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::{DaySolution, FromInput};

pub struct Day15 {
    row: isize,
    beacons: HashMap<isize, (IntervalSet<isize>, HashSet<isize>)>,
}

const ROW: isize = 2_000_000;
const TEST_ROW: isize = 10;

const X_MIN: isize = 0;
const Y_MIN: isize = X_MIN;
const X_MAX: isize = 4_000_000;
const Y_MAX: isize = X_MAX;
const TEST_X_MAX: isize = 20;
const TEST_Y_MAX: isize = TEST_X_MAX;

const FREQUENCY_MULTIPLIER: isize = X_MAX;

impl FromInput for Day15 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut lines_count = 0;
        let mut beacons: HashMap<_, (IntervalSet<_>, HashSet<_>)> = HashMap::new();

        for line in lines {
            lines_count += 1;

            let sensor = Regex::new(
                "^Sensor at x=(?P<x1>-?\\d+), y=(?P<y1>-?\\d+): \
                closest beacon is at x=(?P<x2>-?\\d+), y=(?P<y2>-?\\d+)$",
            )
            .unwrap()
            .captures(&line)
            .unwrap();
            let x1: isize = (&sensor["x1"]).parse().unwrap();
            let y1: isize = (&sensor["y1"]).parse().unwrap();
            let x2: isize = (&sensor["x2"]).parse().unwrap();
            let y2: isize = (&sensor["y2"]).parse().unwrap();
            let dist = (x1 - x2).abs() + (y1 - y2).abs();

            for i in 0..dist * 2 + 1 {
                let y = y1 - (dist - i);
                let count = if i <= dist { i } else { dist - (i - dist) };
                let interval = IntervalSet::new(x1 - count, x1 + count);

                let beacons_pos = if y == y2 {
                    HashSet::from([x2])
                } else {
                    HashSet::new()
                };

                match beacons.get_mut(&y) {
                    Some((position, beacons_positions)) => {
                        *position = position.union(&interval);
                        beacons_positions.extend(beacons_pos);
                    }
                    None => drop(beacons.insert(y, (interval, beacons_pos))),
                }
            }
        }

        Self {
            row: if lines_count == 14 { TEST_ROW } else { ROW },
            beacons,
        }
    }
}

impl DaySolution for Day15 {
    fn part_one(&self) -> String {
        (self
            .beacons
            .get(&self.row)
            .unwrap()
            .0
            .iter()
            .map(|interval| interval.size())
            .sum::<usize>()
            - self.beacons.get(&self.row).unwrap().1.len())
        .to_string()
    }

    fn part_two(&self) -> String {
        let (x_max, y_max) = if self.row == TEST_ROW {
            (TEST_X_MAX, TEST_Y_MAX)
        } else {
            (X_MAX, Y_MAX)
        };

        self.beacons
            .iter()
            .filter_map(|(&y, (interval, _))| {
                if y < Y_MIN || y > y_max {
                    None
                } else {
                    match IntervalSet::new(X_MIN, x_max)
                        .difference(interval)
                        .iter()
                        .nth(0)
                    {
                        Some(beacon) => Some(beacon.lower() * FREQUENCY_MULTIPLIER + y),
                        None => None,
                    }
                }
            })
            .nth(0)
            .unwrap()
            .to_string()
    }
}
