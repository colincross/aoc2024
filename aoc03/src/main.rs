use std::fs::read_to_string;

struct Parser<'a> {
    buf: &'a str,
    muls: Vec<(u64, u64)>,
    n1: u64,
    n2: u64,
}

impl<'a> Parser<'a> {
    fn from(buf: &'a str) -> Self {
        Self {
            buf: buf,
            muls: vec![],
            n1: 0,
            n2: 0,
        }
    }

    fn parse(&mut self) {
        self.parse_muls();
    }

    fn parse_muls(&mut self) {
        const START_SEQUENCE: &str = "mul(";
        while let Some(_) = self.find(START_SEQUENCE) {
            self.parse_first_number();
        }
    }

    fn find(&mut self, seq: &str) -> Option<usize> {
        let start = self.buf.find(seq)?;
        self.buf = &self.buf[start + seq.len()..];
        Some(start)
    }

    fn peek(&self) -> Option<char> {
        self.buf.chars().nth(0)
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.buf = &self.buf[1..];
        Some(c)
    }

    fn parse_number(&mut self) -> Option<u64> {
        let mut num: String = String::new();
        loop {
            let c = self.peek();
            match c {
                Some(n @ '0'..='9') => {
                    num.push(n);
                    self.consume();
                }
                _ => {
                    if num.len() > 0 && num.len() <= 3 {
                        return num.parse().ok();
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    fn parse_first_number(&mut self) {
        let Some(n) = self.parse_number() else { return };
        self.n1 = n;
        self.parse_comma();
    }

    fn parse_second_number(&mut self) {
        let Some(n) = self.parse_number() else { return };
        self.n2 = n;
        self.parse_end_paren();
    }

    fn parse_end_paren(&mut self) {
        let Some(c) = self.peek() else { return };
        if c == ')' {
            self.consume();
            self.muls.push((self.n1, self.n2));
        }
    }

    fn parse_comma(&mut self) {
        let Some(c) = self.peek() else { return };
        if c == ',' {
            self.consume();
            self.parse_second_number()
        }
    }
}

fn parse_muls(memory: &str) -> Vec<(u64, u64)> {
    let mut parser = Parser::from(memory);
    parser.parse();
    parser.muls
}

fn sum_of_mul(memory: &str) -> u64 {
    let muls = parse_muls(memory);
    muls.iter().map(|&(a, b)| a * b).sum()
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
    dbg!(&input_file);
    let memory = read_to_string(&input_file).unwrap();

    println!("sum of muls: {}", sum_of_mul(&memory));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let memory = read_to_string("src/test.txt").unwrap();
        let sum_of_muls = sum_of_mul(&memory);
        assert_eq!(sum_of_muls, 161);
    }

    #[test]
    fn answer_part1() {
        let memory = read_to_string("src/main.txt").unwrap();
        let sum_of_muls = sum_of_mul(&memory);
        assert_eq!(sum_of_muls, 165225049);
    }
}
