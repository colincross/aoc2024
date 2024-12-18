use mygrid::{Grid, Position};
use pathfinding::directed::astar;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Cell {
    t: Option<usize>,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self.t {
            None => ".".to_string(),
            Some(_) => "#".to_string(),
        }
    }
}

struct Maze {
    grid: Grid<Cell>,
    start: Position,
    end: Position,
}

impl Maze {
    fn from(x_size: usize, y_size: usize, data: &str) -> Self {
        let mut grid = Grid::<Cell>::new(x_size, y_size);

        let start = Position::new(0, 0);
        let end = Position::new(x_size as i32 - 1, y_size as i32 - 1);

        for (t, line) in data.lines().enumerate() {
            let (x, y) = line.split_once(",").unwrap();
            let pos = Position::new(x.parse().unwrap(), y.parse().unwrap());
            grid[&pos] = Cell { t: Some(t) };
        }

        Self { grid, start, end }
    }

    #[allow(unused)]
    fn with_max_t(&self, max_t: usize) -> Self {
        let grid = Grid::<Cell>::from_iter(
            self.grid.x_size,
            self.grid.y_size,
            self.grid.iter().map(|c| Cell {
                t: match c.t {
                    None => None,
                    Some(t) => {
                        if t < max_t {
                            Some(t)
                        } else {
                            None
                        }
                    }
                },
            }),
        );

        Self {
            grid,
            start: self.start,
            end: self.end,
        }
    }

    fn successors(&self, pos: &Position, max_t: usize) -> Vec<(Position, usize)> {
        assert!(self.grid[pos].t.map_or(true, |t| t >= max_t));
        let s = mygrid::CARDINAL_DIRECTIONS
            .iter()
            .filter(|&dir| {
                self.grid
                    .at(&pos.step(dir))
                    .is_some_and(|v| v.t.map_or(true, |t| t >= max_t))
            })
            .map(|dir| (pos.step(&dir), 1))
            .collect();
        s
    }

    fn heuristic(&self, pos: &Position) -> usize {
        (self.end.x - pos.x).abs() as usize + (self.end.y - pos.y).abs() as usize
    }

    fn success(&self, pos: &Position) -> bool {
        pos == &self.end
    }

    fn minimum_steps(&self, max_t: usize) -> Option<usize> {
        //println!("{}", self.with_max_t(max_t).grid.to_string());
        let (_, cost) = astar::astar(
            &self.start,
            |n| self.successors(n, max_t),
            |n| self.heuristic(n),
            |n| self.success(n),
        )?;

        Some(cost)
    }

    fn first_blocking_byte(&self) -> Position {
        let max_t = self.grid.iter().filter_map(|c| c.t).max().unwrap();
        for t in 0..=max_t {
            if self.minimum_steps(t).is_none() {
                return self
                    .grid
                    .iter_positions()
                    .find(|pos| self.grid[pos].t == Some(t - 1))
                    .unwrap();
            }
        }
        panic!();
    }
}

fn parse_input(x_size: usize, y_size: usize, data: &str) -> Maze {
    Maze::from(x_size, y_size, data)
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
    let maze: Maze = parse_input(71, 71, &data);
    println!("minimum steps: {}", maze.minimum_steps(1024).unwrap());
    println!(
        "first blocking byte: {}",
        maze.first_blocking_byte().to_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(7, 7, &data);
        assert_eq!(maze.minimum_steps(12), Some(22));
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(71, 71, &data);
        assert_eq!(maze.minimum_steps(1024), Some(416));
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze = parse_input(7, 7, &data);
        assert_eq!(maze.first_blocking_byte(), Position::new(6, 1));
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(71, 71, &data);
        assert_eq!(maze.first_blocking_byte(), Position::new(50, 23));
    }
}
