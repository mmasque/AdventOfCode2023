use itertools::Itertools;

use crate::helpers::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct Number {
    value: i32,
    line_num: i32,
    start_col: i32,
    end_col: i32,
}

struct Grid {
    grid: Vec<Vec<Entry>>,
}

#[derive(Clone)]
struct Entry {
    value: char,
    col_num: i32,
    line_num: i32,
    number: Option<Number>, //hacky, but if an entry is a number, we can put it here
}
impl Entry {
    fn new(value: char, col_num: i32, line_num: i32, number: Option<Number>) -> Entry {
        Entry {
            value,
            col_num,
            line_num,
            number,
        }
    }
}
impl Grid {
    fn get_at(&self, line: i32, col: i32) -> Option<Entry> {
        let line: usize = line.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        self.grid.get(line)?.get(col).cloned()
    }
}

trait Adjacent {
    fn adjacent_entries(&self, grid: &Grid) -> Vec<Entry>;
}

impl Adjacent for Number {
    fn adjacent_entries(&self, grid: &Grid) -> Vec<Entry> {
        let left = grid.get_at(self.line_num, self.start_col - 1);
        let right = grid.get_at(self.line_num, self.end_col + 1);
        let range_above_below = (self.start_col - 1)..=(self.end_col + 1);
        let above: Vec<Option<Entry>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num - 1, i))
            .collect();
        let below: Vec<Option<Entry>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num + 1, i).into())
            .collect();

        [vec![left, right], above, below]
            .concat()
            .iter()
            .flatten()
            .cloned()
            .collect()
    }
}

impl dyn Adjacent {
    fn adjacent_symbols(&self, grid: &Grid) -> Vec<Entry> {
        self.adjacent_entries(grid)
            .iter()
            .filter(|x| x.value.ne(&&'.') && !x.value.is_ascii_digit())
            .cloned()
            .collect()
    }
    fn has_adjacent_symbols(&self, grid: &Grid) -> bool {
        !self.adjacent_symbols(grid).is_empty()
    }
}

impl Adjacent for Entry {
    fn adjacent_entries(&self, grid: &Grid) -> Vec<Entry> {
        let left = grid.get_at(self.line_num, self.col_num - 1);
        let right = grid.get_at(self.line_num, self.col_num + 1);
        let range_above_below = (self.col_num - 1)..=(self.col_num + 1);
        let above: Vec<Option<Entry>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num - 1, i))
            .collect();
        let below: Vec<Option<Entry>> = range_above_below
            .clone()
            .map(|i| grid.get_at(self.line_num + 1, i).into())
            .collect();

        [vec![left, right], above, below]
            .concat()
            .iter()
            .flatten()
            .cloned()
            .collect()
    }
}

fn add_numbers_to_grid(grid: &mut Grid) -> Vec<Number> {
    // find the numbers in each line and turn them into entries
    // also adds the numbers found to the grid entries
    let mut all_numbers = vec![];
    for (line_num, line) in grid.grid.iter_mut().enumerate() {
        let chunks = line
            .iter_mut()
            .enumerate()
            .chunk_by(|(_, s)| s.value.is_ascii_digit());
        let number_chunks = chunks.into_iter().filter(|(k, _)| *k);
        let mut numbers: Vec<Vec<(usize, &mut Entry)>> =
            number_chunks.map(|(_, chunk)| chunk.collect()).collect();

        for number_vector in numbers.iter_mut() {
            let value: i32 = number_vector
                .iter()
                .fold("".to_string(), |s, (_, c)| s + &c.value.to_string())
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
            all_numbers.push(number.clone());
            // add the number to each entry
            for entry in number_vector.iter_mut() {
                entry.1.number = Some(number.clone());
            }
        }
    }
    all_numbers
}

fn build_grid(lines: Vec<String>) -> Grid {
    Grid {
        grid: lines
            .iter()
            .enumerate()
            .map(|(i, s)| {
                s.chars()
                    .enumerate()
                    .map(|(j, c)| Entry::new(c, j as i32, i as i32, None))
                    .collect()
            })
            .collect(),
    }
}

pub fn day3a() {
    let lines = read_lines("inputs/day3.txt");
    let mut grid = build_grid(lines);
    let all_numbers = add_numbers_to_grid(&mut grid);
    let sum_of_part_numbers = all_numbers
        .iter()
        .filter(|n| (*n as &dyn Adjacent).has_adjacent_symbols(&grid)) // This is not great
        .fold(0, |c, n| c + n.value);

    println!("DAY3A: Sum of part numbers: {}", sum_of_part_numbers);
}

pub fn day3b() {
    let lines = read_lines("inputs/day3.txt");
    let mut grid = build_grid(lines);
    add_numbers_to_grid(&mut grid);

    let mut total = 0;
    // now for every *, look at its adjacent entries, and see if they are numbers (with a bit of a trick)
    for line in &grid.grid {
        for entry in line {
            if entry.value == '*' {
                let adjacents = entry.adjacent_entries(&grid);
                // hacky: duplicates of numbers will be next to each other, because of how we compute the adjacents
                let dedupped_numbers: Vec<Number> = adjacents
                    .iter()
                    .map(|x| x.number.clone())
                    .flatten()
                    .dedup()
                    .collect();
                if dedupped_numbers.len() == 2 {
                    total += dedupped_numbers.iter().fold(1, |a, b| a * b.value);
                }
            }
        }
    }
    println!("DAY3B: Sum of gear ratios {}", total);
}
