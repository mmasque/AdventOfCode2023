use itertools::Itertools;

use crate::helpers::read_lines;

#[derive(Debug)]
struct Number {
    value: i32,
    line_num: i32,
    start_col: i32,
    end_col: i32,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn get_at(&self, line: i32, col: i32) -> Option<char> {
        let line: usize = line.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        self.grid.get(line)?.get(col).copied()
    }
}

impl Number {
    fn adjacent_chars(&self, grid: &Grid) -> Vec<char> {
        let left = grid.get_at(self.line_num, self.start_col - 1);
        let right = grid.get_at(self.line_num, self.end_col + 1);
        let range_above_below = (self.start_col - 1)..=(self.end_col + 1);
        let above: Vec<Option<char>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num - 1, i))
            .collect();
        let below: Vec<Option<char>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num + 1, i))
            .collect();

        [vec![left, right], above, below]
            .concat()
            .iter()
            .flatten()
            .copied()
            .collect()
    }
    fn adjacent_symbols(&self, grid: &Grid) -> Vec<char> {
        self.adjacent_chars(grid)
            .iter()
            .filter(|x| x.ne(&&'.') && !x.is_ascii_digit())
            .copied()
            .collect()
    }
    fn has_adjacent_symbols(&self, grid: &Grid) -> bool {
        !self.adjacent_symbols(grid).is_empty()
    }
}

pub fn day3a() {
    let lines = read_lines("inputs/day3.txt");
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let grid = Grid { grid };
    // find the numbers in each line and turn them into entries
    let mut all_numbers = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        let numbers: Vec<Vec<(usize, char)>> = line
            .chars()
            .enumerate()
            .chunk_by(|(_, s)| s.is_ascii_digit())
            .into_iter()
            .filter(|(k, _)| *k)
            .map(|(_, chunk)| chunk.collect())
            .collect();
        for number_vector in numbers {
            let value: i32 = number_vector
                .iter()
                .fold("".to_string(), |s, (_, c)| s + &c.to_string())
                .parse()
                .unwrap();
            let start_col = *number_vector
                .iter()
                .peekable()
                .peek()
                .map(|(i, _)| i)
                .unwrap() as i32;
            let end_col = *number_vector.iter().last().map(|(i, _)| i).unwrap() as i32;
            let number = Number {
                value,
                start_col,
                end_col,
                line_num: line_num as i32,
            };
            all_numbers.push(number);
        }
    }
    let sum_of_part_numbers = all_numbers
        .iter()
        .filter(|n| n.has_adjacent_symbols(&grid))
        .fold(0, |c, n| c + n.value);

    println!("DAY3A: Sum of part numbers: {}", sum_of_part_numbers);
}
