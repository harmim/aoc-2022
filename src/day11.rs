use num_integer;
use regex::Regex;

use crate::{DaySolution, FromInput};

enum Operator {
    PLUS,
    MUL,
}

enum Operand {
    OLD,
    VAL(usize),
}

struct Monkey {
    items: Vec<usize>,
    operation: (Operator, Operand),
    test: (usize, usize, usize),
}

pub struct Day11(Vec<Monkey>);

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        let mut i = 0;

        fn add_monkey(monkeys: &mut Vec<Monkey>) {
            monkeys.push(Monkey {
                items: vec![],
                operation: (Operator::PLUS, Operand::OLD),
                test: (0, 0, 0),
            });
        }

        add_monkey(&mut monkeys);

        for line in lines {
            if line.is_empty() {
                i += 1;
                add_monkey(&mut monkeys);
                continue;
            }

            if Regex::new(r"^Monkey \d+:$").unwrap().is_match(&line) {
                continue;
            }

            match Regex::new(r"^  Starting items: ((?:\d+(?:, )?)+)$")
                .unwrap()
                .captures(&line)
            {
                Some(m) => {
                    monkeys[i].items = m[1].split(", ").map(|item| item.parse().unwrap()).collect();
                    continue;
                }
                None => (),
            }

            match Regex::new(r"^  Operation: new = old (\+|\*) (old|\d+)$")
                .unwrap()
                .captures(&line)
            {
                Some(m) => {
                    let operator = if &m[1] == "+" {
                        Operator::PLUS
                    } else {
                        Operator::MUL
                    };
                    let operand = if &m[2] == "old" {
                        Operand::OLD
                    } else {
                        Operand::VAL(m[2].parse().unwrap())
                    };
                    monkeys[i].operation = (operator, operand);
                    continue;
                }
                None => (),
            }

            match Regex::new(r"^  Test: divisible by (\d+)$")
                .unwrap()
                .captures(&line)
            {
                Some(m) => {
                    monkeys[i].test.0 = m[1].parse().unwrap();
                    continue;
                }
                None => (),
            }

            match Regex::new(r"^    If true: throw to monkey (\d+)$")
                .unwrap()
                .captures(&line)
            {
                Some(m) => {
                    monkeys[i].test.1 = m[1].parse().unwrap();
                    continue;
                }
                None => (),
            }

            match Regex::new(r"^    If false: throw to monkey (\d+)$")
                .unwrap()
                .captures(&line)
            {
                Some(m) => {
                    monkeys[i].test.2 = m[1].parse().unwrap();
                }
                None => panic!("Invalid input."),
            }
        }

        Self(monkeys)
    }
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        self.get_activity(20, |level| level / 3)
    }

    fn part_two(&self) -> String {
        let lcm = self
            .0
            .iter()
            .fold(1, |lcm, monkey| num_integer::lcm(lcm, monkey.test.0));

        self.get_activity(10_000, |level| level % lcm)
    }
}

impl Day11 {
    fn get_activity<'d>(&'d self, rounds: usize, reduce: impl Fn(usize) -> usize) -> String {
        let mut activity = vec![0; self.0.len()];
        let mut items: Vec<_> = self.0.iter().map(|monkey| monkey.items.clone()).collect();

        for _ in 0..rounds {
            for (i, monkey) in self.0.iter().enumerate() {
                for old in items[i].clone() {
                    let operand = match monkey.operation.1 {
                        Operand::OLD => old.clone(),
                        Operand::VAL(val) => val,
                    };
                    let new = reduce(match monkey.operation.0 {
                        Operator::PLUS => old + operand,
                        Operator::MUL => old * operand,
                    });

                    if new % monkey.test.0 == 0 {
                        items[monkey.test.1].push(new);
                    } else {
                        items[monkey.test.2].push(new);
                    }

                    activity[i] += 1;
                }

                items[i].clear();
            }
        }

        activity.sort();

        activity.iter().rev().take(2).product::<usize>().to_string()
    }
}
