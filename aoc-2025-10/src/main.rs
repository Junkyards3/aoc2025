use anyhow::Result;
use anyhow::anyhow;
use pathfinding::prelude::dijkstra;
use std::collections::BTreeSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Indicators {
    list: BTreeSet<usize>,
}

impl Indicators {
    fn new(list: Vec<usize>) -> Self {
        Self {
            list: BTreeSet::from_iter(list),
        }
    }

    fn apply(&self, to_apply: &Indicators) -> Indicators {
        let mut new = self.clone();
        for switch in to_apply.list.iter() {
            if new.list.contains(switch) {
                new.list.remove(switch);
            } else {
                new.list.insert(*switch);
            }
        }
        new
    }
}

struct Machine {
    target: Indicators,
    buttons: Vec<Indicators>,
    joltage: Vec<usize>,
}

impl Machine {
    fn find_shortest_button_press(&self) -> usize {
        let origin = Indicators::new(vec![]);
        dijkstra(
            &origin,
            |node| {
                self.buttons
                    .iter()
                    .map(|button| (node.apply(button), 1))
                    .collect::<Vec<_>>()
            },
            |node| *node == self.target,
        )
        .unwrap()
        .0
        .len()
            - 1
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let machines = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&machines);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&machines);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| machine.find_shortest_button_press())
        .sum()
}

fn part2(machines: &[Machine]) -> u128 {
    0
}

fn parse_file(path: &str) -> Result<Vec<Machine>> {
    let file = File::open(path)?;
    let machines: Vec<Machine> = BufReader::new(file)
        .lines()
        .map(|res_line| {
            res_line
                .map_err(|_| anyhow!("could not read line"))
                .and_then(|line| parse_line(&line))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(machines)
}

fn parse_line(line: &str) -> Result<Machine> {
    let mut target: Option<Indicators> = None;
    let mut buttons: Vec<Indicators> = vec![];
    let mut joltage: Option<Vec<usize>> = None;

    for word in line.split_whitespace() {
        if word.starts_with('[') {
            target = Some(parse_target(&word[1..word.len() - 1])?)
        } else if word.starts_with('(') {
            buttons.push(parse_button(&word[1..word.len() - 1])?)
        } else if word.starts_with('{') {
            joltage = Some(parse_voltage(&word[1..word.len() - 1])?)
        }
    }
    let target = target.ok_or(anyhow!("did not find target"))?;
    let joltage = joltage.ok_or(anyhow!("did not find joltage"))?;
    Ok(Machine {
        target,
        buttons,
        joltage,
    })
}

fn parse_target(word: &str) -> Result<Indicators> {
    let indicators = word
        .char_indices()
        .filter(|(_, ch)| *ch == '#')
        .map(|(pos, _)| pos)
        .collect();
    Ok(Indicators::new(indicators))
}

fn parse_button(word: &str) -> Result<Indicators> {
    let indicators = word
        .split(',')
        .map(|sub_word| {
            sub_word
                .parse::<usize>()
                .map_err(|_| anyhow!("could not parse light {sub_word}"))
        })
        .collect::<Result<Vec<_>>>();
    Ok(Indicators::new(indicators?))
}

fn parse_voltage(word: &str) -> Result<Vec<usize>> {
    let voltage = word
        .split(',')
        .map(|sub_word| {
            sub_word
                .parse::<usize>()
                .map_err(|_| anyhow!("could not parse light {sub_word}"))
        })
        .collect::<Result<Vec<_>>>();
    Ok(voltage?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "7");
        assert_eq!(&part2, "0");
    }
}
