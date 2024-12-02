use std::env;
use std::fs::read_to_string;

fn rotate_vector_of_vectors<T>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!vec.is_empty());
    assert!(!vec[0].is_empty());
    (0..vec[0].len())
        .map(|i| (0..vec.len()).map(|j| vec[j][i]).collect())
        .collect()
}

fn list_distance(lines: &Vec<Vec<i64>>) -> i64 {
    let mut lists: Vec<Vec<i64>> = rotate_vector_of_vectors(lines);
    lists.iter_mut().map(|x| x.sort()).last();
    let pairs = rotate_vector_of_vectors(&lists);

    pairs.iter().map(|x| (x[1] - x[0]).abs()).sum()
}

fn list_similarity(lines: &Vec<Vec<i64>>) -> i64 {
    let lists: Vec<Vec<i64>> = rotate_vector_of_vectors(lines);
    lists[0]
        .iter()
        .map(|&i| i * lists[1].iter().filter(|&&j| j == i).count() as i64)
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(str::split_whitespace)
        .map(|x| x.map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect();

    println!("list distance: {}", list_distance(&lines));
    println!("list distance: {}", list_similarity(&lines));
}
