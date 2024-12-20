use mygrid::{Grid, Position};
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
    distance: Grid<usize>,
    path: Vec<Position>,
    #[allow(unused)]
    start: Position,
    end: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Position,
    cheat: bool,
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

        let mut distance = Grid::<usize>::new(grid.x_size, grid.y_size);
        let mut pos = start;
        let mut prev_pos = start;
        let mut i = 0;
        let mut path = vec![];
        while pos != end {
            path.push(pos);
            distance[&pos] = i;
            i += 1;
            let next_pos = grid
                .iter_neighbor_positions(&pos)
                .find(|next_pos| {
                    next_pos != &prev_pos && grid.at(next_pos).map_or(false, |&c| c == Cell::Empty)
                })
                .unwrap();
            prev_pos = pos;
            pos = next_pos;
        }
        distance[&pos] = i;

        Self {
            grid,
            distance,
            path,
            start,
            end,
        }
    }

    fn count_cheats_that_save(&self, save: usize) -> usize {
        let min_time = self.distance[&self.end];

        let mut count = 0;

        for pos in self.path.iter() {
            count += mygrid::CARDINAL_DIRECTIONS
                .iter()
                .map(|dir| pos.step(&dir).step(&dir))
                .filter(|cheat_end_pos| {
                    self.grid
                        .at(cheat_end_pos)
                        .map_or(false, |&c| c == Cell::Empty)
                })
                .map(|cheat_end_pos| {
                    self.distance[&pos] + (min_time - self.distance[&cheat_end_pos] + 1)
                })
                .filter(|&t| t < min_time && min_time - t >= save)
                .count();
        }

        count
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

    println!("lowest score: {}", maze.count_cheats_that_save(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(&data);
        assert_eq!(maze.count_cheats_that_save(64), 1);
        assert_eq!(maze.count_cheats_that_save(40), 2);
        assert_eq!(maze.count_cheats_that_save(38), 3);
        assert_eq!(maze.count_cheats_that_save(36), 4);
        assert_eq!(maze.count_cheats_that_save(20), 5);
        assert_eq!(maze.count_cheats_that_save(12), 8);
        assert_eq!(maze.count_cheats_that_save(10), 10);
        assert_eq!(maze.count_cheats_that_save(8), 14);
        assert_eq!(maze.count_cheats_that_save(6), 16);
        assert_eq!(maze.count_cheats_that_save(4), 30);
        assert_eq!(maze.count_cheats_that_save(2), 44);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(&data);

        assert_eq!(maze.count_cheats_that_save(100), 1346);
    }
}
