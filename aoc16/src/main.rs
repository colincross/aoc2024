use mygrid::{Direction, Grid, Position};
use pathfinding::directed::astar;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            &Cell::Empty => ".".to_string(),
            &Cell::Wall => "#".to_string(),
        }
    }
}

struct Maze {
    grid: Grid<Cell>,
    start: Position,
    end: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PosAndDir {
    pos: Position,
    dir: Direction,
}

impl Maze {
    fn from(data: &str) -> Self {
        let grid_bytes = Grid::<u8>::from_bytes(&data);

        let start = grid_bytes.find(&b'S').unwrap();
        let end = grid_bytes.find(&b'E').unwrap();

        let grid = Grid::<Cell>::from_iter(
            grid_bytes.x_size,
            grid_bytes.y_size,
            grid_bytes.iter().map(|&b| match b {
                b'#' => Cell::Wall,
                b'.' => Cell::Empty,
                b'S' => Cell::Empty,
                b'E' => Cell::Empty,
                _ => panic!(),
            }),
        );

        Self { grid, start, end }
    }

    fn successors(&self, pos_and_dir: &PosAndDir) -> Vec<(PosAndDir, usize)> {
        assert_eq!(self.grid[&pos_and_dir.pos], Cell::Empty);
        let s = mygrid::CARDINAL_DIRECTIONS
            .iter()
            .filter(|&dir| dir != &pos_and_dir.dir.opposite())
            .filter(|&dir| {
                self.grid
                    .at(&pos_and_dir.pos.step(dir))
                    .is_some_and(|v| v != &Cell::Wall)
            })
            .map(|dir| {
                (
                    PosAndDir {
                        pos: pos_and_dir.pos.step(&dir),
                        dir: dir.clone(),
                    },
                    if dir == &pos_and_dir.dir { 1 } else { 1001 },
                )
            })
            .collect();
        s
    }

    fn heuristic(&self, pos_and_dir: &PosAndDir) -> usize {
        let c = match pos_and_dir.dir {
            mygrid::LEFT => 2000,
            mygrid::RIGHT => 0,
            mygrid::UP => 0,
            mygrid::DOWN => 2000,
            _ => panic!(),
        } + (self.end.x - pos_and_dir.pos.x).abs() as usize
            + (self.end.y - pos_and_dir.pos.y).abs() as usize;
        c
    }

    fn success(&self, pos_and_dir: &PosAndDir) -> bool {
        pos_and_dir.pos == self.end
    }

    fn lowest_score(&self) -> usize {
        let start = PosAndDir {
            pos: self.start,
            dir: mygrid::RIGHT,
        };
        let Some((_, cost)) = astar::astar(
            &start,
            |n| self.successors(n),
            |n| self.heuristic(n),
            |n| self.success(n),
        ) else {
            panic!()
        };

        cost
    }
}

fn parse_input(data: &str) -> Maze {
    Maze::from(data)
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
    let maze: Maze = parse_input(&data);
    println!("lowest score: {}", maze.lowest_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(&data);
        assert_eq!(maze.lowest_score(), 7036);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(&data);
        assert_eq!(maze.lowest_score(), 127520);
    }
}
