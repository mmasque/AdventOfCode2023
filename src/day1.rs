use std::slice::Windows;

use crate::helpers::read_lines;

pub fn day1a() {
    // read file
    let lines = read_lines("inputs/day1.txt");
    let mut total = 0;
    for line in lines {
        let mut digits = line.chars().filter(|c| c.is_ascii_digit()).peekable();
        let first = digits.peek().expect("No first character").to_string();
        let last = digits.last().expect("No last character").to_string();
        let number: i32 = (first + &last).parse().expect("Not a string");
        total += number;
    }
    println!("The result is {}", total);
}

fn digit_from_word(word: &[char]) -> Option<i32> {
    let word = word.iter().collect::<String>();
    match word.as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn add_if_digit(windows: &mut Windows<char>, digits: &mut Vec<i32>) {
    let window = windows.next();
    // if the window is a word then add it to the list
    let digit = window.and_then(digit_from_word);
    if let Some(d) = digit {
        digits.push(d)
    }
}

/// Parses a string for digits, both spelled out and numerical
fn parse_digits(input: String) -> Vec<i32> {
    // sliding window to find digits
    // max length: 5, min length: 3
    // dumbest: length 3, then length 4, then length 5
    let mut digits = vec![];

    let slice = &input.chars().collect::<Vec<char>>()[..];
    let mut input_iter = input.chars();
    let mut threes = slice.windows(3);
    let mut fours = slice.windows(4);
    let mut fives = slice.windows(5);
    loop {
        // if the current char is a digit then add it to the list
        if let Some(d) = input_iter.next() {
            if let Some(d) = d.to_string().parse().ok() {
                digits.push(d);
            }
        } else {
            break;
        }

        add_if_digit(&mut threes, &mut digits);
        add_if_digit(&mut fours, &mut digits);
        add_if_digit(&mut fives, &mut digits);
    }
    digits
}

pub fn day1b() {
    // read file
    let lines = read_lines("inputs/day1.txt");
    let mut total = 0;
    for line in lines {
        let digits = parse_digits(line);
        let mut digits = digits.iter().peekable();
        let first = digits.peek().expect("No first character").to_string();
        let last = digits.last().expect("No last character").to_string();
        let number: i32 = (first + &last).parse().expect("Not a string");
        total += number;
    }
    println!("Total: {}", total);
}
