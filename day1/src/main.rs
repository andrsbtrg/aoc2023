fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = part_2(&input);

    println!("{result}");
}

const DIGITS_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[test]
fn test() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    // part_1(input);
    part_2(input);
}

fn find_first(line: &str) -> u32 {
    let mut pos_dig = 10000;
    let mut result_dig = 0;
    for (i, digit_num) in DIGITS.iter().enumerate() {
        if let Some(found) = line.find(digit_num) {
            if found < pos_dig {
                pos_dig = found;
                result_dig = i + 1;
            }
        }
    }
    let mut pos = 10000;
    let mut result = 0;
    for (i, digit) in DIGITS_STR.iter().enumerate() {
        if let Some(found) = line.find(digit) {
            if found < pos {
                pos = found;
                result = i + 1;
            }
        }
    }
    match pos_dig.cmp(&pos) {
        std::cmp::Ordering::Less => result_dig.try_into().unwrap(),
        std::cmp::Ordering::Equal => result_dig.try_into().unwrap(),
        std::cmp::Ordering::Greater => result.try_into().unwrap(),
    }
}

fn part_2(input: &str) -> u32 {
    let mut calibration = Vec::new();

    input.lines().into_iter().for_each(|line| {
        let mut slice = [0, 0];
        slice[0] = find_first(line);
        slice[1] = find_last(line);
        calibration.push(slice);
    });

    let mut result = 0;

    calibration.iter().for_each(|pair| {
        let v = 10 * pair[0] + pair[1];
        println!("{v}");
        result += v;
    });
    result
}

fn find_last(line: &str) -> u32 {
    let mut pos_dig: i32 = -1;
    let mut result_dig = 0;
    for (i, digit) in DIGITS.iter().enumerate() {
        let v: Vec<_> = line.match_indices(digit).map(|(i, _)| i).collect();
        if v.len() > 0 {
            if *v.last().unwrap() as i32 > pos_dig {
                pos_dig = *v.last().unwrap() as i32;
                result_dig = i as i32 + 1;
            }
        }
    }
    let mut pos: i32 = -1;
    let mut result = 0;
    for (i, digit) in DIGITS_STR.iter().enumerate() {
        let v: Vec<_> = line.match_indices(digit).map(|(i, _)| i).collect();
        if v.len() > 0 {
            if *v.last().unwrap() as i32 > pos {
                pos = *v.last().unwrap() as i32;
                result = i + 1;
            }
        }
    }
    match pos_dig.cmp(&pos) {
        std::cmp::Ordering::Greater => result_dig.try_into().unwrap(),
        std::cmp::Ordering::Equal => result_dig.try_into().unwrap(),
        std::cmp::Ordering::Less => result.try_into().unwrap(),
    }
}

fn part_1(input: &str) -> u32 {
    let mut calibration = Vec::new();

    input.lines().into_iter().for_each(|line| {
        let mut slice = [0, 0];
        for c in line.chars() {
            if c.is_numeric() {
                slice[0] = c.to_digit(10).expect("Was not a digit");
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                slice[1] = c.to_digit(10).expect("Was not a digit");
                break;
            }
        }
        calibration.push(slice);
    });

    let mut result = 0;

    calibration.iter().for_each(|pair| {
        let v = 10 * pair[0] + pair[1];
        println!("{v}");
        result += v;
    });
    result
}
