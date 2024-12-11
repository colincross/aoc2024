use std::fs::read_to_string;

fn num_digits(n: u64) -> u32 {
    (n as f32).log10().floor() as u32 + 1
}

fn blink_stone(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        return (1, None);
    }
    let d = num_digits(n);
    if d % 2 == 0 {
        return (n / 10_u64.pow(d / 2), Some(n % 10_u64.pow(d / 2)));
    }
    return (n * 2024, None);
}

fn blink_stones(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let (n1, n2) = blink_stone(stones[i]);
        stones[i] = n1;
        if let Some(n2) = n2 {
            stones.insert(i + 1, n2);
            i += 1;
        }
        i += 1;
    }
}

fn num_stones_after_25_blinks(stones: &[u64]) -> usize {
    let mut stones = stones.to_vec();
    for _ in 0..25 {
        blink_stones(&mut stones);
    }
    stones.len()
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
    let num_stones_after_25_blinks = num_stones_after_25_blinks(&stones);
    println!("num stones after 25 blinks: {}", num_stones_after_25_blinks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let stones = parse_input(&data);
        let num_stones_after_25_blinks = num_stones_after_25_blinks(&stones);
        assert_eq!(num_stones_after_25_blinks, 55312);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let stones = parse_input(&data);
        let num_stones_after_25_blinks = num_stones_after_25_blinks(&stones);
        assert_eq!(num_stones_after_25_blinks, 194557);
    }
}
