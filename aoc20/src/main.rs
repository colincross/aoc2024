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
    #[allow(unused)]
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

    fn count_cheats_from_start_that_save(
        &self,
        start: &Position,
        save: usize,
        max_cheat: usize,
    ) -> usize {
        let mut count = 0;
        let m = max_cheat as i32;
        for x in -m..=m {
            for y in -m..=m {
                let cheat_len = x.abs() as usize + y.abs() as usize;
                if cheat_len > max_cheat {
                    continue;
                }
                let pos = Position::new(start.x + x, start.y + y);
                let distance_with_cheat = self.distance[start] + cheat_len;
                if self.grid.valid_pos(&pos)
                    && self.distance[&pos]
                        .checked_sub(distance_with_cheat)
                        .unwrap_or(0)
                        >= save
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_cheats_that_save(&self, save: usize, max_cheat: usize) -> usize {
        self.path
            .iter()
            .map(|pos| self.count_cheats_from_start_that_save(pos, save, max_cheat))
            .sum()
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

    println!(
        "count of 2 picosecond cheats: {}",
        maze.count_cheats_that_save(100, 2)
    );
    println!(
        "count of 20 picosecond cheats: {}",
        maze.count_cheats_that_save(100, 20)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(&data);
        assert_eq!(maze.count_cheats_that_save(64, 2), 1);
        assert_eq!(maze.count_cheats_that_save(40, 2), 2);
        assert_eq!(maze.count_cheats_that_save(38, 2), 3);
        assert_eq!(maze.count_cheats_that_save(36, 2), 4);
        assert_eq!(maze.count_cheats_that_save(20, 2), 5);
        assert_eq!(maze.count_cheats_that_save(12, 2), 8);
        assert_eq!(maze.count_cheats_that_save(10, 2), 10);
        assert_eq!(maze.count_cheats_that_save(8, 2), 14);
        assert_eq!(maze.count_cheats_that_save(6, 2), 16);
        assert_eq!(maze.count_cheats_that_save(4, 2), 30);
        assert_eq!(maze.count_cheats_that_save(2, 2), 44);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(&data);

        assert_eq!(maze.count_cheats_that_save(100, 2), 1346);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let maze: Maze = parse_input(&data);
        assert_eq!(maze.count_cheats_that_save(76, 20), 3);
        assert_eq!(maze.count_cheats_that_save(74, 20), 7);
        assert_eq!(maze.count_cheats_that_save(72, 20), 29);
        assert_eq!(maze.count_cheats_that_save(70, 20), 41);
        assert_eq!(maze.count_cheats_that_save(68, 20), 55);
        assert_eq!(maze.count_cheats_that_save(66, 20), 67);
        assert_eq!(maze.count_cheats_that_save(64, 20), 86);
        assert_eq!(
            maze.count_cheats_that_save(50, 20),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let maze: Maze = parse_input(&data);

        assert_eq!(maze.count_cheats_that_save(100, 20), 985482);
    }
}
