use crate::{DaySolution, FromInput};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub struct Day02(Vec<(Shape, char)>);

impl FromInput for Day02 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let opponent = match line.chars().nth(0).unwrap() {
                        'A' => Shape::Rock,
                        'B' => Shape::Paper,
                        'C' => Shape::Scissors,
                        _ => panic!("Invalid shape."),
                    };
                    (opponent, line.chars().nth(2).unwrap())
                })
                .collect(),
        )
    }
}

impl DaySolution for Day02 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|round| match round {
                (opponent, 'X') => (*opponent, Shape::Rock),
                (opponent, 'Y') => (*opponent, Shape::Paper),
                (opponent, 'Z') => (*opponent, Shape::Scissors),
                _ => panic!("Invalid shape."),
            })
            .map(get_score)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .map(|round| match round {
                (Shape::Rock, 'X') => (Shape::Rock, Shape::Scissors),
                (Shape::Paper, 'X') => (Shape::Paper, Shape::Rock),
                (Shape::Scissors, 'X') => (Shape::Scissors, Shape::Paper),
                (Shape::Rock, 'Z') => (Shape::Rock, Shape::Paper),
                (Shape::Paper, 'Z') => (Shape::Paper, Shape::Scissors),
                (Shape::Scissors, 'Z') => (Shape::Scissors, Shape::Rock),
                (opponent, 'Y') => (*opponent, *opponent),
                _ => panic!("Invalid option."),
            })
            .map(get_score)
            .sum::<usize>()
            .to_string()
    }
}

fn get_score(round: (Shape, Shape)) -> usize {
    match round {
        (Shape::Rock, Shape::Rock) => 1 + 3,
        (Shape::Rock, Shape::Paper) => 2 + 6,
        (Shape::Rock, Shape::Scissors) => 3 + 0,
        (Shape::Paper, Shape::Rock) => 1 + 0,
        (Shape::Paper, Shape::Paper) => 2 + 3,
        (Shape::Paper, Shape::Scissors) => 3 + 6,
        (Shape::Scissors, Shape::Rock) => 1 + 6,
        (Shape::Scissors, Shape::Paper) => 2 + 0,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    }
}
