use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy, Debug)]
struct Location {
    x: i32,
    y: i32,
}

type ButtonPair = (char, char);
type ButtonPairSequenceMap = HashMap<ButtonPair, Vec<String>>;
type ButtonPairCostMap = HashMap<ButtonPair, u64>;

fn all_button_pairs_iter() -> impl Iterator<Item = ButtonPair> {
    DIRPAD_BUTTONS
        .iter()
        .cloned()
        .flat_map(|a| DIRPAD_BUTTONS.iter().cloned().map(move |b| (a, b)))
}

fn generate_sequences<'a>(
    keypad: &'a Keypad,
) -> impl Iterator<Item = (ButtonPair, Vec<String>)> + 'a {
    all_button_pairs_iter().map(|(from, to)| {
        (
            (from, to),
            keypad.sequences(&keypad.button_location(from), &keypad.button_location(to)),
        )
    })
}

fn generate_costs(sequences_map: &ButtonPairSequenceMap, n: usize) -> Vec<ButtonPairCostMap> {
    let first_button_pad_costs =
        ButtonPairCostMap::from_iter(all_button_pairs_iter().map(|pair| (pair, 1)));
    let mut costs = Vec::<ButtonPairCostMap>::new();
    costs.push(first_button_pad_costs);

    for _ in 0..n {
        let previous_pad_costs = costs.last().unwrap();
        let pad_costs = ButtonPairCostMap::from_iter(all_button_pairs_iter().map(|pair| {
            (
                pair,
                cost_for_pair(&sequences_map[&pair], previous_pad_costs),
            )
        }));
        costs.push(pad_costs)
    }

    costs
}

fn cost_for_pair(sequences: &[String], previous_pad_costs: &ButtonPairCostMap) -> u64 {
    sequences
        .iter()
        .map(|seq| cost_for_sequence(seq, previous_pad_costs))
        .min()
        .unwrap()
}

fn cost_for_sequence(seq: &str, pad_costs: &ButtonPairCostMap) -> u64 {
    // Implicitly assume all sequences start from 'A'
    ("A".to_owned() + seq)
        .chars()
        .tuple_windows()
        .map(|(a, b)| pad_costs[&(a, b)])
        .sum::<u64>()
}

lazy_static! {
    static ref DIRPAD_BUTTONS: Vec<char> = vec!['<', '>', '^', 'v', 'A'];
    static ref DIRPAD_PATHS: ButtonPairSequenceMap =
        HashMap::from_iter(generate_sequences(&DIRPAD));
    static ref DIRPAD_COSTS: Vec<ButtonPairCostMap> = generate_costs(&DIRPAD_PATHS, 25);
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
        let horiz = if to.x > from.x {
            ">".repeat((to.x - from.x) as usize)
        } else if to.x < from.x {
            "<".repeat((from.x - to.x) as usize)
        } else {
            "".to_string()
        };
        let vert = if to.y > from.y {
            "v".repeat((to.y - from.y) as usize)
        } else if to.y < from.y {
            "^".repeat((from.y - to.y) as usize)
        } else {
            "".to_string()
        };

        let no_vert_first = from.x == 0 && to.y == self.blank.y;
        let no_horiz_first = to.x == 0 && from.y == self.blank.y;

        let mut seqs: Vec<String> = Default::default();
        let horiz_first = horiz.clone() + &vert + &"A";
        let vert_first = vert + &horiz + &"A";
        let duplicate = horiz_first == vert_first;

        if !no_horiz_first {
            seqs.push(horiz_first);
        }
        if !no_vert_first && !duplicate {
            seqs.push(vert_first);
        }
        seqs
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
        let mut min_len = usize::MAX;
        target
            .chars()
            .map(|c| self.push_sequences(c))
            .multi_cartesian_product()
            .map(|seqs| seqs.join(""))
            .filter(|seq| {
                let len = seq.len();
                if len > min_len {
                    false
                } else {
                    min_len = len;
                    true
                }
            })
            .collect()
    }

    #[allow(unused)]
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

fn count_sequence(target: &str, n: usize) -> u64 {
    let mut numpad = KeypadState::new(&NUMPAD);
    let seqs = numpad.sequence(target);

    seqs.iter()
        .map(|seq| cost_for_sequence(seq, &DIRPAD_COSTS[n]))
        .min()
        .unwrap()
}

fn numeric_part(target: &str) -> u64 {
    target[..target.len() - 1].parse().unwrap()
}

fn sum_of_complexities(targets: &[String], n: usize) -> u64 {
    targets
        .par_iter()
        .map(|target| count_sequence(target, n) * numeric_part(target))
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

    println!(
        "sum of complexities with 2 directional keypads: {}",
        sum_of_complexities(&targets, 2)
    );
    println!(
        "sum of complexities with 25 directional keypads: {}",
        sum_of_complexities(&targets, 25)
    );
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
                &"<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A".to_owned()
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
                &"v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A".to_owned()
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

        assert_eq!(sum_of_complexities(&targets, 2), 126384);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let targets = parse_input(&data);

        assert_eq!(sum_of_complexities(&targets, 2), 163920);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let targets = parse_input(&data);

        assert_eq!(sum_of_complexities(&targets, 25), 204040805018350);
    }
}
