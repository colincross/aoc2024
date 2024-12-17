use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug)]
struct Register {
    val: u64,
}

#[derive(Debug)]
struct Computer {
    a: Register,
    b: Register,
    c: Register,
    ip: usize,
    out: Vec<u8>,
}

type OpcodeFn = fn(&mut Computer, operand: u8) -> ();

impl Computer {
    const OPCODE_TABLE: [OpcodeFn; 8] = [
        Self::adv,
        Self::bxl,
        Self::bst,
        Self::jnz,
        Self::bxc,
        Self::out,
        Self::bdv,
        Self::cdv,
    ];

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            n @ 0..=3 => n as u64,
            4 => self.a.val,
            5 => self.b.val,
            6 => self.c.val,
            _ => panic!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a.val = self.a.val >> self.combo_operand(operand);
        self.ip += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.b.val = self.a.val >> self.combo_operand(operand);
        self.ip += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.c.val = self.a.val >> self.combo_operand(operand);
        self.ip += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.b.val = self.b.val ^ operand as u64;
        self.ip += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.b.val = self.combo_operand(operand) % 8;
        self.ip += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a.val == 0 {
            self.ip += 2;
        } else {
            self.ip = operand.into();
        }
    }

    fn bxc(&mut self, _: u8) {
        self.b.val = self.b.val ^ self.c.val;
        self.ip += 2;
    }

    fn out(&mut self, operand: u8) {
        self.out
            .push((self.combo_operand(operand) % 8).try_into().unwrap());
        self.ip += 2;
    }

    fn from(lines: &[&str]) -> Self {
        let registers = lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            })
            .map(|val| Register { val })
            .collect::<Vec<_>>();
        assert_eq!(registers.len(), 3);
        let (a, b, c) = registers.into_iter().next_tuple().unwrap();
        Self {
            a,
            b,
            c,
            ip: 0,
            out: vec![],
        }
    }

    #[allow(unused)]
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self {
            a: Register { val: a },
            b: Register { val: b },
            c: Register { val: c },
            ip: 0,
            out: vec![],
        }
    }

    fn handle_opcode(&mut self, opcode: u8, operand: u8) {
        assert!(opcode < 8);
        let opcode_fn = &Self::OPCODE_TABLE[opcode as usize];
        opcode_fn(self, operand);
    }

    fn read_from_ip(&self, program: &[u8]) -> Option<(u8, u8)> {
        if self.ip + 1 >= program.len() {
            None
        } else {
            Some((program[self.ip], program[self.ip + 1]))
        }
    }

    fn run(&mut self, program: &[u8]) -> Vec<u8> {
        self.ip = 0;
        self.out = vec![];
        while let Some((opcode, operand)) = self.read_from_ip(program) {
            self.handle_opcode(opcode, operand);
        }
        self.out.clone()
    }

    fn run_with_string_output(&mut self, program: &[u8]) -> String {
        self.run(program).iter().map(|n| n.to_string()).join(",")
    }
}

fn parse_input(data: &str) -> (Computer, Vec<u8>) {
    let mut lines = data.lines();
    let computer_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let computer = Computer::from(&computer_lines);

    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    (computer, program)
}

fn find_lowest_self_reproducing_a(program: &[u8]) -> u64 {
    // The given program operates on groups of 3 bits in A, mixing in some higher bits.  Start
    // with the last output.

    fn recurse(a: u64, i: usize, program: &[u8]) -> Option<u64> {
        let max_to_test = if i == 0 { 1024 } else { 8 };
        for j in 0..max_to_test {
            let test = (a << 3) + j;
            let mut computer = Computer::new(test, 0, 0);
            let out = computer.run(program);
            if program.len() >= out.len() && program[program.len() - out.len()..] == out {
                if program.len() == out.len() {
                    return Some(test);
                } else if let Some(answer) = recurse(test, i + 1, program) {
                    return Some(answer);
                }
            }
        }
        None
    }
    let a = recurse(0, 0, program).unwrap();

    let mut computer = Computer::new(a, 0, 0);
    let out = computer.run(program);
    assert_eq!(program, out);
    a
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
    let (mut computer, program) = parse_input(&data);
    println!(
        "program output: {}",
        computer.run_with_string_output(&program)
    );
    println!(
        "lowest self reproducing starting value: {}",
        find_lowest_self_reproducing_a(&program)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (mut computer, program) = parse_input(&data);
        assert_eq!(
            computer.run_with_string_output(&program),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_small() {
        let mut computer1 = Computer::new(0, 0, 9);
        computer1.run(&[2, 6]);
        assert_eq!(computer1.b.val, 1);

        let mut computer2 = Computer::new(10, 0, 0);
        computer2.run(&[5, 0, 5, 1, 5, 4]);
        assert_eq!(computer2.out, vec![0, 1, 2]);

        let mut computer3 = Computer::new(2024, 0, 0);
        computer3.run(&[0, 1, 5, 4, 3, 0]);
        assert_eq!(computer3.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer3.a.val, 0);

        let mut computer4 = Computer::new(0, 29, 0);
        computer4.run(&[1, 7]);
        assert_eq!(computer4.b.val, 26);

        let mut computer5 = Computer::new(0, 2024, 43690);
        computer5.run(&[4, 0]);
        assert_eq!(computer5.b.val, 44354);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (mut computer, program) = parse_input(&data);
        assert_eq!(
            computer
                .run(&program)
                .iter()
                .map(|n| n.to_string())
                .join(","),
            "7,3,1,3,6,3,6,0,2"
        );
    }

    #[test]
    fn test_part2_small() {
        let program = vec![0, 3, 5, 4, 3, 0];
        let mut computer = Computer::new(117440, 0, 0);
        assert_eq!(computer.run(&program), program);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let (_, program) = parse_input(&data);
        assert_eq!(find_lowest_self_reproducing_a(&program), 105843716614554);
    }
}
