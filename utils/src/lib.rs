use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn read_file_to_line_vec(path: PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines_vec = Vec::new();
    for line in reader.lines() {
        lines_vec.push(line.unwrap());
    }
    lines_vec
}
