use crate::{DaySolution, FromInput};

pub struct Day01(Vec<Vec<usize>>);

impl FromInput for Day01 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut output = vec![];
        let mut single_elf = vec![];

        fn update(output: &mut Vec<Vec<usize>>, single_elf: &mut Vec<usize>) {
            if !single_elf.is_empty() {
                output.push(single_elf.to_vec());
                single_elf.clear()
            }
        }

        for line in lines {
            if line.is_empty() {
                update(&mut output, &mut single_elf);
                continue;
            }

            single_elf.push(line.parse().unwrap())
        }

        update(&mut output, &mut single_elf);

        Self(output)
    }
}

impl DaySolution for Day01 {
    fn part_one(&self) -> String {
        match self
            .0
            .iter()
            .map(|single_elf| single_elf.iter().sum::<usize>())
            .max()
        {
            Some(sum) => sum,
            _ => panic!("There should be at least one Elf."),
        }
        .to_string()
    }

    fn part_two(&self) -> String {
        let mut sums: Vec<_> = self
            .0
            .iter()
            .map(|single_elf| single_elf.iter().sum())
            .collect();
        sums.sort();
        sums.reverse();

        sums.iter().take(3).sum::<usize>().to_string()
    }
}
