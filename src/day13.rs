use std::cmp::{self, Ordering};

use crate::{DaySolution, FromInput};

#[derive(PartialEq, Clone)]
enum Data {
    INT(usize),
    PACKET(Packet),
}

#[derive(PartialEq, Clone)]
struct Packet(Vec<Data>);

pub struct Day13(Vec<(Packet, Packet)>);

impl FromInput for Day13 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut list = vec![];
        list.push((Packet(vec![]), Packet(vec![])));

        fn build_packet(mut line: String) -> (Packet, usize) {
            let mut packet = vec![];
            let mut skip = 0;
            let mut number = String::new();

            line.remove(0);
            for (i, c) in line.chars().enumerate() {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }

                if !number.is_empty() && (c == ']' || c == ',') {
                    packet.push(Data::INT(number.parse().unwrap()));
                    number.clear();
                }

                match c {
                    '[' => {
                        let (nested_packet, len) = build_packet(line.clone().drain(i..).collect());
                        packet.push(Data::PACKET(nested_packet));
                        skip = len;
                    }
                    ']' => return (Packet(packet), i + 1),
                    ',' => (),
                    _ => number += &c.to_string(),
                }
            }

            panic!("Invalid input.");
        }

        for (i, line) in lines.enumerate() {
            match i % 3 {
                0 => (list[i / 3].0, _) = build_packet(line),
                1 => (list[i / 3].1, _) = build_packet(line),
                _ => list.push((Packet(vec![]), Packet(vec![]))),
            }
        }

        Self(list)
    }
}

impl DaySolution for Day13 {
    fn part_one(&self) -> String {
        let mut right_order = vec![];

        for (i, (left, right)) in self.0.iter().enumerate() {
            match self.cmp_packets(left, right) {
                Ordering::Less => right_order.push(i + 1),
                _ => (),
            }
        }

        right_order.iter().sum::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        let dividers = [
            Packet(vec![Data::PACKET(Packet(vec![Data::INT(2)]))]),
            Packet(vec![Data::PACKET(Packet(vec![Data::INT(6)]))]),
        ];

        let mut list: Vec<_> = self
            .0
            .iter()
            .flat_map(|(l, r)| [l.clone(), r.clone()])
            .chain(dividers.clone())
            .collect();

        list.sort_by(|a, b| self.cmp_packets(a, b));

        dividers
            .iter()
            .map(|divider| list.iter().position(|packet| packet == divider).unwrap() + 1)
            .product::<usize>()
            .to_string()
    }
}

impl Day13 {
    fn cmp_packets<'d>(&'d self, left: &Packet, right: &Packet) -> Ordering {
        let left = &left.0;
        let right = &right.0;

        for i in 0..cmp::max(left.len(), right.len()) {
            if i >= left.len() {
                return Ordering::Less;
            }

            if i >= right.len() {
                return Ordering::Greater;
            }

            let result = match (&left[i], &right[i]) {
                (Data::INT(l), Data::INT(r)) => l.cmp(r),
                (Data::PACKET(l), Data::PACKET(r)) => self.cmp_packets(l, r),
                (l @ Data::INT(_), Data::PACKET(r)) => {
                    self.cmp_packets(&Packet(vec![l.clone()]), r)
                }
                (Data::PACKET(l), r @ Data::INT(_)) => {
                    self.cmp_packets(l, &Packet(vec![r.clone()]))
                }
            };

            match result {
                Ordering::Equal => (),
                _ => return result,
            }
        }

        Ordering::Equal
    }
}
