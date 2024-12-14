use mygrid::Position;
use regex::Regex;
use std::{cmp::Ordering, fs::read_to_string};

#[derive(Debug)]
struct Robot {
    p: Position,
    v: Position,
}

impl Robot {
    fn from(line: &str) -> Self {
        let re = Regex::new("p=(?<px>-?[0-9]+),(?<py>-?[0-9]+) v=(?<vx>-?[0-9]+),(?<vy>-?[0-9]+)")
            .expect("compiles");
        let caps = re.captures(line).expect("matches");
        Self {
            p: Position {
                x: caps["px"].parse().unwrap(),
                y: caps["py"].parse().unwrap(),
            },
            v: Position {
                x: caps["vx"].parse().unwrap(),
                y: caps["vy"].parse().unwrap(),
            },
        }
    }

    fn traverse(&self, seconds: i32, size: &Position) -> Self {
        let ret = Self {
            p: (((self.p + self.v * seconds) % size) + size) % size,
            v: self.v,
        };
        ret
    }
}

fn product_of_robots_in_quadrants_after_traverse(
    robots: &[Robot],
    seconds: i32,
    size: &Position,
) -> usize {
    let middle = size / 2;
    robots
        .iter()
        .map(|robot| robot.traverse(seconds, size))
        .map(
            |robot| match (robot.p.x.cmp(&middle.x), robot.p.y.cmp(&middle.y)) {
                (Ordering::Less, Ordering::Less) => [1, 0, 0, 0],
                (Ordering::Greater, Ordering::Less) => [0, 1, 0, 0],
                (Ordering::Less, Ordering::Greater) => [0, 0, 1, 0],
                (Ordering::Greater, Ordering::Greater) => [0, 0, 0, 1],
                (_, _) => [0, 0, 0, 0],
            },
        )
        .reduce(|a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]])
        .unwrap()
        .iter()
        .product()
}

fn parse_input(data: &str) -> Vec<Robot> {
    data.lines().map(Robot::from).collect()
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
    let robots = parse_input(&data);
    println!(
        "product of robots in quadrants after traverse: {}",
        product_of_robots_in_quadrants_after_traverse(&robots, 100, &Position::new(101, 103))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let robots = parse_input(&data);
        assert_eq!(
            product_of_robots_in_quadrants_after_traverse(&robots, 100, &Position::new(11, 7)),
            12
        );
    }

    #[test]
    fn test_part1_1() {
        let robot = Robot::from("p=2,4 v=2,-3");
        let size = Position::new(11, 7);
        assert_eq!(robot.traverse(1, &size).p, Position::new(4, 1));
        assert_eq!(robot.traverse(2, &size).p, Position::new(6, 5));
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let robots = parse_input(&data);
        assert_eq!(
            product_of_robots_in_quadrants_after_traverse(&robots, 100, &Position::new(101, 103)),
            236628054
        );
    }
}
