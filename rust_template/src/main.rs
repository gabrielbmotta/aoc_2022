use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

fn main() {
    println!("Hello world!");

    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let (mut val1, mut val2) : (char, char) = (' ', ' ');
        sscanf!(&line, "{} {}", val1, val2).unwrap();
        print!("{val1}, {val2}\n");
    }
}
