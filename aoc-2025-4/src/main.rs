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

#[derive(Clone)]
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

    fn get_pos_to_remove(&self) -> Vec<(usize, usize)> {
        (0..self.0.len() as isize)
            .cartesian_product(0..self.0[0].len() as isize)
            .filter(|&idx| {
                let is_paper = matches!(self.get_at(idx), Some(Spot::Paper));
                if !is_paper {
                    return false;
                }
                let neighbors_paper_count = get_neighbors(idx)
                    .iter()
                    .filter(|&&idx| matches!(self.get_at(idx), Some(Spot::Paper)))
                    .count();
                neighbors_paper_count <= 3
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect::<Vec<_>>()
    }

    fn remove_papers(&self, pos: Vec<(usize, usize)>) -> Grid {
        let mut new_grid_content = self.0.clone();
        for &(x, y) in pos.iter() {
            new_grid_content[x][y] = Spot::Empty;
        }
        Grid(new_grid_content)
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
    let mut total_removed = 0;
    let mut curr_grid = grid.clone();
    loop {
        let pos_to_remove = curr_grid.get_pos_to_remove();
        if pos_to_remove.is_empty() {
            break;
        }

        total_removed += pos_to_remove.len();
        curr_grid = curr_grid.remove_papers(pos_to_remove);
    }
    total_removed as u64
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
        assert_eq!(&part2, "43");
    }
}
