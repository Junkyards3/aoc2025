use anyhow::{Result, anyhow};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn get_neighbors_pos(&self) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        vec![
            Pos { x: x - 1, y: y - 1 },
            Pos { x, y: y - 1 },
            Pos { x: x + 1, y: y - 1 },
            Pos { x: x - 1, y },
            Pos { x: x + 1, y },
            Pos { x: x - 1, y: y + 1 },
            Pos { x, y: y + 1 },
            Pos { x: x + 1, y: y + 1 },
        ]
    }
}

#[derive(Clone)]
struct Status {
    neighbors_count: u8,
}

#[derive(Clone)]
struct Grid {
    map: HashMap<Pos, Status>,
    marked_for_deletion: HashSet<Pos>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            map: HashMap::new(),
            marked_for_deletion: HashSet::new(),
        }
    }

    fn add(&mut self, pos: Pos) {
        let neighbors = self.get_neighbors(pos);
        for neighbor in neighbors.iter() {
            self.map
                .entry(*neighbor)
                .and_modify(|status| status.neighbors_count += 1);

            if self.map.get(neighbor).unwrap().neighbors_count >= 4 {
                self.marked_for_deletion.remove(neighbor);
            }
        }
        self.map.insert(
            pos,
            Status {
                neighbors_count: neighbors.len() as u8,
            },
        );

        if neighbors.len() < 4 {
            self.marked_for_deletion.insert(pos);
        }
    }

    fn get_neighbors(&self, pos: Pos) -> Vec<Pos> {
        pos.get_neighbors_pos()
            .into_iter()
            .filter(|neighbor_pos| self.map.contains_key(neighbor_pos))
            .collect()
    }

    fn remove_papers_once(&mut self) {
        let mut new_marked_for_deletion = HashSet::new();
        for pos in self.marked_for_deletion.clone().iter() {
            new_marked_for_deletion.extend(self.remove(*pos));
        }
        new_marked_for_deletion.retain(|pos| !self.marked_for_deletion.contains(pos));
        self.marked_for_deletion = new_marked_for_deletion;
    }

    fn remove(&mut self, pos: Pos) -> Vec<Pos> {
        let neighbors = self.get_neighbors(pos);
        let mut marked_for_deletion = vec![];
        for neighbor in neighbors.iter() {
            self.map.entry(*neighbor).and_modify(|status| {
                status.neighbors_count = status.neighbors_count.saturating_sub(1);
            });
            if self.map.get(neighbor).unwrap().neighbors_count < 4 {
                marked_for_deletion.push(*neighbor);
            }
        }
        self.map.remove(&pos);
        marked_for_deletion
    }

    fn size(&self) -> usize {
        self.map.len()
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let file = File::open(path)?;
    let mut grid = Grid::new();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.map_err(|_| anyhow!("could not read line")))
        .collect::<Result<Vec<String>>>()?;
    for (x, line) in lines.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid.add(Pos {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&grid);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&grid);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let init_size = grid.size();
    grid.remove_papers_once();
    init_size - grid.size()
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let init_size = grid.size();
    loop {
        grid.remove_papers_once();
        if grid.marked_for_deletion.is_empty() {
            break;
        }
    }
    init_size - grid.size()
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
