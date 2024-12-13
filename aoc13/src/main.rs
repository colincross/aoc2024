use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Button {
    cost: u32,
    x: u32,
    y: u32,
}

impl Button {
    const A_COST: u32 = 3;
    const B_COST: u32 = 1;
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    x: u32,
    y: u32,
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
}

fn parse_input(data: &str) -> Vec<Machine> {
    Machine::from(data)
}

fn min_tokens(machine: &Machine) -> Option<u32> {
    (0..100)
        .filter_map(|a| {
            if let Some(b) =
                (machine.x.checked_sub(a * machine.a.x)).and_then(|f| Some(f / machine.b.x))
            {
                Some((a, b))
            } else {
                None
            }
        })
        .filter(|&(a, b)| {
            machine.a.x * a + machine.b.x * b == machine.x
                && machine.a.y * a + machine.b.y * b == machine.y
        })
        .map(|(a, b)| a * machine.a.cost + b * machine.b.cost)
        .min()
}

fn min_total_tokens(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| min_tokens(machine).unwrap_or_default())
        .sum()
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
}
