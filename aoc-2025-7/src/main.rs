use anyhow::{Result, anyhow};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
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

    fn final_line(&self) -> usize {
        self.splitters
            .iter()
            .filter_map(|splitters_col| {
                splitters_col
                    .last_key_value()
                    .map(|(last_line, _)| *last_line)
            })
            .max()
            .unwrap()
            + 1
    }

    fn get_timelines_count_all(&self) -> usize {
        let mut already_computed = HashMap::new();
        let mut final_count = 0;
        let final_line = self.final_line();
        for col in 0..self.splitters.len() {
            final_count +=
                self.get_timelines_count_memoized((final_line, col), &mut already_computed);
        }
        final_count
    }

    fn get_timelines_count_memoized(
        &self,
        destination: (usize, usize),
        already_computed: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(result) = already_computed.get(&destination) {
            *result
        } else {
            let mut count = 0;
            let (line, col) = destination;
            let min_line = self.splitters[col]
                .range(..line)
                .next_back()
                .map(|(splitter_line, _)| *splitter_line)
                .unwrap_or(0);

            if col > 0 {
                count = self.splitters[col - 1]
                    .range(min_line..line)
                    .map(|(splitter_left_line, _)| {
                        self.get_timelines_count_memoized(
                            (*splitter_left_line, col - 1),
                            already_computed,
                        )
                    })
                    .sum::<usize>();
            }

            if col < self.splitters.len() - 1 {
                count += self.splitters[col + 1]
                    .range(min_line..line)
                    .map(|(splitter_right_line, _)| {
                        self.get_timelines_count_memoized(
                            (*splitter_right_line, col + 1),
                            already_computed,
                        )
                    })
                    .sum::<usize>();
            }

            if min_line == 0 && col == self.source_col {
                count += 1
            }

            already_computed.insert(destination, count);
            count
        }
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let manifold = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&manifold);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&manifold);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(manifold: &Manifold) -> usize {
    manifold.clone().run_split()
}

fn part2(manifold: &Manifold) -> usize {
    manifold.get_timelines_count_all()
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
        assert_eq!(&part2, "40");
    }
}
