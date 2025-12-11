use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

struct Network {
    edges: HashMap<String, Vec<String>>,
}

impl Network {
    fn paths_count(&self, source: &str, target: &str) -> usize {
        let mut cache = HashMap::from([(target.to_owned(), 1)]);
        self.paths_count_cached(source, &mut cache)
    }

    fn paths_count_2(&self) -> usize {
        let fft_to_dac_count = self.paths_count("fft", "dac");
        if fft_to_dac_count != 0 {
            self.paths_count("svr", "fft") * fft_to_dac_count * self.paths_count("dac", "out")
        } else {
            self.paths_count("svr", "dac")
                * self.paths_count("dac", "fft")
                * self.paths_count("fft", "out")
        }
    }

    fn paths_count_cached(&self, origin: &str, cache: &mut HashMap<String, usize>) -> usize {
        if let Some(count) = cache.get(origin) {
            return *count;
        }

        let count = if let Some(targets) = self.edges.get(origin) {
            targets
                .iter()
                .map(|target| self.paths_count_cached(target, cache))
                .sum()
        } else {
            0
        };

        cache.insert(origin.to_string(), count);

        count
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
    network.paths_count("you", "out")
}

fn part2(network: &Network) -> usize {
    network.paths_count_2()
}

fn parse_file(path: &str) -> Result<Network> {
    let file = File::open(path)?;
    let mut edges = HashMap::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (source, targets) = line.split_once(": ").ok_or(anyhow!("did not find colon"))?;
        let targets = targets.split_whitespace().map(|s| s.to_owned()).collect();
        edges.insert(source.to_string(), targets);
    }
    Ok(Network { edges })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "5");
        assert_eq!(&part2, "0");
    }

    #[test]
    fn test2_part() {
        let (part1, part2) = run("./files/test2.txt").expect("could not run");
        assert_eq!(&part1, "0");
        assert_eq!(&part2, "2");
    }
}
