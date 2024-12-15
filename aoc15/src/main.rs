use mygrid::{Direction, Grid, Position};
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    EMPTY,
    BOX,
    WALL,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            &Cell::EMPTY => ".".to_string(),
            &Cell::BOX => "O".to_string(),
            &Cell::WALL => "#".to_string(),
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
                    b'#' => Cell::WALL,
                    b'O' => Cell::BOX,
                    b'@' => Cell::EMPTY,
                    b'.' => Cell::EMPTY,
                    _ => panic!(),
                }),
        );

        let robot = grid
            .iter_positions()
            .find(|&pos| lines[pos.y as usize].as_bytes()[pos.x as usize] == b'@')
            .unwrap();

        Self { grid, robot }
    }

    fn move_robot(&mut self, dir: &Direction) {
        assert_eq!(self.grid[&self.robot], Cell::EMPTY);
        let next_robot_pos = self.robot.step(dir);
        let mut end_of_boxes_pos = next_robot_pos;
        while self.grid[&end_of_boxes_pos] == Cell::BOX {
            end_of_boxes_pos = end_of_boxes_pos.step(dir);
        }
        //dbg!(dir, &self.robot, &next_robot_pos, &end_of_boxes_pos);
        if self.grid[&end_of_boxes_pos] == Cell::EMPTY {
            if end_of_boxes_pos != next_robot_pos {
                self.grid[&end_of_boxes_pos] = Cell::BOX;
                self.grid[&next_robot_pos] = Cell::EMPTY;
            }
            self.robot = next_robot_pos;
        } else {
            assert_eq!(self.grid[&end_of_boxes_pos], Cell::WALL);
        }
    }

    fn move_robot_multiple(&mut self, dirs: &[Direction]) {
        for dir in dirs {
            self.move_robot(dir);
        }
    }

    fn iter_boxes<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        self.grid
            .iter_positions()
            .filter(|pos| self.grid[pos] == Cell::BOX)
    }

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
    state.move_robot_multiple(&movements);
    println!(
        "sum of box gps coordinates: {}",
        sum_of_box_gps_coordinates(&state)
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
}
