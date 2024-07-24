use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn read_lines<T>(path: T) -> Vec<String>
where
    T: AsRef<Path>
{
    let file = File::open(path).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Bad line")).collect()
}

pub fn day1() {
    // read file
    let lines = read_lines("inputs/day1a.txt");
    let mut total = 0;
    for line in lines {
        let mut digits  = line.chars().filter(|c| c.is_ascii_digit()).peekable();
        let first = digits.peek().expect("No first character").to_string();
        let last = digits.last().expect("No last character").to_string();
        let number: i32 = (first + &last).parse().expect("Not a string");
        total += number;
    }
    println!("The result is {}", total);
}