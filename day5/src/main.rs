use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input.txt");
    let lowest = part_2(&input);
    println!("lowest location: {}", lowest)
}

fn part_2(input: &str) -> i64 {
    let mut iter = input.lines().into_iter();
    let (_, seeds) = iter.next().unwrap().split_once(": ").unwrap();

    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seeds: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| {
            if let [start, len] = chunk {
                let range = *start..*start + *len;
                range
            } else {
                panic!()
            }
        })
        .collect();

    println!("len: {:?}", seeds);
    let map = create_map(input);
    let results = seeds
        .iter()
        .map(|seed| {
            //
            let min = seed.clone().map(|s| transform(s, &map)).min().unwrap();
            println!("min: {}", min);
            min
        })
        .collect::<Vec<i64>>();

    *results.iter().min().unwrap()
}

#[derive(Clone, Copy, Debug)]
struct Range {
    destination: i64,
    source: i64,
    length: i64,
}
fn part_1(input: &str) -> i64 {
    let mut iter = input.lines().into_iter();
    let (_, seeds) = iter.next().unwrap().split_once(": ").unwrap();

    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let map = create_map(input);
    let results = seeds
        .iter()
        .map(|seed| transform(*seed, &map))
        .collect::<Vec<i64>>();

    *results.iter().min().unwrap()
}

#[test]
fn test() {
    let input = fs::read_to_string("test.txt").expect("Missing input.txt");
    let map = create_map(&input);
    let result = transform(82, &map);
    assert_eq!(result, 46_i64);
}
fn create_map(input: &str) -> HashMap<i64, Vec<Range>> {
    let mut iter = input.lines().into_iter();
    iter.next();
    iter.next();

    let mut map: HashMap<i64, Vec<Range>> = HashMap::new();
    let mut map_index = 0_i64;
    while let Some(line) = iter.next() {
        match line {
            "seed-to-soil map:" => map_index = 0,
            "soil-to-fertilizer map:" => map_index = 1,
            "fertilizer-to-water map:" => map_index = 2,
            "water-to-light map:" => map_index = 3,
            "light-to-temperature map:" => map_index = 4,
            "temperature-to-humidity map:" => map_index = 5,
            "humidity-to-location map:" => map_index = 6,
            "" => (),
            _ => parse(line, map_index, &mut map),
        };
    }
    map
}

fn transform(seed: i64, map: &HashMap<i64, Vec<Range>>) -> i64 {
    let mut result = seed;
    for i in 0..=6 {
        let ranges = map.get(&i).unwrap();
        for range in ranges {
            if range.source <= result && range.source + range.length > result {
                // range found
                result = result + (range.destination - range.source);
                break;
            }
        }
        // if not found, no
    }
    result
}

fn parse(line: &str, map_index: i64, map: &mut HashMap<i64, Vec<Range>>) {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let range = Range {
        destination: numbers[0],
        source: numbers[1],
        length: numbers[2],
    };
    map.entry(map_index)
        .and_modify(|vec| vec.push(range))
        .or_insert(vec![range]);
}
