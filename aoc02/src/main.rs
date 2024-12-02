use std::env;
use std::fs::read_to_string;

fn is_safe(diffs: &Vec<i64>) -> bool {
    let all_positive = diffs.iter().all(|&x| x > 0);
    let all_negative = diffs.iter().all(|&x| x < 0);
    let max_magnitude = diffs.iter().map(|x| x.abs()).max().unwrap();
    let safe = (all_positive || all_negative) && max_magnitude <= 3;

    dbg!(&diffs, all_positive, all_negative, max_magnitude, safe);
    safe
}

fn safe_reports(reports: &Vec<Vec<i64>>) -> usize {
    let report_diffs = reports
        .iter()
        .map(|report| {
            report
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    report_diffs.iter().filter(|diffs| is_safe(diffs)).count()
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
}
