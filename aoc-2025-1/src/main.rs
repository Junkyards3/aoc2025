use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let file = File::open(path)?;
    let numbers: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|s| parse_number(s?.as_str()))
        .collect::<Result<Vec<_>>>()?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&numbers);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&numbers);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(numbers: &[i32]) -> usize {
    numbers
        .iter()
        .scan(50, |state, val| {
            *state = (*state + *val).rem_euclid(100);
            Some(*state)
        })
        .filter(|pos| *pos == 0)
        .count()
}

fn part2(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .scan(50, |state, val| {
            let (new_pos, by_zero) = get_by_zero(*state, *val);
            *state = new_pos;
            Some(by_zero)
        })
        .sum()
}

fn get_by_zero(pos: i32, turn: i32) -> (i32, i32) {
    let quot = (pos + turn).div_euclid(100);
    let new_pos = (pos + turn) - quot * 100;

    let by_zero = if new_pos == 0 && quot <= 0 {
        -quot + 1
    } else if pos == 0 && quot < 0 {
        -(quot + 1)
    } else {
        quot.abs()
    };

    (new_pos, by_zero)
}

fn parse_number(line: &str) -> Result<i32> {
    if let Some(negative) = line.strip_prefix("L") {
        Ok(negative.parse::<i32>().map(|nb| -nb)?)
    } else if let Some(positive) = line.strip_prefix("R") {
        Ok(positive.parse::<i32>()?)
    } else {
        Err(anyhow!("did not start with L or R : {}", &line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "3");
        assert_eq!(&part2, "6");
    }

    #[test]
    fn test_get_by_zero_exact() {
        let (new_pos, by_zero) = get_by_zero(50, 150);

        assert_eq!(by_zero, 2);
        assert_eq!(new_pos, 0)
    }

    proptest! {
        #[test]
        fn test_get_by_zero(pos in 0..99i32, step in -1000..1000i32) {
            if step == 0 {
                return Ok(())
            }
            let (new_pos, by_zero) = get_by_zero(pos, step);

            let min_step = if step < 0 {
                -1
            } else {
                1
            };

            let exp_by_zero = (1..step.abs()+1)
                .map(|x| pos + x * min_step)
                .filter(|int_pos| {
                    let ret = int_pos.rem_euclid(100) == 0;
                    if ret {
                        dbg!(int_pos);
                    }
                    ret
                })
                .count();

            assert_eq!(by_zero, exp_by_zero as i32, "should have crossed zero the same number of times");
            assert_eq!((pos + step - new_pos).rem_euclid(100), 0, "should have landed on the same pos")
        }
    }
}
