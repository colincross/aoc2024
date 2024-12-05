use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn parse_input(data: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let orders = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .fold(
            HashMap::<u32, HashSet<u32>>::new(),
            |mut map, (from, to)| {
                map.entry(from).or_default().insert(to);
                map
            },
        );

    let pages = data
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (orders, pages)
}

fn page_is_valid(orders: &HashMap<u32, HashSet<u32>>, page: u32, before: &[u32]) -> bool {
    let Some(page_must_be_before) = orders.get(&page) else {
        return true;
    };

    !before.iter().any(|&a| page_must_be_before.contains(&a))
}

fn valid_pages(orders: &HashMap<u32, HashSet<u32>>, pages: &[u32]) -> bool {
    pages
        .iter()
        .enumerate()
        .all(|(i, &page)| page_is_valid(orders, page, &pages[..i]))
}

fn sum_of_middle_digits_of_valid_updates(
    orders: &HashMap<u32, HashSet<u32>>,
    pages: &[Vec<u32>],
) -> u32 {
    pages
        .iter()
        .filter(|&pages| valid_pages(orders, pages))
        .map(|v| v[v.len() / 2])
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
    let (orders, pages) = parse_input(&data);
    println!(
        "sum of middle digits of valid updates: {}",
        sum_of_middle_digits_of_valid_updates(&orders, &pages)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (orders, pages) = parse_input(&data);
        let sum = sum_of_middle_digits_of_valid_updates(&orders, &pages);
        assert_eq!(sum, 143);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (orders, pages) = parse_input(&data);
        let sum = sum_of_middle_digits_of_valid_updates(&orders, &pages);
        assert_eq!(sum, 5275);
    }
}
