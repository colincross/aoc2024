use std::env;
use std::fs::read_to_string;

fn is_safe(report: &Vec<i64>) -> bool {
    let diffs = report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    let all_positive = diffs.iter().all(|&x| x > 0);
    let all_negative = diffs.iter().all(|&x| x < 0);
    let max_magnitude = diffs.iter().map(|x| x.abs()).max().unwrap();
    let safe = (all_positive || all_negative) && max_magnitude <= 3;

    safe
}

fn is_safe_with_dampener(report: &Vec<i64>) -> bool {
    (0..report.len())
        .map(|i| [&report[0..i], &report[i + 1..report.len()]].concat())
        .any(|dampened| is_safe(&dampened))
}

fn safe_reports(reports: &Vec<Vec<i64>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn safe_reports_with_dampener(reports: &Vec<Vec<i64>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reports = read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(str::split_whitespace)
        .map(|x| x.map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect();

    println!("safe reports: {}", safe_reports(&reports));
    println!(
        "safe reports with dampener: {}",
        safe_reports_with_dampener(&reports)
    );
}
