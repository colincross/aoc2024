use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Debug)]
struct Grid<T>
where
    T: Default,
{
    x_size: usize,
    y_size: usize,
    grid: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone + Copy,
{
    fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            x_size,
            y_size,
            grid: vec![T::default(); x_size * y_size],
        }
    }

    fn from_file(data: &str) -> Grid<u8> {
        let lines = data.lines().collect::<Vec<_>>();
        assert!(!lines.is_empty());
        let x_size = lines[0].len();
        let y_size = lines.len();
        let grid = lines.iter().flat_map(|line| line.bytes()).collect();
        Grid::<u8> {
            x_size,
            y_size,
            grid,
        }
    }

    fn from_iter<I>(x_size: usize, y_size: usize, iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let grid = iter.collect();
        Self {
            x_size,
            y_size,
            grid,
        }
    }

    fn valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.x_size && pos.y >= 0 && (pos.y as usize) < self.y_size
    }

    fn at(&self, pos: &Position) -> Option<T> {
        if self.valid_pos(pos) {
            Some(self.grid[pos.y as usize * self.x_size + pos.x as usize])
        } else {
            None
        }
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut T> {
        if self.valid_pos(pos) {
            self.grid
                .get_mut(pos.y as usize * self.x_size + pos.x as usize)
        } else {
            None
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.y_size).flat_map(|y| {
            (0..self.x_size)
                .map(move |x| Position::new(x.try_into().unwrap(), y.try_into().unwrap()))
        })
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn step(&self, dir: &Direction) -> Self {
        Self {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn rotate_90_cw(self) -> Self {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!(),
        }
    }
}

const UP: Direction = Direction { x: 0, y: -1 };
const DOWN: Direction = Direction { x: 0, y: 1 };
const LEFT: Direction = Direction { x: -1, y: 0 };
const RIGHT: Direction = Direction { x: 1, y: 0 };

fn parse_input(data: &str) -> (Grid<bool>, Position) {
    let text_grid = Grid::<u8>::from_file(data);
    let obstruction_grid = Grid::from_iter(
        text_grid.x_size,
        text_grid.y_size,
        text_grid.iter().map(|&c| c == b'#'),
    );
    let start_position = text_grid
        .iter_positions()
        .find(|&pos| text_grid.at(&pos) == Some(b'^'))
        .unwrap();
    (obstruction_grid, start_position)
}

fn walk_to_exit(grid: &Grid<bool>, start: &Position) -> Option<usize> {
    let mut visited_grid = Grid::<bool>::new(grid.x_size, grid.y_size);
    let mut dir = UP;
    let mut pos = start.clone();
    let mut visited_obstacles = HashSet::<(Position, Direction)>::new();
    loop {
        let v = visited_grid.at_mut(&pos).expect("pos is valid");
        *v = true;

        let mut next_pos = pos.step(&dir);
        if !grid.valid_pos(&next_pos) {
            break;
        }
        if grid.at(&next_pos).expect("next_pos is valid") {
            // obstruction
            let key: (Position, Direction) = (pos.clone(), dir.clone());
            if visited_obstacles.contains(&key) {
                return None;
            }
            visited_obstacles.insert(key);

            dir = dir.rotate_90_cw();
            next_pos = pos.step(&dir);
        }
        if !grid.valid_pos(&next_pos) {
            break;
        }

        pos = next_pos;
    }

    Some(visited_grid.iter().filter(|&&v| v).count())
}

fn grid_with_obstruction(grid: &Grid<bool>, pos: &Position) -> Grid<bool> {
    let mut new_grid = grid.clone();
    *new_grid.at_mut(pos).expect("pos is valid") = true;
    new_grid
}

fn count_possible_looping_obstructions(grid: &Grid<bool>, start: &Position) -> usize {
    grid.iter_positions()
        .filter(|pos| pos != start)
        .filter(|pos| walk_to_exit(&grid_with_obstruction(grid, pos), start).is_none())
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
    let (grid, start) = parse_input(&data);
    println!(
        "cells visited walking to exit: {}",
        walk_to_exit(&grid, &start).expect("not a loop")
    );
    println!(
        "possible looping obstructions: {}",
        count_possible_looping_obstructions(&grid, &start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (grid, start) = parse_input(&data);
        let visited = walk_to_exit(&grid, &start).expect("not a loop");
        assert_eq!(visited, 41);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (grid, start) = parse_input(&data);
        let visited = walk_to_exit(&grid, &start).expect("not a loop");
        assert_eq!(visited, 4988);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let (grid, start) = parse_input(&data);
        let visited = count_possible_looping_obstructions(&grid, &start);
        assert_eq!(visited, 6);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let (grid, start) = parse_input(&data);
        let visited = count_possible_looping_obstructions(&grid, &start);
        assert_eq!(visited, 1697);
    }
}
