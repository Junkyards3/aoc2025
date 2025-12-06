use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Op {
    Plus,
    Mult,
}

struct Problem {
    numbers: Vec<Vec<u64>>,
    ops: Vec<Op>,
}

impl Problem {
    fn compute_problem(&self, index: usize) -> Result<u64> {
        let op = match self.ops[index] {
            Op::Plus => |a, b| a + b,
            Op::Mult => |a, b| a * b,
        };

        (0..self.numbers.len())
            .map(|line| self.numbers[line][index])
            .reduce(op)
            .ok_or(anyhow!("not any numbers on column {index}"))
    }

    fn compute_total_problems(&self) -> Result<u64> {
        (0..self.numbers[0].len())
            .map(|index| self.compute_problem(index))
            .sum()
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let problem = parse(path)?;
    let part1 = part1(&problem);
    let part2 = part2(&problem);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(problem: &Problem) -> u64 {
    problem.compute_total_problems().expect("part 1 error")
}

fn part2(problem: &Problem) -> u64 {
    0
}

enum ParseResult {
    Numbers(Vec<u64>),
    Ops(Vec<Op>),
}

fn parse(path: &str) -> Result<Problem> {
    let file = File::open(path)?;
    let mut numbers = vec![];
    let mut ops = vec![];

    let parsed_lines = BufReader::new(file)
        .lines()
        .map(|s| parse_line(s?.as_str()))
        .collect::<Result<Vec<_>>>()?;

    for parsed_line in parsed_lines {
        match parsed_line {
            ParseResult::Numbers(parsed_items) => numbers.push(parsed_items),
            ParseResult::Ops(parsed_ops) => ops = parsed_ops,
        }
    }

    Ok(Problem { numbers, ops })
}

fn parse_line(line: &str) -> Result<ParseResult> {
    let mut words = line.split_whitespace();
    let first_word = words.next().ok_or(anyhow!("no word on line"))?;
    if first_word.starts_with(['+', '*']) {
        let first_op = parse_op(first_word)?;
        let mut ops = vec![first_op];
        for word in words {
            ops.push(parse_op(word)?);
        }
        Ok(ParseResult::Ops(ops))
    } else {
        let first_number = first_word.parse::<u64>()?;
        let mut numbers = vec![first_number];
        for word in words {
            numbers.push(word.parse::<u64>()?);
        }
        Ok(ParseResult::Numbers(numbers))
    }
}

fn parse_op(op: &str) -> Result<Op> {
    match op {
        "*" => Ok(Op::Mult),
        "+" => Ok(Op::Plus),
        _ => Err(anyhow!("{op} is not an operation")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "4277556");
        assert_eq!(&part2, "0");
    }
}
