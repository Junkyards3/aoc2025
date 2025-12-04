use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Spot {
    Paper,
    Empty,
}

impl Spot {
    fn is_paper(&self) -> bool {
        matches!(self, Spot::Paper)
    }
}
struct Grid(Vec<Vec<Spot>>);

impl Grid {
    fn get_at(&self, index: (isize, isize)) -> Option<Spot> {
        let (x, y) = index;
        if (0..self.0.len() as isize).contains(&x) && (0..self.0[0].len() as isize).contains(&y) {
            Some(self.0[x as usize][y as usize])
        } else {
            None
        }
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let file = File::open(path)?;
    let grid_content = BufReader::new(file)
        .lines()
        .map(|s| parse_line(s?.as_str()))
        .collect::<Result<Vec<_>>>()?;
    let grid = Grid(grid_content);

    let part1 = part1(&grid);
    let part2 = part2(&grid);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(grid: &Grid) -> u64 {
    (0..grid.0.len() as isize)
        .cartesian_product(0..grid.0[0].len() as isize)
        .filter(|&idx| {
            let is_paper = matches!(grid.get_at(idx), Some(Spot::Paper));
            if !is_paper {
                return false;
            }
            let neighbors_paper_count = get_neighbors(idx)
                .iter()
                .filter(|&&idx| matches!(grid.get_at(idx), Some(Spot::Paper)))
                .count();
            neighbors_paper_count <= 3
        })
        .count() as u64
}

fn part2(grid: &Grid) -> u64 {
    0
}

fn get_neighbors(index: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = index;
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn parse_line(line: &str) -> Result<Vec<Spot>> {
    line.chars()
        .map(|c| match c {
            '.' => Ok(Spot::Empty),
            '@' => Ok(Spot::Paper),
            _ => Err(anyhow!("cannot match {}", c)),
        })
        .collect::<Result<Vec<Spot>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "13");
        assert_eq!(&part2, "0");
    }
}
