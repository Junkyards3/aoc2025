use anyhow::Result;
use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(u64, u64);

impl Point {
    fn area(&self, point: &Point) -> u64 {
        (point.0.saturating_sub(self.0) + 1) * (point.1.saturating_sub(self.1) + 1)
    }
}

#[derive(Debug)]
struct Corners {
    upper_left: Vec<Point>,
    upper_right: Vec<Point>,
    lower_left: Vec<Point>,
    lower_right: Vec<Point>,
}

impl Corners {
    fn get_max_area(&self) -> u64 {
        let mut curr_max = 0;
        for point1 in self.upper_left.iter() {
            for point2 in self.lower_right.iter() {
                curr_max = curr_max.max(point1.area(point2))
            }
        }
        for point1 in self.upper_right.iter() {
            for point2 in self.lower_left.iter() {
                curr_max = curr_max.max(point1.area(point2))
            }
        }
        curr_max
    }
}

struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn new(mut points: Vec<Point>) -> Self {
        points.sort();
        Grid { points }
    }

    fn get_corners(&self) -> Corners {
        let mut upper_left = vec![];
        let mut upper_right = vec![];
        let mut lower_left = vec![];
        let mut lower_right = vec![];

        //uppers
        let mut left_y = self.points[0].1;
        let mut right_y = 0;
        upper_left.push(self.points[0]);

        for points in self.points.windows(2) {
            let x1 = points[0].0;
            let y1 = points[0].1;
            let x2 = points[1].0;
            let y2 = points[1].1;

            if x1 != x2 {
                //second point is first point of next line
                //first point is last point of previous line
                if y1 > right_y {
                    right_y = y1;
                    upper_right.push(points[0]);
                }
                if y2 < left_y {
                    left_y = y2;
                    upper_left.push(points[1]);
                }
            }
        }

        //lowers
        let mut left_y = u64::MAX;
        let mut right_y = self.points.last().unwrap().1;
        lower_right.push(*self.points.last().unwrap());

        for points in self.points.windows(2).rev() {
            let x1 = points[0].0;
            let y1 = points[0].1;
            let x2 = points[1].0;
            let y2 = points[1].1;

            if x1 != x2 {
                //second point is last point of line below
                //first point is first point of current line
                if y1 > right_y {
                    right_y = y1;
                    lower_right.push(points[0]);
                }
                if y2 < left_y {
                    left_y = y2;
                    lower_left.push(points[1]);
                }
            }
        }
        Corners {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
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
    let grid = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&grid);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&grid);
    println!("duration part 2 : {:?}", now.elapsed());

    Ok((part1.to_string(), part2.to_string()))
}

fn part1(grid: &Grid) -> u64 {
    grid.get_corners().get_max_area()
}

fn part2(grid: &Grid) -> u64 {
    0
}

fn parse_file(path: &str) -> Result<Grid> {
    let file = File::open(path)?;
    let points = BufReader::new(file)
        .lines()
        .map(|res_line| {
            res_line
                .map_err(|_| anyhow!("could not parse line"))
                .and_then(|line| {
                    let res_coords: Result<Vec<u64>> = line
                        .split(',')
                        .map(|comp| {
                            comp.parse::<u64>()
                                .map_err(|_| anyhow!("could not parse number"))
                        })
                        .collect();
                    res_coords.map(|coords| Point(coords[0], coords[1]))
                })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Grid::new(points))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "50");
        assert_eq!(&part2, "0");
    }

    #[test]
    fn test_corners() {
        let grid = parse_file("./files/test.txt").expect("could not run");
        grid.get_corners();
    }
}
