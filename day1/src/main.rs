use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_file_to_vector(file_path: &str) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let mut result: Vec<i64> = Vec::new();
    for line_result in buffer.lines() {
        let line = line_result?;
        result.push(line.parse().unwrap());
    }

    Ok(result)
}

fn main() {
    let vec = read_file_to_vector("input1.txt").unwrap();
    let mut sum: i64 = 0;
    for n in vec {
        sum  = sum + n;
    }
    println!("la suma es: {:?}", sum);
}
