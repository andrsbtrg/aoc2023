use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result: u32 = part_1(&input);
    println!("result: {}", result)
}

fn part_1(input: &str) -> u32 {
    let mut total = 0;
    input.lines().enumerate().for_each(|(_i, line)| {
        let (_, tickets) = line
            .split_once(": ")
            .expect("Wrong input format, expected 'Card n: ...'");
        let (winning, mine) = tickets
            .split_once(" | ")
            .expect("Wrong input format, expected 'nn | nn ...'");

        let winning: HashSet<u32> = winning
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mine: Vec<u32> = mine
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let matches: u32 = mine
            .iter()
            .map(|n| {
                if winning.contains(n) {
                    return 1;
                }
                return 0;
            })
            .sum();
        let score = if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        };
        total += score;
    });
    total
}
