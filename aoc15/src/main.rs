use mygrid::{Direction, Grid, Position};
use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            &Cell::Empty => ".".to_string(),
            &Cell::Box => "O".to_string(),
            &Cell::BoxLeft => "[".to_string(),
            &Cell::BoxRight => "]".to_string(),
            &Cell::Wall => "#".to_string(),
        }
    }
}

#[derive(Debug)]
struct State {
    grid: Grid<Cell>,
    robot: Position,
}

impl State {
    fn from(lines: &[&str]) -> Self {
        let x_size = lines[0].len();
        let y_size = lines.len();

        let grid = Grid::from_iter(
            x_size,
            y_size,
            lines
                .iter()
                .flat_map(|&line| line.bytes())
                .map(|c| match c {
                    b'#' => Cell::Wall,
                    b'O' => Cell::Box,
                    b'@' => Cell::Empty,
                    b'.' => Cell::Empty,
                    _ => panic!(),
                }),
        );

        let robot = grid
            .iter_positions()
            .find(|&pos| lines[pos.y as usize].as_bytes()[pos.x as usize] == b'@')
            .unwrap();

        Self { grid, robot }
    }

    fn double_from(from: &Self) -> Self {
        let doubler = from
            .grid
            .iter()
            .map(|cell| match cell {
                &Cell::Box => vec![Cell::BoxLeft, Cell::BoxRight],
                &Cell::Empty => vec![Cell::Empty, Cell::Empty],
                &Cell::Wall => vec![Cell::Wall, Cell::Wall],
                _ => panic!(),
            })
            .flatten();
        let grid = Grid::<Cell>::from_iter(from.grid.x_size * 2, from.grid.y_size, doubler);
        let robot = Position::new(from.robot.x * 2, from.robot.y);

        Self { grid, robot }
    }

    fn recurse_move_boxes(&self, pos: &Position, dir: &Direction) -> (Vec<Position>, bool) {
        match self.grid[&pos] {
            Cell::Wall => (vec![], true),
            Cell::Empty => (vec![], false),
            box_cell => {
                let next_box = pos.step(dir);
                let (mut boxes, mut hits_wall) = self.recurse_move_boxes(&next_box, dir);
                if (box_cell == Cell::BoxLeft || box_cell == Cell::BoxRight)
                    && (dir == &mygrid::UP || dir == &mygrid::DOWN)
                {
                    let pair_box = pos.step(if box_cell == Cell::BoxLeft {
                        &mygrid::RIGHT
                    } else {
                        &mygrid::LEFT
                    });
                    let pair_next_box = pair_box.step(dir);
                    let (pair_boxes, other_hits_wall) =
                        self.recurse_move_boxes(&pair_next_box, dir);
                    boxes.extend(pair_boxes);
                    boxes.insert(0, pair_box.clone());
                    hits_wall |= other_hits_wall;
                }
                boxes.insert(0, pos.clone());
                (boxes, hits_wall)
            }
        }
    }

    fn move_robot(&mut self, dir: &Direction) {
        assert_eq!(self.grid[&self.robot], Cell::Empty);
        let next_robot_pos = self.robot.step(dir);
        let (boxes, hits_wall) = self.recurse_move_boxes(&next_robot_pos, dir);
        if hits_wall {
            return;
        }

        let mut already_moved_boxes = HashSet::<Position>::new();
        for box_pos in boxes.iter().rev() {
            if already_moved_boxes.contains(box_pos) {
                continue;
            }
            let moved_box_pos = box_pos.step(dir);
            self.grid[&moved_box_pos] = self.grid[&box_pos];
            self.grid[&box_pos] = Cell::Empty;
            already_moved_boxes.insert(box_pos.clone());
        }

        assert_eq!(self.grid[&next_robot_pos], Cell::Empty);
        self.robot = next_robot_pos;
    }

    fn move_robot_multiple(&mut self, dirs: &[Direction]) {
        for dir in dirs {
            self.move_robot(dir);
        }
    }

    fn iter_boxes<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        self.grid
            .iter_positions()
            .filter(|pos| self.grid[pos] == Cell::Box || self.grid[pos] == Cell::BoxLeft)
    }

    #[allow(unused)]
    fn pretty_print_grid(&self) -> String {
        self.grid.to_string()
    }
}

fn parse_input(data: &str) -> (State, Vec<Direction>) {
    let mut lines = data.lines();
    let grid_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let state = State::from(&grid_lines);

    let movements = lines
        .flat_map(|line| line.bytes())
        .map(Direction::from)
        .collect();

    (state, movements)
}

fn sum_of_box_gps_coordinates(state: &State) -> u64 {
    state
        .iter_boxes()
        .map(|pos| pos.y as u64 * 100 + pos.x as u64)
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
    let (mut state, movements) = parse_input(&data);
    let mut double_state = State::double_from(&state);
    state.move_robot_multiple(&movements);
    println!(
        "sum of box gps coordinates: {}",
        sum_of_box_gps_coordinates(&state)
    );
    double_state.move_robot_multiple(&movements);
    println!(
        "sum of box gps coordinates after doubling: {}",
        sum_of_box_gps_coordinates(&double_state)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (mut state, movements) = parse_input(&data);
        state.move_robot_multiple(&movements);
        assert_eq!(sum_of_box_gps_coordinates(&state), 10092);
    }

    #[test]
    fn test_part1_small() {
        let data = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let (mut state, movements) = parse_input(&data);
        state.move_robot_multiple(&movements);
        println!("{}", state.pretty_print_grid());
        assert_eq!(sum_of_box_gps_coordinates(&state), 2028);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (mut state, movements) = parse_input(&data);
        state.move_robot_multiple(&movements);
        assert_eq!(sum_of_box_gps_coordinates(&state), 1526673);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let (mut state, movements) = parse_input(&data);
        state = State::double_from(&state);
        state.move_robot_multiple(&movements);
        assert_eq!(sum_of_box_gps_coordinates(&state), 9021);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let (mut state, movements) = parse_input(&data);
        state = State::double_from(&state);
        state.move_robot_multiple(&movements);
        assert_eq!(sum_of_box_gps_coordinates(&state), 1535509);
    }
}
