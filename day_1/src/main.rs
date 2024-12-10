use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut column_1: Vec<i32> = Vec::new();
    let mut column_2: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.split_whitespace().collect();
        column_1.push(chunks[0].parse::<i32>().unwrap());
        column_2.push(chunks[1].parse::<i32>().unwrap());
    }
    column_1.sort();
    column_2.sort();
    let mut total = 0;
    for i in 0..column_1.len() {
        let diff = (column_1[i] - column_2[i]).abs();
        total += diff;
    }
    println!("Total: {}", total);
}
