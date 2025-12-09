use anyhow::Result;
use anyhow::anyhow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(u64, u64);

impl Point {
    fn area(&self, point: &Point) -> u64 {
        (point.0.abs_diff(self.0) + 1) * (point.1.abs_diff(self.1) + 1)
    }

    fn get_other_corners(&self, point: &Point) -> (Point, Point) {
        (Point(self.0, point.1), Point(point.0, self.1))
    }
}

enum PositionToWall {
    Minus,
    On,
    Plus,
}

#[derive(Debug)]
struct YWall {
    x1: u64,
    x2: u64,
}

impl YWall {
    fn new(x1: u64, x2: u64) -> Self {
        if x1 < x2 {
            Self { x1, x2 }
        } else {
            Self { x1: x2, x2: x1 }
        }
    }

    fn get_position(&self, point: Point) -> PositionToWall {
        let x = point.0;
        if x < self.x1 {
            PositionToWall::Minus
        } else if x > self.x2 {
            PositionToWall::Plus
        } else {
            PositionToWall::On
        }
    }

    fn intersects(&self, x: u64) -> bool {
        (self.x1..=self.x2).contains(&x)
    }

    fn intersects_middle(&self, x: u64) -> bool {
        (self.x1 + 1..self.x2).contains(&x)
    }
}

#[derive(Debug)]
struct XWall {
    y1: u64,
    y2: u64,
}

impl XWall {
    fn new(y1: u64, y2: u64) -> Self {
        if y1 < y2 {
            Self { y1, y2 }
        } else {
            Self { y1: y2, y2: y1 }
        }
    }

    fn get_position(&self, point: Point) -> PositionToWall {
        let y = point.1;
        if y < self.y1 {
            PositionToWall::Minus
        } else if y > self.y2 {
            PositionToWall::Plus
        } else {
            PositionToWall::On
        }
    }

    fn intersects(&self, y: u64) -> bool {
        (self.y1..=self.y2).contains(&y)
    }

    fn intersects_middle(&self, y: u64) -> bool {
        (self.y1 + 1..self.y2).contains(&y)
    }
}

#[derive(Debug)]
struct Walls {
    x_left: u64,
    x_right: u64,
    x_walls: BTreeMap<u64, XWall>,
    y_walls: BTreeMap<u64, YWall>,
}

impl Walls {
    fn is_inside(&self, point: Point) -> bool {
        let x = point.0;
        let y = point.1;
        if x <= self.x_left || x >= self.x_right {
            return false;
        }
        if let Some(PositionToWall::On) = self
            .x_walls
            .get(&x)
            .map(|x_wall| x_wall.get_position(point))
        {
            return true;
        }

        let ray_cast_minus = if let Some(y_wall) = self.y_walls.get(&y) {
            match y_wall.get_position(point) {
                PositionToWall::Minus => true,
                PositionToWall::On => return true,
                PositionToWall::Plus => false,
            }
        } else {
            true
        };

        if ray_cast_minus {
            self.x_walls
                .range(self.x_left..x)
                .filter(|(_, x_wall)| x_wall.intersects(y))
                .count()
                % 2
                == 1
        } else {
            self.x_walls
                .range(x..self.x_right)
                .filter(|(_, x_wall)| x_wall.intersects(y))
                .count()
                % 2
                == 1
        }
    }

    fn intersects_segments(&self, corner1: Point, corner2: Point) -> bool {
        let (min_x, max_x) = if corner1.0 < corner2.0 {
            (corner1.0, corner2.0)
        } else {
            (corner2.0, corner1.0)
        };

        let (min_y, max_y) = if corner1.1 < corner2.1 {
            (corner1.1, corner2.1)
        } else {
            (corner2.1, corner1.1)
        };

        let x_intersect = min_x + 1 < max_x
            && self.x_walls.range(min_x + 1..max_x - 1).any(|(_, x_wall)| {
                x_wall.intersects_middle(min_y) || x_wall.intersects_middle(max_y)
            });

        if x_intersect {
            return true;
        }
        min_y + 1 < max_y
            && self.y_walls.range(min_y + 1..max_y - 1).any(|(_, y_wall)| {
                y_wall.intersects_middle(min_x) || y_wall.intersects_middle(max_x)
            })
    }
}

struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn new(points: Vec<Point>) -> Self {
        Grid { points }
    }

    fn get_max_area(&self) -> u64 {
        (0..self.points.len())
            .map(|i| {
                (i + 1..self.points.len())
                    .map(|j| self.points[i].area(&self.points[j]))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }

    fn get_walls(&self) -> Walls {
        let mut points_by_x: HashMap<u64, Point> = HashMap::with_capacity(self.points.len());
        let mut points_by_y: HashMap<u64, Point> = HashMap::with_capacity(self.points.len());

        let mut x_walls = BTreeMap::new();
        let mut y_walls = BTreeMap::new();

        let mut x_left = u64::MAX;
        let mut x_right = 0;

        for point in self.points.iter() {
            x_left = x_left.min(point.0);
            x_right = x_right.max(point.0);

            if let Some((x, other_point)) = points_by_x.remove_entry(&point.0) {
                x_walls.insert(x, XWall::new(point.1, other_point.1));
            } else {
                points_by_x.insert(point.0, *point);
            }

            if let Some((y, other_point)) = points_by_y.remove_entry(&point.1) {
                y_walls.insert(y, YWall::new(point.0, other_point.0));
            } else {
                points_by_y.insert(point.1, *point);
            }
        }

        x_left -= 1;
        x_right += 1;

        Walls {
            x_left,
            x_right,
            x_walls,
            y_walls,
        }
    }

    fn get_max_area_inside(&self) -> (Point, Point, u64) {
        let walls = self.get_walls();
        (0..self.points.len())
            .map(|i| {
                (i + 1..self.points.len())
                    .map(|j| (self.points[i], self.points[j]))
                    .filter(|(point1, point2)| {
                        let (point3, point4) = point1.get_other_corners(point2);
                        point1.0 != point2.0
                            && point1.1 != point2.1
                            && walls.is_inside(*point1)
                            && walls.is_inside(*point2)
                            && walls.is_inside(point3)
                            && walls.is_inside(point4)
                            && !walls.intersects_segments(*point1, *point2)
                    })
                    .map(|(point1, point2)| (point1, point2, point1.area(&point2)))
                    .max_by_key(|(_, _, v)| *v)
                    .unwrap_or((Point(0, 0), Point(0, 0), 0))
            })
            .max_by_key(|(_, _, v)| *v)
            .unwrap_or((Point(0, 0), Point(0, 0), 0))
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
    grid.get_max_area()
}

fn part2(grid: &Grid) -> u64 {
    grid.get_max_area_inside().2
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
        assert_eq!(&part2, "24");
    }

    #[test]
    fn test_corners() {
        let grid = parse_file("./files/test.txt").expect("could not run");
        let walls = grid.get_walls();
        let (p1, p2, _v) = grid.get_max_area_inside();
        for x in p1.0..p2.0 {
            assert!(
                walls.is_inside(Point(x, p1.1)),
                "{x} {} not inside !!",
                p1.1
            );
            assert!(
                walls.is_inside(Point(x, p2.1)),
                "{x} {} not inside !!",
                p2.1
            );
        }
        for y in p2.1..p1.1 {
            assert!(
                walls.is_inside(Point(p1.0, y)),
                "{} {y} not inside !!",
                p1.0
            );
            assert!(
                walls.is_inside(Point(p2.0, y)),
                "{} {y} not inside !!",
                p2.0
            );
        }
    }

    #[test]
    fn test_specific_corners() {
        let grid = parse_file("./files/test.txt").expect("could not run");
        let walls = grid.get_walls();
        assert!(walls.is_inside(Point(2, 4)));
    }
}
