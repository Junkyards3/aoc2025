use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u128, u128, u128);

fn norm(point: &Point) -> u128 {
    point.0 * point.0 + point.1 * point.1 + point.2 * point.2
}

fn distance(point1: &Point, point2: &Point) -> u128 {
    norm(&Point(
        point1.0.abs_diff(point2.0),
        point1.1.abs_diff(point2.1),
        point1.2.abs_diff(point2.2),
    ))
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first: Point,
    second: Point,
    dist: u128,
}

fn pair_from_points(point1: &Point, point2: &Point) -> Pair {
    Pair {
        first: *point1,
        second: *point2,
        dist: distance(point1, point2),
    }
}

fn insert_sorted(pairs: &mut Vec<Pair>, pair: Pair) {
    match pairs.binary_search_by(|t_pair| t_pair.dist.cmp(&pair.dist)) {
        Ok(pos) => pairs.insert(pos, pair),
        Err(pos) => pairs.insert(pos, pair),
    }
}

struct Network {
    points: Vec<Point>,
    count_pairs: usize,
}

impl Network {
    fn compute_smallest_distances(&self) -> Vec<Pair> {
        let mut current_distances = Vec::with_capacity(self.points.len() * self.points.len() / 2);
        let mut curr_points = Vec::with_capacity(self.points.len());
        let mut curr_max = 0;
        for point in self.points.iter() {
            for already_point in curr_points.iter() {
                let pair = pair_from_points(point, already_point);
                if current_distances.len() < self.count_pairs {
                    curr_max = curr_max.max(pair.dist);
                    insert_sorted(&mut current_distances, pair);
                } else if curr_max > pair.dist {
                    current_distances.pop();
                    insert_sorted(&mut current_distances, pair);
                    curr_max = current_distances.last().unwrap().dist;
                }
            }

            curr_points.push(*point);
        }

        current_distances
    }

    fn find_smallest_distances_circuit_prod(&self) -> usize {
        let pairs = self.compute_smallest_distances();
        let mut circuits: Vec<HashSet<Point>> = vec![];
        //ugly union find
        for pair in pairs {
            let point1 = pair.first;
            let point2 = pair.second;
            let index1 = circuits
                .iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.contains(&point1))
                .map(|(idx, _)| idx)
                .next();
            let index2 = circuits
                .iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.contains(&point2))
                .map(|(idx, _)| idx)
                .next();
            match (index1, index2) {
                (None, None) => {
                    circuits.push(HashSet::from([point1, point2]));
                }
                (None, Some(index2)) => {
                    circuits[index2].insert(point1);
                }
                (Some(index1), None) => {
                    circuits[index1].insert(point2);
                }
                (Some(index1), Some(index2)) => {
                    if index1 != index2 {
                        let points_to_add = circuits[index2].clone();
                        circuits[index1].extend(points_to_add);
                        circuits.remove(index2);
                    }
                }
            }
        }
        let mut circuits_length = circuits
            .iter()
            .map(|circuit| circuit.len())
            .collect::<Vec<_>>();
        circuits_length.sort();
        circuits_length.reverse();
        circuits_length.iter().take(3).product()
    }

    fn find_last_pair_prod(&self) -> usize {
        let pairs = self.compute_smallest_distances();
        let mut circuits: Vec<HashSet<Point>> = vec![];
        //ugly union find
        for pair in pairs {
            let point1 = pair.first;
            let point2 = pair.second;
            let index1 = circuits
                .iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.contains(&point1))
                .map(|(idx, _)| idx)
                .next();
            let index2 = circuits
                .iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.contains(&point2))
                .map(|(idx, _)| idx)
                .next();
            match (index1, index2) {
                (None, None) => {
                    circuits.push(HashSet::from([point1, point2]));
                }
                (None, Some(index2)) => {
                    circuits[index2].insert(point1);
                }
                (Some(index1), None) => {
                    circuits[index1].insert(point2);
                }
                (Some(index1), Some(index2)) => {
                    if index1 != index2 {
                        let points_to_add = circuits[index2].clone();
                        circuits[index1].extend(points_to_add);
                        circuits.remove(index2);
                    }
                }
            }
        }
        let mut circuits_length = circuits
            .iter()
            .map(|circuit| circuit.len())
            .collect::<Vec<_>>();
        circuits_length.sort();
        circuits_length.reverse();
        circuits_length.iter().take(3).product()
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
    network.find_smallest_distances_circuit_prod()
}

fn part2(network: &Network) -> usize {
    network.find_last_pair_prod()
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
    Ok(Network {
        points,
        count_pairs,
    })
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
