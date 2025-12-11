use anyhow::{Result, anyhow};
use std::time::Instant;

struct Problem {
    width: usize,
    height: usize,
    piece_counts: Vec<usize>,
}

impl Problem {
    fn definitely_fits(&self) -> bool {
        let width_3 = self.width / 3;
        let height_3 = self.height / 3;
        self.piece_counts.iter().sum::<usize>() <= width_3 * height_3
    }

    fn definitely_does_not_fit(&self, piece_sizes: &[usize]) -> bool {
        let cells_count = self.width * self.height;

        piece_sizes
            .iter()
            .zip(self.piece_counts.iter())
            .map(|(size, count)| size * count)
            .sum::<usize>()
            > cells_count
    }
}

#[derive(Debug, Default)]
struct ProblemResult {
    fit: usize,
    does_not_fit: usize,
    unknown: usize,
}

struct Problems {
    piece_sizes: Vec<usize>,
    problems: Vec<Problem>,
}

impl Problems {
    fn part1(&self) -> ProblemResult {
        let mut result = ProblemResult::default();
        for problem in self.problems.iter() {
            if problem.definitely_fits() {
                result.fit += 1;
            } else if problem.definitely_does_not_fit(&self.piece_sizes) {
                result.does_not_fit += 1;
            } else {
                result.unknown += 1;
            }
        }
        result
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let problems = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&problems);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2();
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((format!("{:?}", part1), part2.to_string()))
}

fn part1(problems: &Problems) -> ProblemResult {
    problems.part1()
}

fn part2() -> u128 {
    0
}

fn parse_file(path: &str) -> Result<Problems> {
    let text = std::fs::read_to_string(path)?;
    let mut piece_sizes = vec![];
    let mut problems = vec![];
    for part in text.split("\n\n") {
        if part.contains("#") {
            piece_sizes.push(parse_piece_part(part));
        } else {
            for line in part.lines() {
                problems.push(parse_grid_line(line)?);
            }
        }
    }

    Ok(Problems {
        piece_sizes,
        problems,
    })
}

fn parse_piece_part(part: &str) -> usize {
    part.chars().filter(|ch| *ch == '#').count()
}

fn parse_grid_line(line: &str) -> Result<Problem> {
    let (dimensions, counts) = line
        .split_once(": ")
        .ok_or(anyhow!("could not split on :"))?;
    let dimensions = dimensions
        .split("x")
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| anyhow!("could not parse as usize {}", s))
        })
        .collect::<Result<Vec<_>>>()?;
    let counts = counts
        .split_whitespace()
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| anyhow!("could not parse as usize {}", s))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Problem {
        width: dimensions[0],
        height: dimensions[1],
        piece_counts: counts,
    })
}
