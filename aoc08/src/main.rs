use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::{Add, Sub},
};

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

    fn valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.x_size && pos.y >= 0 && (pos.y as usize) < self.y_size
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut T> {
        if self.valid_pos(pos) {
            self.grid
                .get_mut(pos.y as usize * self.x_size + pos.x as usize)
        } else {
            None
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Position, T)> + '_ {
        (0..self.y_size).flat_map(move |y| {
            (0..self.x_size).map(move |x| {
                (
                    Position::new(x.try_into().unwrap(), y.try_into().unwrap()),
                    self.grid[x + y * self.x_size],
                )
            })
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
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &'b Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a> Sub<&'a Position> for Position {
    type Output = Self;

    fn sub(self, other: &'a Position) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Delta {
    x: i32,
    y: i32,
}

fn parse_input(data: &str) -> Grid<u8> {
    let text_grid = Grid::<u8>::from_file(data);
    text_grid
}

fn find_antenna_groups(grid: &Grid<u8>) -> Vec<Vec<Position>> {
    let mut antenna_groups = HashMap::<u8, Vec<Position>>::new();
    grid.iter()
        .filter(|&(_, v)| v != b'.')
        .for_each(|(pos, v)| antenna_groups.entry(v).or_default().push(pos));
    antenna_groups.values().map(|v| v.clone()).collect()
}

fn iter_pairs<T>(v: &[T]) -> impl Iterator<Item = (&T, &T)> {
    v.iter()
        .enumerate()
        .flat_map(|(i, e1)| v[i + 1..].iter().map(move |e2| (e1, e2)))
}

fn antinodes(x_size: usize, y_size: usize, antenna_groups: &[Vec<Position>]) -> Grid<bool> {
    let mut antinodes = Grid::<bool>::new(x_size, y_size);
    for antennas in antenna_groups {
        for (a1, a2) in iter_pairs(antennas) {
            let an1 = a1 + a1 - a2;
            let an2 = a2 + a2 - a1;
            if let Some(v) = antinodes.at_mut(&an1) {
                *v = true;
            }
            if let Some(v) = antinodes.at_mut(&an2) {
                *v = true;
            }
        }
    }
    antinodes
}

fn count_antinodes(grid: &Grid<u8>) -> usize {
    let antenna_groups = find_antenna_groups(grid);
    let antinodes_grid = antinodes(grid.x_size, grid.y_size, &antenna_groups);
    antinodes_grid.iter().filter(|&(_, v)| v).count()
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
    let grid = parse_input(&data);
    println!("count antinodes: {}", count_antinodes(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = parse_input(&data);
        let count_antinodes = count_antinodes(&grid);
        assert_eq!(count_antinodes, 14);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = parse_input(&data);
        let count_antinodes = count_antinodes(&grid);
        assert_eq!(count_antinodes, 222);
    }
}
