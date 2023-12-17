use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result: u32 = part_2(&input);
    println!("result: {}", result)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Scratchcard {
    index: usize,
    matches: u32,
}
impl Scratchcard {
    fn new(index: usize, winning: &HashSet<u32>, mine: Vec<u32>) -> Self {
        let matches = Scratchcard::calculate_matches(winning, mine);
        Scratchcard { index, matches }
    }
    fn calculate_matches(winning: &HashSet<u32>, mine: Vec<u32>) -> u32 {
        let matches: u32 = mine
            .iter()
            .map(|n| {
                if winning.contains(n) {
                    return 1;
                }
                return 0;
            })
            .sum();
        matches
    }
}

fn part_2(input: &str) -> u32 {
    let mut cards = vec![];
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
        let card = Scratchcard::new(_i, &winning, mine);
        cards.push(card);
    });
    let mut map = Vec::<u32>::new();
    // populate map with starting value of 1
    for _i in 0..cards.len() {
        map.push(1);
    }

    for i in 0..cards.len() {
        let wins = cards[i].matches;
        for _ in 0..map[i] {
            for ii in 0..wins {
                map[i + ii as usize + 1] += 1;
            }
        }
    }
    let sum = map.iter().sum();
    sum
}
#[allow(dead_code)]
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
