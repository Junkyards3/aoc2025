use anyhow::{Result, anyhow};

enum RangeFusionResult {
    Left(FoodRange),
    Right(FoodRange),
    Fused(FoodRange),
}

enum RangeContainResult {
    Left,
    Right,
    Inside,
}

#[derive(Debug, Clone, Copy)]
struct FoodRange {
    lower: u64,
    upper: u64,
}

impl FoodRange {
    fn new(lower: u64, upper: u64) -> Self {
        FoodRange { lower, upper }
    }

    fn fuse_with(&self, range: FoodRange) -> RangeFusionResult {
        if range.upper < self.lower {
            RangeFusionResult::Left(range)
        } else if self.upper < range.lower {
            RangeFusionResult::Right(range)
        } else {
            RangeFusionResult::Fused(FoodRange {
                lower: self.lower.min(range.lower),
                upper: self.upper.max(range.upper),
            })
        }
    }

    fn contains(&self, id: u64) -> RangeContainResult {
        if id < self.lower {
            RangeContainResult::Left
        } else if id > self.upper {
            RangeContainResult::Right
        } else {
            RangeContainResult::Inside
        }
    }

    fn size(&self) -> u64 {
        self.upper - self.lower + 1
    }
}

#[derive(Debug)]
struct NodeRange {
    value: FoodRange,
    left: RangeTree,
    right: RangeTree,
}

impl NodeRange {
    fn new(range: FoodRange) -> Self {
        Self {
            value: range,
            left: RangeTree { node: None },
            right: RangeTree { node: None },
        }
    }

    fn insert(&mut self, range: FoodRange) {
        match self.value.fuse_with(range) {
            RangeFusionResult::Left(food_range) => self.left.insert(food_range),
            RangeFusionResult::Right(food_range) => self.right.insert(food_range),
            RangeFusionResult::Fused(food_range) => {
                self.value = food_range;
                self.rebalance();
            }
        }
    }

    fn rebalance(&mut self) {
        while let Some(node) = self.left.node.as_mut() {
            if let RangeFusionResult::Fused(food_range) = node.value.fuse_with(self.value) {
                self.value = food_range;
                self.left = RangeTree {
                    node: node.left.node.take(),
                }
            } else {
                break;
            }
        }

        while let Some(node) = self.right.node.as_mut() {
            if let RangeFusionResult::Fused(food_range) = node.value.fuse_with(self.value) {
                self.value = food_range;
                self.right = RangeTree {
                    node: node.right.node.take(),
                }
            } else {
                break;
            }
        }
    }

    fn contains(&self, id: u64) -> bool {
        match self.value.contains(id) {
            RangeContainResult::Left => self.left.contains(id),
            RangeContainResult::Right => self.right.contains(id),
            RangeContainResult::Inside => true,
        }
    }

    fn size(&self) -> u64 {
        self.value.size() + self.left.size() + self.right.size()
    }
}

#[derive(Debug)]
struct RangeTree {
    node: Option<Box<NodeRange>>,
}

impl RangeTree {
    fn new() -> Self {
        RangeTree { node: None }
    }

    fn insert(&mut self, range: FoodRange) {
        match &mut self.node {
            Some(node) => node.insert(range),
            None => self.node = Some(Box::new(NodeRange::new(range))),
        }
    }

    fn contains(&self, id: u64) -> bool {
        self.node.as_ref().is_some_and(|node| node.contains(id))
    }

    fn size(&self) -> u64 {
        self.node.as_ref().map(|node| node.size()).unwrap_or(0)
    }
}
fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

fn run(path: &str) -> Result<(String, String)> {
    let (tree, ids) = parse_file(path)?;

    let part1 = part1(&tree, &ids);

    let part2 = part2(&tree);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(tree: &RangeTree, ids: &[u64]) -> usize {
    ids.iter().filter(|id| tree.contains(**id)).count()
}

fn part2(tree: &RangeTree) -> u64 {
    tree.size()
}

fn parse_file(path: &str) -> Result<(RangeTree, Vec<u64>)> {
    let content = std::fs::read_to_string(path)?;
    let (ranges, ids_str) = content
        .split_once("\n\n")
        .ok_or(anyhow!("does not contain double line jump"))?;
    let mut tree = RangeTree::new();

    for line in ranges.lines() {
        let food_range = parse_line_range(line)?;
        tree.insert(food_range);
        dbg!(&tree);
    }

    let ids: Vec<u64> = ids_str
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|_| anyhow!("could not parse id {line}"))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((tree, ids))
}

fn parse_line_range(line: &str) -> Result<FoodRange> {
    let (lower_str, upper_str) = line
        .split_once('-')
        .ok_or(anyhow!("could not find - in range line"))?;
    let lower = lower_str.parse::<u64>()?;
    let upper = upper_str.parse::<u64>()?;
    Ok(FoodRange::new(lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "3");
        assert_eq!(&part2, "14");
    }
}
