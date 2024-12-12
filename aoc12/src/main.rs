use mygrid::Grid;
use std::fs::read_to_string;

fn parse_input(data: &str) -> Grid<u8> {
    Grid::<u8>::from_bytes(data)
}

fn total_fence_price(grid: &Grid<u8>) -> usize {
    let mut visited = Grid::<bool>::new(grid.x_size, grid.y_size);
    let mut total_fence_price = 0;
    for pos in grid.iter_positions() {
        if !visited.at(&pos).expect("valid") {
            let start_value = grid.at(&pos).expect("valid");
            let in_same_region = |_, v| v == start_value;
            let region_iter = grid.iter_region(&pos, in_same_region);
            let mut region_size = 0;
            let mut region_border_len = 0;
            for (in_region_pos, _) in region_iter {
                *visited.at_mut(&in_region_pos).expect("valid") = true;
                region_size += 1;
                region_border_len += grid
                    .iter_neighbors(&in_region_pos)
                    .filter(|&(pos, v)| !in_same_region(pos, v))
                    .count();
                if in_region_pos.x == 0
                    || in_region_pos.x == (grid.x_size - 1).try_into().expect("fits i32")
                {
                    region_border_len += 1;
                }
                if in_region_pos.y == 0
                    || in_region_pos.y == (grid.y_size - 1).try_into().expect("fits i32")
                {
                    region_border_len += 1;
                }
            }
            total_fence_price += region_size * region_border_len;
        }
    }
    total_fence_price
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
    println!("total fence price: {}", total_fence_price(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = parse_input(&data);
        assert_eq!(total_fence_price(&grid), 1930);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = parse_input(&data);
        assert_eq!(total_fence_price(&grid), 1465112);
    }
}
