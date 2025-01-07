use itertools::Itertools;
use std::path::PathBuf;
use utils::read_file_to_line_vec;

fn read_input(file_name: PathBuf) -> Vec<Vec<i32>> {
    let input = read_file_to_line_vec(file_name);
    let reports: Vec<Vec<i32>> = input
        .iter()
        .map(|l| {
            l.split(" ")
                .map(|r| r.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    reports
}

fn main() {
    let reports = read_input("src/input".into());

    let num_reports_safe = reports
        .iter()
        .filter(|&r| {
            *r == r.iter().sorted().map(|i| *i).collect::<Vec<i32>>()
                || *r == r.iter().sorted().map(|i| *i).rev().collect::<Vec<i32>>()
        })
        .map(|r| {
            r[0..r.len() - 1]
                .iter()
                .zip(r[1..r.len()].iter())
                .map(|r| r.0 - r.1)
                .map(|d| d.wrapping_abs())
                .collect::<Vec<_>>()
        })
        .filter(|r| r.iter().max() <= Some(&3))
        .filter(|r| r.iter().min() >= Some(&1))
        .count();

    println!("There are {} safe reports", num_reports_safe);
}
