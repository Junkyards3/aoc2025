use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

enum Op {
    Plus,
    Mult,
}

struct Problem {
    numbers: Vec<Vec<(usize, u64)>>,
    ops: Vec<(usize, Op)>,
}

fn get_digits(num: u64) -> Vec<u64> {
    let mut num = num;
    let mut digits = vec![];
    while num != 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

impl Problem {
    fn compute_problem(&self, index: usize) -> Result<u64> {
        let op = match self.ops[index].1 {
            Op::Plus => |a, b| a + b,
            Op::Mult => |a, b| a * b,
        };

        (0..self.numbers.len())
            .map(|line| self.numbers[line][index].1)
            .reduce(op)
            .ok_or(anyhow!("not any numbers on column {index}"))
    }

    fn compute_total_problems(&self) -> Result<u64> {
        (0..self.numbers[0].len())
            .map(|index| self.compute_problem(index))
            .sum()
    }

    fn compute_problem2(&self, index: usize) -> Result<u64> {
        let op = match self.ops[index].1 {
            Op::Plus => |a, b| a + b,
            Op::Mult => |a, b| a * b,
        };

        let op_pos = self.ops[index].0;

        let numbers_with_pos = (0..self.numbers.len())
            .map(|line| {
                let (pos, num) = self.numbers[line][index];
                let digits = get_digits(num);
                let digits_count = digits.len();
                digits
                    .into_iter()
                    .enumerate()
                    .map(|(pos_in_num, digit)| (pos + digits_count - pos_in_num - 1, digit))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut numbers = vec![];
        let mut curr_pos = op_pos;
        loop {
            let curr_num = numbers_with_pos
                .iter()
                .filter_map(|number_with_pos| {
                    number_with_pos
                        .iter()
                        .filter(|(pos, _)| *pos == curr_pos)
                        .map(|(_, digit)| *digit)
                        .next()
                })
                .reduce(|acc, e| acc * 10 + e);

            if let Some(curr_num) = curr_num {
                numbers.push(curr_num);
                curr_pos += 1;
            } else {
                break;
            }
        }
        numbers
            .into_iter()
            .reduce(op)
            .ok_or(anyhow!("not any numbers on column {index}"))
    }

    fn compute_total_problems2(&self) -> Result<u64> {
        (0..self.numbers[0].len())
            .map(|index| self.compute_problem2(index))
            .sum()
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let problem = parse(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&problem);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&problem);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(problem: &Problem) -> u64 {
    problem.compute_total_problems().expect("part 1 error")
}

fn part2(problem: &Problem) -> u64 {
    problem.compute_total_problems2().expect("part 2 error")
}

enum ParseResult {
    Numbers(Vec<(usize, u64)>),
    Ops(Vec<(usize, Op)>),
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
    let words = split_whitespace_pos(line);
    let first_word = words[0].1;
    if first_word.starts_with(['+', '*']) {
        let mut ops = vec![];
        for (pos, word) in words {
            ops.push((pos, parse_op(word)?));
        }
        Ok(ParseResult::Ops(ops))
    } else {
        let mut numbers = vec![];
        for (pos, word) in words {
            numbers.push((pos, word.parse::<u64>()?));
        }
        Ok(ParseResult::Numbers(numbers))
    }
}

fn split_whitespace_pos(line: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start = None;

    for (i, c) in line.char_indices() {
        match (start, c.is_whitespace()) {
            (None, false) => start = Some(i),
            (Some(st), true) => {
                out.push((st, &line[st..i]));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(st) = start {
        out.push((st, &line[st..]));
    }

    out
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
        assert_eq!(&part2, "3263827");
    }
}
