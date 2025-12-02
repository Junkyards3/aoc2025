use anyhow::{Result, anyhow};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::from_utf8,
};

fn main() {
    let (part1, part2) = run("./files/input.txt").expect("could not run");
    println!("part1 : {part1}");
    println!("part2 : {part2}");
}

struct IdRange {
    begin: u64,
    end: u64,
}

fn run(path: &str) -> Result<(String, String)> {
    let file = File::open(path)?;
    let ranges: Vec<IdRange> = BufReader::new(file)
        .split(b',')
        .map(|s| parse_range(&s?))
        .collect::<Result<Vec<_>>>()?;

    let part1 = part1(&ranges);
    let part2 = part2(&ranges);
    Ok((part1.to_string(), part2.to_string()))
}

fn part1(ranges: &[IdRange]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| compute_invalid(range.begin, range.end))
        .sum()
}

fn part2(ranges: &[IdRange]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| compute_invalid2(range.begin, range.end))
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}

fn compute_invalid(begin: u64, end: u64) -> Vec<u64> {
    let begin_digits_count = begin.to_string().len() as u32;
    let end_digits_count = end.to_string().len() as u32;
    let min_prefix_length = 1.max(begin_digits_count / 2);
    let max_prefix_length = 1.max(end_digits_count / 2);
    (min_prefix_length..=max_prefix_length)
        .flat_map(|length| {
            let start_prefix = 10u64.pow(length - 1);
            let limit = 10u64.pow(length);

            (start_prefix..limit)
                .map(move |prefix| prefix * 10u64.pow(length) + prefix)
                .skip_while(|id| *id < begin)
                .take_while(|id| *id <= end)
        })
        .collect()
}

fn compute_invalid2(begin: u64, end: u64) -> HashSet<u64> {
    let begin_digits_count = begin.to_string().len() as u32;
    let end_digits_count = end.to_string().len() as u32;
    let max_prefix_length = 1.max(end_digits_count / 2);
    (1..=max_prefix_length)
        .flat_map(|length| {
            let begin_repeat = (begin_digits_count / length).max(2);
            let end_repeat = end_digits_count / length;
            let start_prefix = 10u64.pow(length - 1);
            let limit = 10u64.pow(length);

            (begin_repeat..=end_repeat).flat_map(move |count_repeat| {
                (start_prefix..limit)
                    .map(move |prefix| repeat(prefix, count_repeat))
                    .skip_while(move |id| *id < begin)
                    .take_while(move |id| *id <= end)
            })
        })
        .collect()
}

fn repeat(prefix: u64, count: u32) -> u64 {
    let length = prefix.to_string().len() as u32;
    (0..count).map(|idx| prefix * 10u64.pow(idx * length)).sum()
}

fn parse_range(bytes: &[u8]) -> Result<IdRange> {
    let str = from_utf8(bytes)?.trim_end_matches("\n");
    let (begin_str, end_str) = str
        .split_once('-')
        .ok_or(anyhow!("could not split on - in the range"))?;
    let begin = begin_str.parse::<u64>()?;
    let end = end_str.parse::<u64>()?;
    Ok(IdRange { begin, end })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_part() {
        let (part1, part2) = run("./files/test.txt").expect("could not run");
        assert_eq!(&part1, "1227775554");
        assert_eq!(&part2, "4174379265");
    }

    #[test]
    fn test_compute_invalid_1() {
        let invalids = compute_invalid(11, 22);
        assert_eq!(invalids, vec![11, 22]);

        let invalids = compute_invalid(95, 115);
        assert_eq!(invalids, vec![99]);

        let invalids = compute_invalid(222220, 222224);
        assert_eq!(invalids, vec![222222]);

        let invalids = compute_invalid(1698522, 1698528);
        assert_eq!(invalids, vec![]);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(1, 3), 111);
        assert_eq!(repeat(13, 2), 1313);
        assert_eq!(repeat(1970, 3), 197019701970);
    }

    fn is_repeat(chain: &str) -> bool {
        let chars: Vec<char> = chain.chars().collect();
        (1..chars.len())
            .filter(|chunk_count| chars.len() % chunk_count == 0)
            .any(|length| {
                let chunks = chars.chunks(length).collect::<Vec<_>>();
                chunks.windows(2).all(|window| window[0] == window[1])
            })
    }

    #[test]
    fn test_is_repeat() {
        assert!(is_repeat("111"));
        assert!(is_repeat("1313"));
        assert!(is_repeat("197019701970"));
        assert!(!is_repeat("197019701971"));
    }

    #[test]
    fn test_low_invalid() {
        assert_eq!(compute_invalid2(1, 14), HashSet::from([11]))
    }

    proptest! {
        #[test]
        fn test_invalids(a in 1..1000u64, b in 1..1000u64) {
            let begin = a.min(b);
            let end = a.max(b);
            let invalids = compute_invalid2(begin, end);

            for invalid in invalids {
                assert!((begin..=end).contains(&invalid));
                assert!(is_repeat(&invalid.to_string()));
            }
        }
    }
}
