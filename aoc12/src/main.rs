use itertools::Itertools;
use mygrid::*;
use std::fs::read_to_string;

fn parse_input(data: &str) -> Grid<u8> {
    Grid::<u8>::from_bytes(data)
}

fn total_fence_price(grid: &Grid<u8>) -> (usize, usize) {
    let mut visited = Grid::<bool>::new(grid.x_size, grid.y_size);
    let mut total_fence_price = 0;
    let mut bulk_fence_price = 0;
    for pos in grid.iter_positions() {
        if !visited.at(&pos).expect("valid") {
            let start_value = grid.at(&pos).expect("valid");
            let in_same_region = |_, v| v == start_value;
            let mut region_size = 0;
            let mut region_border_len = 0;
            let mut region_grid = Grid::<bool>::new(grid.x_size, grid.y_size);
            for (in_region_pos, _) in grid.iter_region(&pos, in_same_region) {
                *visited.at_mut(&in_region_pos).expect("valid") = true;
                *region_grid.at_mut(&in_region_pos).expect("valid") = true;
                region_size += 1;
                let not_in_region_neighbors = grid
                    .iter_neighbor_positions(&in_region_pos)
                    .filter(|pos| !grid.at(pos).is_some_and(|v| v == start_value))
                    .count();
                region_border_len += not_in_region_neighbors;
            }

            let mut region_corners = 0;
            for (x1, x2) in (-1..(grid.x_size + 1) as i32).tuple_windows() {
                for (y1, y2) in (-1..(grid.y_size + 1) as i32).tuple_windows() {
                    let xy_in_same_region = |x, y| -> bool {
                        region_grid
                            .at(&Position { x, y })
                            .and_then(|&v| Some(v))
                            .unwrap_or_default()
                    };
                    let in_same_region_count = xy_in_same_region(x1, y1) as usize
                        + xy_in_same_region(x1, y2) as usize
                        + xy_in_same_region(x2, y1) as usize
                        + xy_in_same_region(x2, y2) as usize;
                    region_corners += match in_same_region_count {
                        0 => 0, // exterior
                        1 => 1, // outside corner
                        2 => {
                            if xy_in_same_region(x1, y1) && xy_in_same_region(x2, y2)
                                || xy_in_same_region(x1, y2) && xy_in_same_region(x2, y1)
                            {
                                2 // shared corner
                            } else {
                                0 // edge
                            }
                        }
                        3 => 1, // inside corner
                        4 => 0, // interior
                        _ => panic!(),
                    }
                }
            }

            //dbg!(&pos, start_value, region_size, region_corners);

            total_fence_price += region_size * region_border_len;
            bulk_fence_price += region_size * region_corners;
        }
    }
    (total_fence_price, bulk_fence_price)
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
    let (total_fence_price, bulk_fence_price) = total_fence_price(&grid);
    println!("total fence price: {}", total_fence_price);
    println!("bulk fence price: {}", bulk_fence_price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = parse_input(&data);
        let (total_fence_price, _) = total_fence_price(&grid);
        assert_eq!(total_fence_price, 1930);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = parse_input(&data);
        let (total_fence_price, _) = total_fence_price(&grid);
        assert_eq!(total_fence_price, 1465112);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let grid = parse_input(&data);
        let (_, bulk_fence_price) = total_fence_price(&grid);
        assert_eq!(bulk_fence_price, 1206);
    }

    #[test]
    fn test_part2_1() {
        let data = "AAAA\nBBCD\nBBCC\nEEEC";
        let grid = parse_input(&data);
        let (_, bulk_fence_price) = total_fence_price(&grid);
        assert_eq!(bulk_fence_price, 80);
    }

    #[test]
    fn test_part2_2() {
        let data = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        let grid = parse_input(&data);
        let (_, bulk_fence_price) = total_fence_price(&grid);
        assert_eq!(bulk_fence_price, 436);
    }

    #[test]
    fn test_part2_3() {
        let data = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";
        let grid = parse_input(&data);
        let (_, bulk_fence_price) = total_fence_price(&grid);
        assert_eq!(bulk_fence_price, 368);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let grid = parse_input(&data);
        let (_, bulk_fence_price) = total_fence_price(&grid);
        assert_eq!(bulk_fence_price, 893790);
    }
}
