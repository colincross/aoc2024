use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Direction {
    x: i32,
    y: i32,
}

const DIRECTIONS: &'static [Direction] = &[
    Direction { x: 1, y: 0 },   // right
    Direction { x: -1, y: 0 },  // left
    Direction { x: 0, y: -1 },  // up
    Direction { x: 0, y: 1 },   // down
    Direction { x: 1, y: -1 },  // up right
    Direction { x: -1, y: -1 }, // up left
    Direction { x: 1, y: 1 },   // down right
    Direction { x: -1, y: 1 },  // down left
];

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn mv(&self, dir: &Direction, count: usize) -> Self {
        let x = self.x + dir.x * count as i32;
        let y = self.y + dir.y * count as i32;
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid<'a> {
    grid: Vec<&'a str>,
    x_size: i32,
    y_size: i32,
}

impl<'a> Grid<'a> {
    fn from(buf: &'a str) -> Self {
        let grid = buf.lines().collect::<Vec<_>>();
        let x_size = grid[0].len().try_into().unwrap();
        let y_size = grid.len().try_into().unwrap();
        Self {
            grid,
            x_size,
            y_size,
        }
    }

    fn at(&self, pos: Position) -> Option<u8> {
        if pos.x >= 0 && pos.x < self.x_size && pos.y >= 0 && pos.y < self.y_size {
            Some(self.grid[pos.y as usize].as_bytes()[pos.x as usize])
        } else {
            None
        }
    }

    fn iter(&'a self) -> impl Iterator<Item = (Position, u8)> + 'a {
        self.grid.iter().enumerate().flat_map(|(y, &line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| (Position::from(x as i32, y as i32), c))
        })
    }

    fn xmas_count(&self, pos: &Position) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&dir| self.xmas_match_dir(pos, dir))
            .count()
    }

    fn xmas_match_dir(&self, pos: &Position, dir: &Direction) -> bool {
        "XMAS"
            .bytes()
            .enumerate()
            .all(|(i, c)| self.at(pos.mv(dir, i)).unwrap_or(0) == c)
    }
}

fn count_of_xmas(grid: &Grid) -> usize {
    grid.iter()
        .filter(|&(_, c)| c == b'X')
        .map(|(pos, _)| grid.xmas_count(&pos))
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
    let grid = Grid::from(&data);
    println!("count of XMAS: {}", count_of_xmas(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = Grid::from(&data);
        let count_of_xmas = count_of_xmas(&grid);
        assert_eq!(count_of_xmas, 18);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = Grid::from(&data);
        let count_of_xmas = count_of_xmas(&grid);
        assert_eq!(count_of_xmas, 2390);
    }
}
