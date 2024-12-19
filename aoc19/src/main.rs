use std::{collections::HashMap, fs::read_to_string};

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

fn count_arrangements(
    available: &[String],
    design: &str,
    seen: &mut HashMap<String, usize>,
) -> usize {
    if design == "" {
        return 1;
    }

    if let Some(&previous) = seen.get(design) {
        return previous;
    }

    let mut count = 0;
    for pattern in available {
        if design.starts_with(pattern) {
            count += count_arrangements(available, &design[pattern.len()..], seen);
        }
    }

    seen.insert(design.to_string(), count);

    return count;
}

fn possible_designs(available: &[String], designs: &[String]) -> usize {
    let mut seen = HashMap::new();
    designs
        .iter()
        .filter(|&design| count_arrangements(available, design, &mut seen) > 0)
        .count()
}

fn count_total_arrangements(available: &[String], designs: &[String]) -> usize {
    let mut seen = HashMap::new();
    designs
        .iter()
        .map(|design| count_arrangements(available, design, &mut seen))
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
    let (available, designs) = parse_input(&data);
    println!(
        "possible designs: {}",
        possible_designs(&available, &designs)
    );
    println!(
        "total arrangements: {}",
        count_total_arrangements(&available, &designs)
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

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let (available, designs) = parse_input(&data);
        assert_eq!(count_total_arrangements(&available, &designs), 16);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let (available, designs) = parse_input(&data);
        assert_eq!(
            count_total_arrangements(&available, &designs),
            603191454138773
        );
    }
}
