use mygrid::{Grid, Position};
use pathfinding::directed::astar;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Cell {
    t: usize,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self.t {
            0 => ".".to_string(),
            _ => "#".to_string(),
        }
    }
}

struct Maze {
    grid: Grid<Cell>,
    start: Position,
    end: Position,
}

impl Maze {
    fn from(x_size: usize, y_size: usize, data: &str, max_t: usize) -> Self {
        let mut grid = Grid::<Cell>::new(x_size, y_size);

        let start = Position::new(0, 0);
        let end = Position::new(x_size as i32 - 1, y_size as i32 - 1);

        for (t, line) in data.lines().enumerate().take(max_t) {
            let (x, y) = line.split_once(",").unwrap();
            let pos = Position::new(x.parse().unwrap(), y.parse().unwrap());
            grid[&pos] = Cell { t: t + 1 };
        }

        Self { grid, start, end }
    }

    fn successors(&self, pos: &Position) -> Vec<(Position, usize)> {
        assert_eq!(self.grid[pos].t, 0);
        let s = mygrid::CARDINAL_DIRECTIONS
            .iter()
            .filter(|&dir| self.grid.at(&pos.step(dir)).is_some_and(|v| v.t == 0))
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

    fn minimum_steps(&self) -> usize {
        println!("{}", self.grid.to_string());
        let Some((_, cost)) = astar::astar(
            &self.start,
            |n| self.successors(n),
            |n| self.heuristic(n),
            |n| self.success(n),
        ) else {
            panic!()
        };

        cost
    }
}

fn parse_input(x_size: usize, y_size: usize, data: &str, max_t: usize) -> Maze {
    Maze::from(x_size, y_size, data, max_t)
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
    let maze: Maze = parse_input(71, 71, &data, 1024);
    println!("minimum steps: {}", maze.minimum_steps());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(7, 7, &data, 12);
        assert_eq!(maze.minimum_steps(), 22);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(71, 71, &data, 1024);
        assert_eq!(maze.minimum_steps(), 416);
    }
}
