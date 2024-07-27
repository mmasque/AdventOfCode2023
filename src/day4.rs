use std::{cmp::max, collections::HashMap};

use crate::helpers::read_lines;

fn parse_card(line: &str) -> Card {
    let mut number_card = line.split(":");
    let number: i32 = number_card
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let card = number_card.next().unwrap();
    let mut winners_haves = card.split("|");
    let winners: Vec<i32> = winners_haves
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let haves: Vec<i32> = winners_haves
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Card {
        id: number,
        haves,
        winners,
    }
}

struct Card {
    id: i32,
    haves: Vec<i32>,
    winners: Vec<i32>,
}

pub fn day4a() {
    let lines = read_lines("inputs/day4.txt");
    let mut total = 0;
    for line in lines {
        let mut card = parse_card(&line);
        // check how many winners there are in the haves
        let count = card
            .haves
            .clone()
            .into_iter()
            .filter(|h| card.winners.contains(h))
            .count();

        let power = if count == 0 {
            0
        } else {
            2_usize.pow(max(count - 1, 0) as u32)
        };
        total += power;
    }
    println!("DAY4A: Power total of winners: {}", total);
}

pub fn day4b() {
    let lines = read_lines("inputs/day4.txt");
    // look at a card: compute the cards it spawns
    let mut cards: Vec<Card> = lines.iter().map(|x| parse_card(&x)).collect();
    // compute the winners in each card
    let mut winners: Vec<(usize, usize)> = cards
        .iter()
        .map(|x| {
            (
                x.haves
                    .clone()
                    .into_iter()
                    .filter(|h| x.winners.contains(h))
                    .count(),
                1,
            )
        })
        .collect();

    for i in 0..winners.len() {
        let (count, multiplier) = winners[i];
        for j in (i + 1)..=(i + count) {
            winners[j].1 += multiplier;
        }
    }
    let total = winners.iter().fold(0, |c, x| x.1 + c);
    println!("DAY4B: Total cards {:?}", total);
}
