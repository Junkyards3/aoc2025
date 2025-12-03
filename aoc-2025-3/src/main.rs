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
        .map(|battery_line| compute_voltage(battery_line))
        .sum()
}

fn part2(battery_lines: &[BatteryLine]) -> u64 {
    0
}

struct VoltageLoop {
    first: Option<u8>,
    second: Option<u8>,
}

impl VoltageLoop {
    fn new() -> Self {
        VoltageLoop {
            first: None,
            second: None,
        }
    }

    fn update(&self, digit: u8, is_last: bool) -> VoltageLoop {
        if !is_last && self.first.is_none_or(|first| first < digit) {
            VoltageLoop {
                first: Some(digit),
                second: None,
            }
        } else if self.second.is_none_or(|sec| sec < digit) {
            VoltageLoop {
                first: self.first,
                second: Some(digit),
            }
        } else {
            VoltageLoop {
                first: self.first,
                second: self.second,
            }
        }
    }

    fn get_value(&self) -> u64 {
        self.first.expect("should have first value") as u64 * 10
            + self.second.expect("should have second value") as u64
    }
}

fn compute_voltage(battery_line: &BatteryLine) -> u64 {
    let length = battery_line.0.len();
    battery_line
        .0
        .iter()
        .enumerate()
        .fold(VoltageLoop::new(), |acc, (index, digit)| {
            acc.update(*digit, index == length - 1)
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
        assert_eq!(&part2, "0");
    }
}
