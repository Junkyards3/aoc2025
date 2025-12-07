use anyhow::{Result, anyhow};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Manifold {
    source_col: usize,
    splitters: Vec<BTreeMap<usize, bool>>,
}

impl Manifold {
    fn run_split(&mut self) -> usize {
        let mut rays = Vec::with_capacity(self.splitters_count());
        rays.push((0, self.source_col));
        while let Some((ray_line, ray_col)) = rays.pop() {
            if let Some((next_splitter_line, has_split)) =
                self.splitters[ray_col].range_mut(ray_line..).next()
                && !*has_split
            {
                *has_split = true;
                rays.push((*next_splitter_line, ray_col - 1));
                rays.push((*next_splitter_line, ray_col + 1));
            }
        }
        self.splitters_split_count()
    }

    fn splitters_count(&self) -> usize {
        self.splitters
            .iter()
            .map(|splitters_col| splitters_col.len())
            .sum()
    }

    fn splitters_split_count(&self) -> usize {
        self.splitters
            .iter()
            .map(|splitters_col| {
                splitters_col
                    .iter()
                    .filter(|(_, has_split)| **has_split)
                    .count()
            })
            .sum()
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let manifold = parse_file(path)?;
    let part1 = part1(&manifold);
    let part2 = part2(&manifold);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(manifold: &Manifold) -> usize {
    manifold.clone().run_split()
}

fn part2(manifold: &Manifold) -> usize {
    0
}

fn parse_file(path: &str) -> Result<Manifold> {
    let mut source_col: usize = 0;
    let mut splitters: Vec<BTreeMap<usize, bool>> = vec![];
    let file = File::open(path)?;
    for (line_idx, line) in BufReader::new(file).lines().enumerate() {
        for (col_idx, ch) in line?.char_indices() {
            if line_idx == 0 {
                splitters.push(BTreeMap::new());
            }
            match ch {
                'S' => {
                    source_col = col_idx;
                }
                '^' => {
                    splitters[col_idx].insert(line_idx, false);
                }
                '.' => {}
                ch => return Err(anyhow!("unexpected char {ch}")),
            }
        }
    }
    Ok(Manifold {
        source_col,
        splitters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "21");
        assert_eq!(&part2, "0");
    }
}
