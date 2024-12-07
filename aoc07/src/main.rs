use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    answer: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn from(line: &str) -> Self {
        let (answer_str, nums_str) = line.split_once(':').expect("has colon");
        let answer = answer_str.parse().expect("answer is number");
        let nums = nums_str
            .trim()
            .split(" ")
            .map(|n| n.parse().expect("is number"))
            .collect();
        Self { answer, nums }
    }

    fn apply_operators(&self, operators: &[Operator]) -> u64 {
        assert_eq!(operators.len(), self.nums.len() - 1);
        self.nums[1..]
            .iter()
            .enumerate()
            .fold(self.nums[0], |accum, (i, &n)| operators[i].apply(accum, n))
    }
}

#[derive(Clone, Copy, Debug)]
struct Operator {
    f: fn(u64, u64) -> u64,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        (self.f)(a, b)
    }
}

const ADD: Operator = Operator { f: |a, b| a + b };
const MUL: Operator = Operator { f: |a, b| a * b };
const CONCAT: Operator = Operator {
    f: |a, b| (10 as u64).pow(count_digits(b)) * a + b,
};

fn count_digits(n: u64) -> u32 {
    (n as f64).log10() as u32 + 1
}

const FIRST_OPERATORS: &[Operator] = &[ADD, MUL];
const SECOND_OPERATORS: &[Operator] = &[ADD, MUL, CONCAT];

struct OperatorIterator<'a> {
    operator_list: &'a [Operator],
    len: u32,
    i: u64,
}

impl<'a> OperatorIterator<'a> {
    fn new(operator_list: &'a [Operator], len: usize) -> Self {
        Self {
            operator_list,
            len: len as u32,
            i: 0,
        }
    }
}

impl<'a> Iterator for OperatorIterator<'a> {
    type Item = Vec<Operator>;
    fn next(&mut self) -> Option<Self::Item> {
        let num_operators = self.operator_list.len() as u64;
        if self.i < num_operators.pow(self.len) {
            let mut operators = vec![ADD; self.len as usize];
            for (operator_number, operator) in operators.iter_mut().enumerate() {
                *operator = self.operator_list[((self.i
                    / num_operators.pow(operator_number as u32))
                    % num_operators) as usize]
            }
            self.i += 1;
            Some(operators)
        } else {
            None
        }
    }
}

fn solvable_equation(operators: &[Operator], equation: &Equation) -> bool {
    OperatorIterator::new(operators, equation.nums.len() - 1)
        .any(|operators| equation.apply_operators(&operators) == equation.answer)
}

fn solvable_equations_sum(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|&equation| solvable_equation(FIRST_OPERATORS, equation))
        .map(|equation| equation.answer)
        .sum()
}

fn solvable_equations_with_concat_sum(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|&equation| solvable_equation(SECOND_OPERATORS, equation))
        .map(|equation| equation.answer)
        .sum()
}

fn parse_input(data: &str) -> Vec<Equation> {
    data.lines().map(Equation::from).collect()
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
    let equations = parse_input(&data);
    println!(
        "solvable equations sum: {}",
        solvable_equations_sum(&equations)
    );
    println!(
        "solvable equations with concat sum: {}",
        solvable_equations_with_concat_sum(&equations)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let equations = parse_input(&data);
        let solvable_equations_sum = solvable_equations_sum(&equations);
        assert_eq!(solvable_equations_sum, 3749);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let equations = parse_input(&data);
        let solvable_equations_sum = solvable_equations_sum(&equations);
        assert_eq!(solvable_equations_sum, 8401132154762);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let equations = parse_input(&data);
        let solvable_equations_with_concat_sum = solvable_equations_with_concat_sum(&equations);
        assert_eq!(solvable_equations_with_concat_sum, 11387);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let equations = parse_input(&data);
        let solvable_equations_with_concat_sum = solvable_equations_with_concat_sum(&equations);
        assert_eq!(solvable_equations_with_concat_sum, 95297119227552);
    }

    #[test]
    fn test_concat() {
        assert_eq!(CONCAT.apply(1, 2), 12);
        assert_eq!(CONCAT.apply(15, 6), 156);
        assert_eq!(CONCAT.apply(48, 6), 486);
        assert_eq!(CONCAT.apply(17, 8), 178);
        assert_eq!(CONCAT.apply(1000, 1000), 10001000);
        assert_eq!(CONCAT.apply(12, 345), 12345);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(5), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(11), 2);
    }
}
