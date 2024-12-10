use mygrid::{Grid, Position};
use std::{collections::HashSet, fs::read_to_string, hash::Hash};

fn parse_input(data: &str) -> Grid<u8> {
    let grid_bytes = Grid::<u8>::from_bytes(data);
    Grid::from_iter(
        grid_bytes.x_size,
        grid_bytes.y_size,
        grid_bytes.iter().map(|b| b - b'0'),
    )
}

fn reachable_peaks(grid: &Grid<u8>, pos: &mygrid::Position) -> Option<HashSet<Position>> {
    let Some(n) = grid.at(pos) else { return None };

    if n == 9 {
        return Some(HashSet::<Position>::from([pos.clone()]));
    }

    mygrid::CARDINAL_DIRECTIONS
        .iter()
        .map(|dir| pos.step(dir))
        .filter(|pos| grid.at(&pos).is_some_and(|v| v == n + 1))
        .map(|pos| reachable_peaks(grid, &pos))
        .reduce(|a, b| {
            if let Some(a) = a {
                if let Some(b) = b {
                    Some(a.union(&b).copied().collect())
                } else {
                    Some(a)
                }
            } else if let Some(b) = b {
                Some(b)
            } else {
                None
            }
        })
        .unwrap_or_default()
}

fn trailhead_score(grid: &Grid<u8>, pos: &mygrid::Position) -> usize {
    reachable_peaks(grid, pos).unwrap_or_default().len()
}

fn sum_trailhead_scores(grid: &Grid<u8>) -> usize {
    grid.iter_positions()
        .filter(|pos| grid.at(pos).expect("valid") == 0)
        .map(|pos| trailhead_score(grid, &pos))
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
    let grid = parse_input(&data);
    let sum_trailhead_scores = sum_trailhead_scores(&grid);
    println!("sum of trailhead scores: {}", sum_trailhead_scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = parse_input(&data);
        let sum_trailhead_scores = sum_trailhead_scores(&grid);
        assert_eq!(sum_trailhead_scores, 36);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = parse_input(&data);
        let sum_trailhead_scores = sum_trailhead_scores(&grid);
        assert_eq!(sum_trailhead_scores, 624);
    }
}
