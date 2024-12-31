use std::path::PathBuf;
use utils::read_file_to_line_vec;

fn read_input(file_name: PathBuf) -> (Vec<i32>, Vec<i32>) {
    let mut set_1: Vec<i32> = vec![];
    let mut set_2: Vec<i32> = vec![];

    for line in read_file_to_line_vec(file_name) {
        let line_split: Vec<&str> = line.split(" ").collect();
        set_1.push(line_split[0].parse::<i32>().unwrap());
        set_2.push(line_split[3].parse::<i32>().unwrap());
    }

    (set_1, set_2)
}

fn main() {
    let (mut set_1, mut set_2) = read_input("src/input".into());
    set_1.sort();
    set_2.sort();

    let distance: i32 = set_1
        .iter()
        .zip(set_2.iter())
        .map(|(i, j)| i - j)
        .map(|i| i.wrapping_abs())
        .sum();

    let similarity_score: i32 = set_1
        .iter()
        .map(|i| i * (set_2.iter().filter(|j| *j == i).count() as i32))
        .sum();

    println!("The distance between the sets is {}", distance);
    println!("The similarrity score is {}", similarity_score);
}
