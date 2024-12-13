use regex::Regex;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Button {
    cost: u64,
    x: u64,
    y: u64,
}

impl Button {
    const A_COST: u64 = 3;
    const B_COST: u64 = 1;
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    x: u64,
    y: u64,
}

impl Machine {
    fn from(data: &str) -> Vec<Self> {
        let re = Regex::new(
            r#"Button A: X\+(?<ax>[0-9]+), Y\+(?<ay>[0-9]+)
Button B: X\+(?<bx>[0-9]+), Y\+(?<by>[0-9]+)
Prize: X=(?<px>[0-9]+), Y=(?<py>[0-9]+)
"#,
        )
        .expect("compiles");
        re.captures_iter(data)
            .map(|caps| Self {
                a: Button {
                    cost: Button::A_COST,
                    x: caps["ax"].parse().unwrap(),
                    y: caps["ay"].parse().unwrap(),
                },
                b: Button {
                    cost: Button::B_COST,
                    x: caps["bx"].parse().unwrap(),
                    y: caps["by"].parse().unwrap(),
                },
                x: caps["px"].parse().unwrap(),
                y: caps["py"].parse().unwrap(),
            })
            .collect()
    }

    fn add_10_trillion(&self) -> Self {
        const INC: u64 = 10000000000000;
        Self {
            a: self.a,
            b: self.b,
            x: self.x + INC,
            y: self.y + INC,
        }
    }

    fn solve(&self) -> (u64, u64) {
        // a * Ax + b * Bx = X
        // a * Ay + b * By = Y
        // a = (X - b * Bx) / Ax
        // (X - b * Bx) / Ax * Ay + b * By = Y
        // X * Ay / Ax - b * Bx * Ay / Ax + b * By = Y
        // b * (By - Bx * Ay / Ax) = Y - X * Ay / Ax
        // b = (Y - X * Ay / Ax) / (By - Bx * Ay / Ax)
        let ax = self.a.x as f64;
        let ay = self.a.y as f64;
        let bx = self.b.x as f64;
        let by = self.b.y as f64;
        let x = self.x as f64;
        let y = self.y as f64;

        let b = (y - x * ay / ax) / (by - bx * ay / ax);
        let a = (x - b * bx) / ax;

        (a.round() as u64, b.round() as u64)
    }
}

fn parse_input(data: &str) -> Vec<Machine> {
    Machine::from(data)
}

fn min_tokens(machine: &Machine) -> Option<u64> {
    let (a, b) = machine.solve();
    if a * machine.a.x + b * machine.b.x == machine.x
        && a * machine.a.y + b * machine.b.y == machine.y
    {
        Some(a * machine.a.cost + b * machine.b.cost)
    } else {
        None
    }
}

fn min_total_tokens(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .map(|machine| min_tokens(machine).unwrap_or_default())
        .sum()
}

fn add_10_trillion(machines: &[Machine]) -> Vec<Machine> {
    machines
        .iter()
        .map(|machine| machine.add_10_trillion())
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
    let machines = parse_input(&data);
    println!("min total tokens: {}", min_total_tokens(&machines));
    let machines = add_10_trillion(&machines);
    println!("min total tokens: {}", min_total_tokens(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let machines = parse_input(&data);
        assert_eq!(min_total_tokens(&machines), 480);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let machines = parse_input(&data);
        assert_eq!(min_total_tokens(&machines), 29877);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let machines = add_10_trillion(&parse_input(&data));
        assert_eq!(min_total_tokens(&machines), 99423413811305);
    }
}
