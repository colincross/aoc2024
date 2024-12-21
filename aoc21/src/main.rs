use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

struct Keypad {
    buttons: HashMap<char, Location>,
    blank: Location,
}

impl Keypad {
    fn from(rows: &[&str]) -> Self {
        let mut buttons = HashMap::new();
        let mut blank = Location { x: -1, y: -1 };

        for (y, row) in rows.iter().enumerate() {
            for (x, b) in row.chars().enumerate() {
                let loc = Location {
                    x: x as i32,
                    y: y as i32,
                };
                if b == '\0' {
                    blank = loc;
                } else {
                    buttons.insert(b, loc);
                }
            }
        }

        Self { buttons, blank }
    }

    fn button_location(&self, c: char) -> Location {
        self.buttons[&c]
    }

    fn sequences(&self, from: &Location, to: &Location) -> Vec<String> {
        fn recurse(seq: &str, from: &Location, to: &Location, blank: &Location) -> Vec<String> {
            if from.x == blank.x && from.y == blank.y {
                return vec![];
            }
            let mut seqs = Vec::<String>::new();
            if from.x > to.x {
                seqs.extend(recurse(
                    &(seq.to_owned() + "<"),
                    &Location {
                        x: from.x - 1,
                        y: from.y,
                    },
                    to,
                    blank,
                ));
            } else if from.x < to.x {
                seqs.extend(recurse(
                    &(seq.to_owned() + ">"),
                    &Location {
                        x: from.x + 1,
                        y: from.y,
                    },
                    to,
                    blank,
                ));
            }

            if from.y > to.y {
                seqs.extend(recurse(
                    &(seq.to_owned() + "^"),
                    &Location {
                        x: from.x,
                        y: from.y - 1,
                    },
                    to,
                    blank,
                ));
            } else if from.y < to.y {
                seqs.extend(recurse(
                    &(seq.to_owned() + "v"),
                    &Location {
                        x: from.x,
                        y: from.y + 1,
                    },
                    to,
                    blank,
                ));
            }

            if from.x == to.x && from.y == to.y {
                seqs.push(seq.to_owned() + &"A");
            }

            seqs
        }
        recurse("", from, to, &self.blank)
    }
}

lazy_static! {
    static ref NUMPAD: Keypad = Keypad::from(&["789", "456", "123", "\00A"]);
    static ref DIRPAD: Keypad = Keypad::from(&["\0^A", "<v>"]);
}
struct KeypadState<'a> {
    keypad: &'a Keypad,
    loc: Location,
}

impl<'a> KeypadState<'a> {
    fn new(keypad: &'a Keypad) -> Self {
        Self {
            keypad,
            loc: keypad.buttons[&'A'],
        }
    }

    fn push_sequences(&mut self, c: char) -> Vec<String> {
        let to = self.keypad.button_location(c);
        let seqs = self.keypad.sequences(&self.loc, &to);
        self.loc = to;
        seqs
    }

    fn sequence(&mut self, target: &str) -> Vec<String> {
        target
            .chars()
            .map(|c| self.push_sequences(c))
            .multi_cartesian_product()
            .map(|seqs| seqs.join(""))
            .collect()
    }

    fn sequences(&mut self, targets: &[String]) -> Vec<String> {
        let seqs = targets
            .iter()
            .flat_map(|target| self.sequence(target))
            .collect::<Vec<_>>();

        let min_len = seqs.iter().map(|seq| seq.len()).min().unwrap();

        seqs.into_iter()
            .filter(|seq| seq.len() == min_len)
            .collect()
    }
}

fn count_sequence(target: &str) -> usize {
    let mut numpad = KeypadState::new(&NUMPAD);
    let mut dirpad1 = KeypadState::new(&DIRPAD);
    let mut dirpad2 = KeypadState::new(&DIRPAD);

    dirpad2
        .sequences(&dirpad1.sequences(&numpad.sequence(target)))
        .iter()
        .map(|seq| seq.len())
        .min()
        .unwrap()
}

fn numeric_part(target: &str) -> usize {
    target[..target.len() - 1].parse().unwrap()
}

fn sum_of_complexities(targets: &[String]) -> usize {
    targets
        .iter()
        .map(|target| count_sequence(target) * numeric_part(target))
        .sum()
}

fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(String::from).collect::<Vec<_>>()
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
    let targets = parse_input(&data);

    println!("sum of complexities: {}", sum_of_complexities(&targets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numpad_sequence() {
        let mut numpad = KeypadState::new(&NUMPAD);
        assert!(numpad.sequence("029A").contains(&"<A^A>^^AvvvA".to_owned()));
    }

    #[test]
    fn test_dirpad_sequence() {
        let mut numpad = KeypadState::new(&NUMPAD);
        let mut dirpad = KeypadState::new(&DIRPAD);
        assert!(dirpad
            .sequences(&numpad.sequence("029A"))
            .contains(&"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_owned()))
    }

    #[test]
    fn test_dirpad2_sequence() {
        let mut numpad = KeypadState::new(&NUMPAD);
        let mut dirpad1 = KeypadState::new(&DIRPAD);
        let mut dirpad2 = KeypadState::new(&DIRPAD);
        assert!(dirpad2
            .sequences(&dirpad1.sequences(&numpad.sequence("029A")))
            .contains(
                &"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_owned()
            ))
    }

    #[test]
    fn test_379() {
        let mut numpad = KeypadState::new(&NUMPAD);
        let mut dirpad1 = KeypadState::new(&DIRPAD);
        let mut dirpad2 = KeypadState::new(&DIRPAD);
        assert!(dirpad2
            .sequences(&dirpad1.sequences(&numpad.sequence("379A")))
            .contains(
                &"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_owned()
            ))
    }

    #[test]
    fn test_379_dirpad1() {
        let mut numpad = KeypadState::new(&NUMPAD);
        let mut dirpad1 = KeypadState::new(&DIRPAD);
        assert!(dirpad1
            .sequences(&numpad.sequence("379A"))
            .contains(&"<A>A<AAv<AA>>^AvAA^Av<AAA^>A".to_owned()));
    }

    #[test]
    fn test_379_numpad() {
        let mut numpad = KeypadState::new(&NUMPAD);
        assert!(numpad
            .sequence("379A")
            .contains(&"^A^^<<A>>AvvvA".to_string()))
    }

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let targets = parse_input(&data);

        assert_eq!(sum_of_complexities(&targets), 126384);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let targets = parse_input(&data);

        assert_eq!(sum_of_complexities(&targets), 163920);
    }
}
