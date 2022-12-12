use crate::{DaySolution, FromInput};

enum Ins {
    ADD(isize),
    NOOP,
}

pub struct Day10(Vec<Ins>);

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    if line.starts_with("addx") {
                        Ins::ADD(line.split_once(' ').unwrap().1.parse().unwrap())
                    } else {
                        Ins::NOOP
                    }
                })
                .collect(),
        )
    }
}

impl DaySolution for Day10 {
    fn part_one(&self) -> String {
        let mut output = String::from('0');

        self.process_instrs(
            &mut output,
            vec![20, 60, 100, 140, 180, 220],
            |_, _, _| (),
            |cycle, x, output| {
                *output = (output.parse::<isize>().unwrap() + *cycle as isize * x).to_string()
            },
        );

        output
    }

    fn part_two(&self) -> String {
        let mut output = String::from("\n");

        self.process_instrs(
            &mut output,
            vec![41, 81, 121, 161, 201, 241],
            |cycle, x, output| {
                let cycle = (cycle - 1) as isize;

                if x - 1 <= cycle && x + 1 >= cycle {
                    *output += "#";
                } else {
                    *output += ".";
                }
            },
            |cycle, _, output| {
                *cycle = 1;
                *output += "\n";
            },
        );

        output
    }
}

impl Day10 {
    fn process_instrs<'d>(
        &'d self,
        output: &mut String,
        specials: Vec<usize>,
        each_cycle_callback: impl Fn(usize, isize, &mut String),
        special_cycle_callback: impl Fn(&mut usize, isize, &mut String),
    ) {
        let mut cycle = 1;
        let mut x = 1;

        let special_cycles = |cycle: &mut usize, x, output: &mut String| {
            for c in specials.clone() {
                if c == *cycle {
                    special_cycle_callback(cycle, x, output);
                }
            }
        };

        for ins in self.0.iter() {
            match ins {
                Ins::NOOP => {
                    each_cycle_callback(cycle, x, output);
                    cycle += 1;
                    special_cycles(&mut cycle, x, output);
                }
                Ins::ADD(op) => {
                    each_cycle_callback(cycle, x, output);
                    cycle += 1;
                    special_cycles(&mut cycle, x, output);

                    each_cycle_callback(cycle, x, output);
                    x += op;
                    cycle += 1;
                    special_cycles(&mut cycle, x, output);
                }
            }
        }
    }
}
