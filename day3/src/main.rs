use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum_part_numbers: u32 = part_1(&input);

    println!("total: {}", sum_part_numbers);
}

#[derive(Debug)]
struct Pos {
    x: u32,
    y: u32,
}
impl Pos {
    fn new(x: u32, y: u32) -> Self {
        Pos { x, y }
    }
    fn zero() -> Self {
        Pos { x: 0, y: 0 }
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
                x: *i as u32,
                y: y as u32,
            };
            locs.push(pos);
        })
    });

    // look in every direction for numbers
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
                    start_pos = Pos::new(_i as u32, j as u32);
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
    // println!("{:?}", parts);
    // if a number find, get the total number by lookinf left and right
    let mut total = 0;
    parts.iter().for_each(|part| {
        if is_symbol_around(part, &locs) {
            total += part.value;
        } else {
            println!("{:?}", part);
        }
    });
    // add number
    total
}

fn is_symbol_around(part: &Part, locs: &[Pos]) -> bool {
    let x_start = if part.start.x == u32::MIN {
        0
    } else {
        part.start.x - 1
    };
    let x_end = part.start.x + part.len;
    let y = part.start.y;
    // check up
    if y > 0 {
        let yy = y - 1;
        for i in x_start..=x_end {
            if locs.iter().find(|sym| sym.x == i && sym.y == yy).is_some() {
                return true;
            }
        }
    }
    // check same line
    let left_right = vec![x_start, x_end];
    for pos in left_right.iter() {
        if locs
            .iter()
            .find(|sym| sym.x == *pos && sym.y == y)
            .is_some()
        {
            return true;
        }
    }
    // check down

    let yy = y + 1;
    for i in x_start..=x_end {
        if locs.iter().find(|sym| sym.x == i && sym.y == yy).is_some() {
            return true;
        }
    }
    false
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
