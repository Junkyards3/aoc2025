use std::time::Instant;

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

    fn adapt_new_lower(&mut self, new_lower: u64) -> Option<FoodRange> {
        let new_upper = self.upper.min(new_lower.saturating_sub(1));
        if new_upper < self.lower {
            None
        } else {
            Some(FoodRange {
                lower: self.lower,
                upper: new_upper,
            })
        }
    }

    fn adapt_new_upper(&mut self, new_upper: u64) -> Option<FoodRange> {
        let new_lower = self.lower.max(new_upper.saturating_add(1));
        if new_lower > self.upper {
            None
        } else {
            Some(FoodRange {
                lower: new_lower,
                upper: self.upper,
            })
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
                self.left.push_new_lower(food_range.lower);
                self.right.push_new_upper(food_range.upper);
            }
        }
    }

    fn push_new_lower(&mut self, new_lower: u64) -> bool {
        self.left.push_new_lower(new_lower);
        self.right.push_new_lower(new_lower);
        match self.value.adapt_new_lower(new_lower) {
            Some(adapted) => {
                self.value = adapted;
                true
            }
            None => false,
        }
    }

    fn push_new_upper(&mut self, new_upper: u64) -> bool {
        self.left.push_new_upper(new_upper);
        self.right.push_new_upper(new_upper);
        match self.value.adapt_new_upper(new_upper) {
            Some(adapted) => {
                self.value = adapted;
                true
            }
            None => false,
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

    fn push_new_lower(&mut self, new_lower: u64) {
        if let Some(node) = &mut self.node {
            let is_node_alive = node.push_new_lower(new_lower);
            if !is_node_alive {
                self.node = node.left.node.take();
            }
        }
    }

    fn push_new_upper(&mut self, new_upper: u64) {
        if let Some(node) = &mut self.node {
            let is_node_alive = node.push_new_upper(new_upper);
            if !is_node_alive {
                self.node = node.right.node.take();
            }
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
    let (tree, ids) = parse_file(path)?;
    println!("duration parsing : {:?}", now.elapsed());

    let now = Instant::now();
    let part1 = part1(&tree, &ids);
    println!("duration part 1 : {:?}", now.elapsed());

    let now = Instant::now();
    let part2 = part2(&tree);
    println!("duration part 2 : {:?}", now.elapsed());

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

    #[test]
    fn test_tree_simple() {
        let mut tree = RangeTree::new();
        tree.insert(FoodRange::new(1, 3));
        dbg!("{}", &tree);
        tree.insert(FoodRange::new(11, 13));
        dbg!("{}", &tree);
        tree.insert(FoodRange::new(14, 14));
        dbg!("{}", &tree);
        tree.insert(FoodRange::new(20, 37));
        dbg!("{}", &tree);
        tree.insert(FoodRange::new(5, 10));
        dbg!("{}", &tree);
        tree.insert(FoodRange::new(2, 6));
        dbg!("{}", &tree);
        assert_eq!(tree.size(), 32);
    }

    #[test]
    fn test_tree_structure_example() {
        let tree = parse_file("./files/test.txt").expect("could not parse").0;
        check_tree_structure(&tree);
    }

    #[test]
    fn test_tree_structure() {
        let content = std::fs::read_to_string("./files/input.txt").unwrap();
        let (ranges, _) = content
            .split_once("\n\n")
            .ok_or(anyhow!("does not contain double line jump"))
            .unwrap();
        let mut tree = RangeTree::new();

        for line in ranges.lines() {
            let food_range = parse_line_range(line).unwrap();
            tree.insert(food_range);
            check_tree_structure(&tree);
        }
    }

    fn check_tree_structure(tree: &RangeTree) {
        if let Some(node) = &tree.node {
            let largest_left = get_largest_value(&node.left);
            let smallest_right = get_smallest_value(&node.right);
            match (largest_left, smallest_right) {
                (Some(largest_left), Some(smallest_right)) => {
                    assert!(
                        largest_left < node.value.lower,
                        "tree structure violated {} {}",
                        largest_left,
                        node.value.lower
                    );
                    assert!(
                        smallest_right > node.value.upper,
                        "tree structure violated {} {}",
                        node.value.upper,
                        smallest_right
                    );
                }
                (Some(largest_left), None) => {
                    assert!(
                        largest_left < node.value.lower,
                        "tree structure violated {} {}",
                        node.value.lower,
                        largest_left
                    );
                }
                (None, Some(smallest_right)) => {
                    assert!(
                        smallest_right > node.value.upper,
                        "tree structure violated at node {:?} {} {}",
                        node,
                        node.value.upper,
                        smallest_right
                    );
                }
                (None, None) => {}
            }
            check_tree_structure(&node.left);
            check_tree_structure(&node.right);
        }
    }

    fn get_smallest_value(tree: &RangeTree) -> Option<u64> {
        let mut current = tree;
        while let Some(node) = &current.node {
            if node.left.node.is_none() {
                return Some(node.value.lower);
            }
            current = &node.left;
        }
        None
    }

    fn get_largest_value(tree: &RangeTree) -> Option<u64> {
        let mut current = tree;
        while let Some(node) = &current.node {
            if node.right.node.is_none() {
                return Some(node.value.upper);
            }
            current = &node.right;
        }
        None
    }

    #[test]
    fn test_all_in_tree() {
        let content = std::fs::read_to_string("./files/input.txt").unwrap();
        let (ranges, _) = content
            .split_once("\n\n")
            .ok_or(anyhow!("does not contain double line jump"))
            .unwrap();
        let mut tree = RangeTree::new();
        let mut food_ranges = vec![];

        for line in ranges.lines() {
            let food_range = parse_line_range(line).unwrap();
            food_ranges.push(food_range);
            tree.insert(food_range);
        }

        for range in food_ranges {
            assert!(
                tree.contains(range.lower),
                "tree does not contain lower {}",
                range.lower
            );
            assert!(
                tree.contains(range.upper),
                "tree does not contain upper {}",
                range.upper
            );
        }
    }

    #[test]
    fn test_specific_leaves_tree() {
        let specific = 67073553206566;
        let mut has_seen = false;

        let content = std::fs::read_to_string("./files/input.txt").unwrap();
        let (ranges, _) = content
            .split_once("\n\n")
            .ok_or(anyhow!("does not contain double line jump"))
            .unwrap();
        let mut tree = RangeTree::new();
        let mut food_ranges = vec![];

        for line in ranges.lines() {
            let food_range = parse_line_range(line).unwrap();
            food_ranges.push(food_range);
            tree.insert(food_range);

            if !has_seen && tree.contains(specific) {
                has_seen = true;
            }

            if has_seen {
                assert!(tree.contains(specific))
            }
        }
    }
}
