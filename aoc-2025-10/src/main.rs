use anyhow::Result;
use anyhow::anyhow;
use good_lp::Constraint;
use good_lp::Expression;
use good_lp::ProblemVariables;
use good_lp::Solution;
use good_lp::SolverModel;
use good_lp::scip;
use good_lp::variable;
use std::collections::BTreeSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Indicators {
    list: BTreeSet<usize>,
}

impl Indicators {
    fn new(list: Vec<usize>) -> Self {
        Self {
            list: BTreeSet::from_iter(list),
        }
    }
}

struct Machine {
    target: Indicators,
    buttons: Vec<Indicators>,
    joltage: Vec<usize>,
}

impl Machine {
    fn find_shortest_button_press(&self) -> Result<f64> {
        let mut problem = ProblemVariables::new();
        let but_vars = problem.add_vector(variable().integer().min(0).max(1), self.buttons.len());
        let eveness_vars = problem.add_vector(variable().integer().min(0), self.joltage.len());

        let mut obj = Expression::from(0);

        //minimise sum of button presses
        for var in but_vars.iter() {
            obj.add_mul(1, var);
        }

        //add wanted constraints
        let mut constraints = vec![Expression::from(0); self.joltage.len()];

        for (button, var) in self.buttons.iter().zip(but_vars.iter()) {
            for pos in button.list.iter() {
                constraints[*pos].add_mul(1, var);
            }
        }

        let constraints: Vec<Constraint> = constraints
            .into_iter()
            .zip(eveness_vars.iter())
            .enumerate()
            .map(|(pos, (constraint, e_var))| {
                if self.target.list.contains(&pos) {
                    Expression::eq(constraint, *e_var * 2 + 1)
                } else {
                    Expression::eq(constraint, *e_var * 2)
                }
            })
            .collect();

        //evaluate sum of button presses
        Ok(problem
            .minimise(&obj)
            .using(scip)
            .with_all(constraints)
            .solve()?
            .eval(obj))
    }

    fn find_shortest_button_press_joltage(&self) -> Result<f64> {
        let mut problem = ProblemVariables::new();
        let but_vars = problem.add_vector(variable().integer().min(0), self.buttons.len());

        let mut obj = Expression::from(0);

        //minimise sum of button presses
        for var in but_vars.iter() {
            obj.add_mul(1, var);
        }

        //add wanted constraints
        let mut constraints = vec![Expression::from(0); self.joltage.len()];

        for (button, var) in self.buttons.iter().zip(but_vars.iter()) {
            for pos in button.list.iter() {
                constraints[*pos].add_mul(1, var);
            }
        }

        let constraints: Vec<Constraint> = constraints
            .into_iter()
            .enumerate()
            .map(|(pos, constraint)| Expression::eq(constraint, self.joltage[pos] as u32))
            .collect();

        //evaluate sum of button presses
        Ok(problem
            .minimise(&obj)
            .using(scip)
            .with_all(constraints)
            .solve()?
            .eval(obj))
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
        .map(|machine| {
            machine
                .find_shortest_button_press()
                .expect("could not solve machine") as usize
        })
        .sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| {
            machine
                .find_shortest_button_press_joltage()
                .expect("could not solve machine") as usize
        })
        .sum()
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
    word.split(',')
        .map(|sub_word| {
            sub_word
                .parse::<usize>()
                .map_err(|_| anyhow!("could not parse light {sub_word}"))
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "7");
        assert_eq!(&part2, "33");
    }
}
