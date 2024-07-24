use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn read_lines<T>(path: T) -> Vec<String>
where
    T: AsRef<Path>
{
    let file = File::open(path).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Bad line")).collect()
}