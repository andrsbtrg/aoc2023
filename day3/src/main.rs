use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum_part_numbers: u32 = part_2(&input);

    println!("total: {}", sum_part_numbers);
}

fn part_2(input: &str) -> u32 {
    // just find the '*'
    let mut gears: Vec<Pos> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let indices: Vec<usize> = line.match_indices('*').map(|(i, _)| i).collect();
        indices.iter().for_each(|i| {
            let pos = Pos {
                x: *i as i32,
                y: y as i32,
            };
            gears.push(pos);
        })
    });
    let parts = find_parts(&input);

    let mut hash: HashMap<Pos, Vec<u32>> = HashMap::new();
    parts.iter().for_each(|part| {
        if let Some(gear) = is_symbol_around(part, &gears) {
            if let Some(count) = hash.get_mut(gear) {
                count.push(part.value);
            } else {
                hash.insert(gear.clone(), vec![part.value]);
            }
        }
    });
    let mut total = 0;

    hash.iter().for_each(|(_, parts)| {
        if parts.len() == 2 {
            total += parts.iter().product::<u32>();
        }
    });

    total
}

fn find_parts(input: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    input.lines().enumerate().for_each(|(j, line)| {
        let mut start = true;
        let mut start_pos = Pos::zero();
        let mut curr_number = Vec::new();

        for (_i, c) in line.char_indices() {
            if c.is_numeric() {
                let n: u32 = c.to_digit(10).unwrap();
                if start {
                    curr_number = Vec::new();
                    start_pos = Pos::new(_i as i32, j as i32);
                    start = false;
                }
                curr_number.push(n);
                if _i == line.len() - 1 {
                    let part = Part::new(curr_number, start_pos);
                    parts.push(part);
                    curr_number = Vec::new();
                    start = true;
                    start_pos = Pos::zero();
                }
            }
            // is either symbol or .
            else {
                if curr_number.len() > 0 {
                    let part = Part::new(curr_number, start_pos);
                    parts.push(part);
                    curr_number = Vec::new();
                    start = true;
                    start_pos = Pos::zero();
                }
            }
        }
    });
    parts
}

fn part_1(input: &str) -> u32 {
    // find symbols location
    let mut locs: Vec<Pos> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let indices: Vec<usize> = SYMBOLS
            .iter()
            .map(|c| {
                let ii: Vec<_> = line.match_indices(*c).map(|(i, _)| i).collect();
                ii
            })
            .flatten()
            .collect();
        indices.iter().for_each(|i| {
            let pos = Pos {
                x: *i as i32,
                y: y as i32,
            };
            locs.push(pos);
        })
    });

    // look in every direction for numbers
    let parts = find_parts(input);
    // if a number find, get the total number by lookinf left and right
    let mut total = 0;
    parts.iter().for_each(|part| {
        if is_symbol_around(part, &locs).is_some() {
            total += part.value;
        }
    });
    // add number
    total
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
    fn zero() -> Self {
        Pos { x: 0, y: 0 }
    }
    fn get_neighbours(&self) -> Vec<Pos> {
        let mut neighbours = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip the case when both dx and dy are 0 (the center position)
                if dx != 0 || dy != 0 {
                    neighbours.push(Pos {
                        x: self.x + dx,
                        y: self.y + dy,
                    });
                }
            }
        }

        neighbours
    }
}

#[derive(Debug)]
struct Part {
    start: Pos,
    value: u32,
    len: u32,
}

impl Part {
    fn new(part_numbers: Vec<u32>, start: Pos) -> Self {
        let part_number = part_number(&part_numbers);
        let len = part_numbers.len() as u32;
        Part {
            value: part_number,
            start,
            len,
        }
    }
}
const SYMBOLS: [char; 11] = ['!', '/', '#', '$', '%', '&', '*', '+', '@', '-', '='];

fn is_symbol_around<'a>(part: &'a Part, locs: &'a [Pos]) -> Option<&'a Pos> {
    let x_start = if part.start.x == 0 {
        0
    } else {
        part.start.x - 1
    };
    let x_end = part.start.x + part.len as i32;
    let y = part.start.y;
    // check up
    if y > 0 {
        let yy = y - 1;
        for i in x_start..=x_end {
            if let Some(found) = locs.iter().find(|sym| sym.x == i && sym.y == yy) {
                return Some(found);
            }
        }
    }
    // check same line
    let left_right = vec![x_start, x_end];
    for pos in left_right.iter() {
        if let Some(found) = locs.iter().find(|sym| sym.x == *pos && sym.y == y) {
            return Some(found);
        }
    }
    // check down

    let yy = y + 1;
    for i in x_start..=x_end {
        if let Some(found) = locs.iter().find(|sym| sym.x == i && sym.y == yy) {
            return Some(found);
        }
    }
    None
}

fn part_number(number: &Vec<u32>) -> u32 {
    let mut tens = number.len() as u32;
    let mut total = 0;
    number.iter().for_each(|n| {
        total += (10_u32.pow(tens - 1)) * n;
        tens -= 1;
    });
    total
}

#[test]
fn test_part_number() {
    let number = vec![6, 6, 4];
    assert_eq!(664, part_number(&number));
}
