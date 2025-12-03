use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct BatteryLine(Vec<u8>);

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let file = File::open(path)?;
    let battery_lines: Vec<BatteryLine> = BufReader::new(file)
        .lines()
        .map(|s| parse_line(s?.as_str()))
        .collect::<Result<Vec<_>>>()?;

    let part1 = part1(&battery_lines);
    let part2 = part2(&battery_lines);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(battery_lines: &[BatteryLine]) -> u64 {
    battery_lines
        .iter()
        .map(|battery_line| compute_voltage(battery_line, 2))
        .sum()
}

fn part2(battery_lines: &[BatteryLine]) -> u64 {
    battery_lines
        .iter()
        .map(|battery_line| compute_voltage(battery_line, 12))
        .sum()
}

struct VoltageLoop {
    size: usize,
    values: Vec<Option<u8>>,
}

impl VoltageLoop {
    fn new(size: usize) -> Self {
        VoltageLoop {
            size,
            values: vec![None; size],
        }
    }

    fn update(&mut self, digit: u8, remaining_digits: usize) {
        let start_index = self.size.saturating_sub(remaining_digits);
        for index in start_index..self.size {
            if self.values[index].is_none_or(|value| value < digit) {
                self.values[index] = Some(digit);
                for rem_index in index + 1..self.size {
                    self.values[rem_index] = None;
                }
                return;
            }
        }
    }

    fn get_value(&self) -> u64 {
        self.values
            .iter()
            .map(|v| v.expect("value should be filled") as u64)
            .fold(0, |acc, val| acc * 10 + val)
    }
}

fn compute_voltage(battery_line: &BatteryLine, size: usize) -> u64 {
    let length = battery_line.0.len();
    battery_line
        .0
        .iter()
        .enumerate()
        .fold(VoltageLoop::new(size), |mut acc, (index, digit)| {
            acc.update(*digit, length - index);
            acc
        })
        .get_value()
}

fn parse_line(line: &str) -> Result<BatteryLine> {
    line.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|digit| digit as u8)
                .ok_or(anyhow!("could not parse digit {}", c))
        })
        .collect::<Result<Vec<u8>>>()
        .map(BatteryLine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "357");
        assert_eq!(&part2, "3121910778619");
    }
}
