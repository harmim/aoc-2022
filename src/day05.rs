use regex::Regex;

use crate::{DaySolution, FromInput};

pub struct Day05((Vec<Vec<char>>, Vec<(usize, usize, usize)>));

impl FromInput for Day05 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut stacks = vec![];
        let mut proc = vec![];
        let mut proc_start = false;

        for line in lines {
            if !proc_start {
                if line.is_empty() {
                    proc_start = true;
                    continue;
                }

                if line.find('[') == None {
                    continue;
                }

                let mut stack = vec![];
                let mut i = 0;

                for c in line.chars() {
                    i = match c {
                        '[' => i,
                        ']' => -1,
                        ' ' if i == 3 => 0,
                        ' ' => {
                            let ii = i + 1;
                            if ii == 3 {
                                stack.push(None);
                            }
                            ii
                        }
                        x => {
                            stack.push(Some(x));
                            i
                        }
                    }
                }

                if stacks.is_empty() {
                    for _ in 0..stack.len() {
                        stacks.push(vec![]);
                    }
                }

                for (i, s) in stacks.iter_mut().enumerate() {
                    match stack[i] {
                        Some(x) => s.push(x),
                        _ => (),
                    }
                }
            } else {
                let op = Regex::new(r"^move (?P<q>\d+) from (?P<from>\d+) to (?P<to>\d+)$")
                    .unwrap()
                    .captures(&line)
                    .unwrap();

                proc.push((
                    op["q"].parse().unwrap(),
                    op["from"].parse::<usize>().unwrap() - 1,
                    op["to"].parse::<usize>().unwrap() - 1,
                ))
            }
        }

        Self((stacks, proc))
    }
}

impl DaySolution for Day05 {
    fn part_one(&self) -> String {
        let mut stacks = self.0 .0.clone();
        for (q, from, to) in &self.0 .1 {
            for _ in 0..*q {
                let x = stacks[*from].remove(0);
                stacks[*to].insert(0, x);
            }
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }

    fn part_two(&self) -> String {
        let mut stacks = self.0 .0.clone();
        for (q, from, to) in &self.0 .1 {
            let mut xs: Vec<_> = stacks[*from].drain(0..*q).collect();
            xs.append(&mut stacks[*to]);

            stacks[*to] = xs
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }
}
