use anyhow::Result;
use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u128, u128, u128);
impl Point {
    fn norm(&self) -> u128 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn distance(&self, point: &Point) -> u128 {
        Point(
            self.0.abs_diff(point.0),
            self.1.abs_diff(point.1),
            self.2.abs_diff(point.2),
        )
        .norm()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first: usize,
    second: usize,
    dist: u128,
}

impl Pair {
    fn new(point1: &Point, point2: &Point, index1: usize, index2: usize) -> Pair {
        Pair {
            first: index1,
            second: index2,
            dist: point1.distance(point2),
        }
    }
}

struct Kruskal {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl Kruskal {
    fn with_capacity(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut j = i;
        while self.parents[j] != j {
            self.parents[j] = self.parents[self.parents[j]];
            j = self.parents[j];
        }
        j
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        if self.sizes[x] >= self.sizes[y] {
            self.parents[y] = x;
            self.sizes[x] += self.sizes[y];
        } else {
            self.parents[x] = y;
            self.sizes[y] += self.sizes[x];
        }
    }

    fn size_forests(&self) -> Vec<usize> {
        self.parents
            .iter()
            .enumerate()
            .filter(|(parent, x)| **x == *parent)
            .filter_map(|(_, x)| Some(self.sizes[*x]).take_if(|v| *v != 1))
            .collect()
    }
}

struct Network {
    points: Vec<Point>,
    edges: Vec<Pair>,
    count_pairs: usize,
}

impl Network {
    fn new(points: Vec<Point>, count_pairs: usize) -> Self {
        let mut edges: Vec<Pair> = (0..points.len())
            .flat_map(|i| {
                (i + 1..points.len())
                    .map(|j| Pair::new(&points[i], &points[j], i, j))
                    .collect::<Vec<_>>()
            })
            .collect();
        edges.sort_by_key(|pair| pair.dist);

        Network {
            points,
            edges,
            count_pairs,
        }
    }

    fn kruskal_limited_part1(&self) -> usize {
        let mut kruskal = Kruskal::with_capacity(self.points.len());

        for edge in self.edges[0..self.count_pairs].iter() {
            let x = kruskal.find(edge.first);
            let y = kruskal.find(edge.second);

            if x != y {
                kruskal.union(x, y);
            }
        }
        let mut size_forests = kruskal.size_forests();
        size_forests.sort();
        size_forests.iter().rev().take(3).product()
    }

    fn kruskal_full_part2(&self) -> Result<u128> {
        let mut kruskal = Kruskal::with_capacity(self.points.len());
        let mut edge_count = 0;

        for edge in self.edges.iter() {
            let x = kruskal.find(edge.first);
            let y = kruskal.find(edge.second);

            if x != y {
                kruskal.union(x, y);
                edge_count += 1;
            }

            if edge_count == self.points.len() - 1 {
                return Ok(self.points[edge.first].0 * self.points[edge.second].0);
            }
        }
        Err(anyhow!("graph does not have a spanning tree"))
    }
}

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let now = Instant::now();
    let network = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&network);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&network);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(network: &Network) -> usize {
    network.kruskal_limited_part1()
}

fn part2(network: &Network) -> u128 {
    network
        .kruskal_full_part2()
        .expect("graph does not have a spanning tree")
}

fn parse_file(path: &str) -> Result<Network> {
    let file = File::open(path)?;
    let count_pairs = if path.contains("test") { 10 } else { 1000 };
    let points = BufReader::new(file)
        .lines()
        .map(|res_line| {
            res_line
                .map_err(|_| anyhow!("could not parse line"))
                .and_then(|line| {
                    let res_coords: Result<Vec<u128>> = line
                        .split(',')
                        .map(|comp| {
                            comp.parse::<u128>()
                                .map_err(|_| anyhow!("could not parse number"))
                        })
                        .collect();
                    res_coords.map(|coords| Point(coords[0], coords[1], coords[2]))
                })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Network::new(points, count_pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "40");
        assert_eq!(&part2, "25272");
    }
}
