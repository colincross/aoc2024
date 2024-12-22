use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashSet, fs::read_to_string};

struct Secret {
    num: u32,
}

impl Secret {
    fn from(s: &str) -> Self {
        Self::new(s.parse().unwrap())
    }

    fn new(num: u32) -> Self {
        Self { num }
    }

    fn next_num(num: u32) -> u32 {
        let mut num = num;
        num ^= num << 6;
        num %= 0x1000000;
        num ^= num >> 5;
        num %= 0x1000000;
        num ^= num << 11;
        num %= 0x1000000;
        num
    }

    #[allow(unused)]
    fn next(&self) -> Self {
        Self {
            num: Self::next_num(self.num),
        }
    }

    fn nth(&self, n: usize) -> Self {
        let mut num = self.num;

        for _ in 0..n {
            num = Self::next_num(num);
        }

        Self { num }
    }

    fn first_n(&self, n: usize) -> Vec<u32> {
        let mut num = self.num;
        let mut secrets = Vec::<u32>::with_capacity(n + 1);

        secrets.push(num);
        for _ in 0..n {
            num = Self::next_num(num);
            secrets.push(num);
        }
        secrets
    }

    fn sequences_and_prices_for_first_n(&self, n: usize) -> Vec<([i32; 4], u32)> {
        let secrets = self.first_n(n);
        let prices: Vec<_> = secrets.into_iter().map(|secret| secret % 10).collect();

        let deltas = prices
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| b as i32 - a as i32)
            .collect::<Vec<_>>();

        let sequences = deltas
            .windows(4)
            .map(|window| window.try_into().unwrap())
            .collect::<Vec<_>>();

        sequences
            .into_iter()
            .zip(prices.into_iter().skip(4))
            .collect()
    }
}

fn sum_of_2000th(secrets: &[Secret]) -> u64 {
    secrets
        .par_iter()
        .map(|secret| secret.nth(2000).num as u64)
        .sum()
}

fn price_for_sequence(sequences_and_prices: &[([i32; 4], u32)], sequence: &[i32; 4]) -> u32 {
    sequences_and_prices
        .iter()
        .find(|&(s, _)| s == sequence)
        .map_or(0, |&(_, price)| price)
}

fn total_bananas(secrets: &[Secret]) -> u64 {
    let all_sequences_and_prices: Vec<_> = secrets
        .par_iter()
        .map(|secret| secret.sequences_and_prices_for_first_n(2000))
        .collect();

    let all_sequences: HashSet<_> = all_sequences_and_prices
        .iter()
        .flat_map(|v| v.iter().map(|(sequence, _)| sequence))
        .collect();

    let max_bananas = all_sequences
        .into_par_iter()
        .map(|sequence| {
            all_sequences_and_prices
                .iter()
                .map(|sequences_and_prices| {
                    price_for_sequence(sequences_and_prices, sequence) as u64
                })
                .sum()
        })
        .max()
        .unwrap();

    max_bananas
}

fn parse_input(data: &str) -> Vec<Secret> {
    data.lines().map(Secret::from).collect::<Vec<_>>()
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
    let secrets = parse_input(&data);

    println!("sum of 2000th: {}", sum_of_2000th(&secrets));
    println!("total bananas: {}", total_bananas(&secrets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth() {
        let secret = Secret::new(123);
        assert_eq!(secret.next().num, 15887950);
        assert_eq!(Secret::new(15887950).next().num, 16495136);
        assert_eq!(secret.next().next().num, 16495136);
        assert_eq!(secret.nth(1).num, 15887950);
        assert_eq!(secret.nth(2).num, 16495136);
        assert_eq!(secret.nth(3).num, 527345);
        assert_eq!(secret.nth(4).num, 704524);
        assert_eq!(secret.nth(5).num, 1553684);
        assert_eq!(secret.nth(6).num, 12683156);
        assert_eq!(secret.nth(7).num, 11100544);
        assert_eq!(secret.nth(8).num, 12249484);
        assert_eq!(secret.nth(9).num, 7753432);
        assert_eq!(secret.nth(10).num, 5908254);
    }

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let secrets = parse_input(&data);
        assert_eq!(sum_of_2000th(&secrets), 37327623);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let secrets = parse_input(&data);
        assert_eq!(sum_of_2000th(&secrets), 20411980517);
    }

    #[test]
    fn test_price_for_sequence() {
        let secret = Secret::new(1);
        let sequences_and_prices = secret.sequences_and_prices_for_first_n(2000);
        assert_eq!(
            price_for_sequence(
                &Secret::new(123).sequences_and_prices_for_first_n(2000),
                &[-3, 6, -1, -1]
            ),
            4
        );
        assert_eq!(
            price_for_sequence(&sequences_and_prices, &[-2, 1, -1, 3]),
            7
        );
    }

    #[test]
    fn test_part2() {
        let data = "1\n2\n3\n2024";
        let secrets = parse_input(&data);
        let sequence = [-2, 1, -1, 3];
        assert_eq!(
            price_for_sequence(
                &Secret::new(1).sequences_and_prices_for_first_n(2000),
                &sequence
            ),
            7
        );
        assert_eq!(
            price_for_sequence(
                &Secret::new(2).sequences_and_prices_for_first_n(2000),
                &sequence
            ),
            7
        );
        assert_eq!(
            price_for_sequence(
                &Secret::new(3).sequences_and_prices_for_first_n(2000),
                &sequence
            ),
            0
        );
        assert_eq!(
            price_for_sequence(
                &Secret::new(2024).sequences_and_prices_for_first_n(2000),
                &sequence
            ),
            9
        );

        let all_sequences_and_prices: Vec<_> = secrets
            .iter()
            .map(|secret| secret.sequences_and_prices_for_first_n(2000))
            .collect();

        let prices: Vec<_> = all_sequences_and_prices
            .iter()
            .map(|sequences_and_prices| price_for_sequence(&sequences_and_prices, &[-2, 1, -1, 3]))
            .collect();
        assert_eq!(prices, &[7, 7, 0, 9]);

        assert_eq!(total_bananas(&secrets), 23);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let secrets = parse_input(&data);
        assert_eq!(total_bananas(&secrets), 2362);
    }
}
