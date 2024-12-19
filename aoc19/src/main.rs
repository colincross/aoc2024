use std::fs::read_to_string;

fn parse_input(data: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = data.lines();

    let available = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect();

    lines.next();

    let designs = lines.map(String::from).collect();

    (available, designs)
}

fn design_is_possible(available: &[String], design: &str) -> bool {
    if design == "" {
        return true;
    }

    for pattern in available {
        if design.starts_with(pattern) {
            if design_is_possible(available, &design[pattern.len()..]) {
                return true;
            }
        }
    }

    return false;
}

fn possible_designs(available: &[String], designs: &[String]) -> usize {
    designs
        .iter()
        .filter(|&design| design_is_possible(available, design))
        .count()
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
    let (available, designs) = parse_input(&data);
    println!(
        "possible designs: {}",
        possible_designs(&available, &designs)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (available, designs) = parse_input(&data);
        assert_eq!(possible_designs(&available, &designs), 6);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (available, designs) = parse_input(&data);
        assert_eq!(possible_designs(&available, &designs), 216);
    }
}
