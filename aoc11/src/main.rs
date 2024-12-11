use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
};

fn num_digits(n: u64) -> u32 {
    (n as f32).log10().floor() as u32 + 1
}

fn blink_stone(n: u64) -> (u64, Option<u64>) {
    let d = num_digits(n);
    if n == 0 {
        (1, None)
    } else if d % 2 == 0 {
        (n / 10_u64.pow(d / 2), Some(n % 10_u64.pow(d / 2)))
    } else {
        (n * 2024, None)
    }
}

fn num_stones_after_blinks_one(
    n: u64,
    times: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if times == 0 {
        return 1;
    }
    let key = (n, times);
    match cache.entry((n, times)) {
        Entry::Occupied(occupied) => {
            return occupied.get().clone();
        }
        Entry::Vacant(_) => (),
    }
    let (n1, n2) = blink_stone(n);
    let mut stones = num_stones_after_blinks_one(n1, times - 1, cache);
    if let Some(n2) = n2 {
        stones += num_stones_after_blinks_one(n2, times - 1, cache);
    }
    cache.insert(key, stones);
    stones
}

fn num_stones_after_blinks(stones: &[u64], times: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| num_stones_after_blinks_one(stone, times, &mut cache))
        .sum()
}

fn parse_input(data: &str) -> Vec<u64> {
    data.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("number"))
        .collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = if args.len() >= 2 {
        std::path::PathBuf::from(&args[1])
    } else {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .join("../..")
            .join(exe.file_name().unwrap())
            .join("src/main.txt")
    };
    let data = read_to_string(&input_file).unwrap();
    let stones = parse_input(&data);
    let num_stones_after_25_blinks = num_stones_after_blinks(&stones, 25);
    println!("num stones after 25 blinks: {}", num_stones_after_25_blinks);
    let num_stones_after_75_blinks = num_stones_after_blinks(&stones, 75);
    println!("num stones after 75 blinks: {}", num_stones_after_75_blinks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let stones = parse_input(&data);
        let num_stones_after_25_blinks = num_stones_after_blinks(&stones, 25);
        assert_eq!(num_stones_after_25_blinks, 55312);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let stones = parse_input(&data);
        let num_stones_after_25_blinks = num_stones_after_blinks(&stones, 25);
        assert_eq!(num_stones_after_25_blinks, 194557);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let stones = parse_input(&data);
        let num_stones_after_75_blinks = num_stones_after_blinks(&stones, 75);
        assert_eq!(num_stones_after_75_blinks, 231532558973909);
    }
}
