use rayon::prelude::*;
use std::fs::read_to_string;

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
}

fn sum_of_2000th(secrets: &[Secret]) -> u64 {
    secrets
        .par_iter()
        .map(|secret| secret.nth(2000).num as u64)
        .sum()
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
}
