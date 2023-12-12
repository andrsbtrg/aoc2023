fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let total: i32 = part_1(&input);

    println!("total: {total}");
}

const R_MAX: i32 = 12;
const G_MAX: i32 = 13;
const B_MAX: i32 = 14;

fn part_1(input: &str) -> i32 {
    let mut sum_ids = 0;
    input.lines().for_each(|line| {
        let v: Vec<&str> = line.split(": ").collect();

        let game_id: i32 = get_id(v[0]);
        let subsets: Vec<&str> = v[1].split("; ").collect();

        let possible: bool = subsets.iter().all(|s| is_possible(s));

        println!("{} possible: {}", game_id, possible);
        if possible {
            sum_ids += game_id;
        }
    });
    sum_ids
}

// subset consist of number + color separated by comma.
// e.g. 3 blue, 1 red
fn is_possible(subset: &str) -> bool {
    let records: Vec<&str> = subset.split(", ").collect();
    records.iter().all(|r| {
        let opt: Vec<&str> = r.split(" ").collect();
        let n: i32 = opt[0].parse().unwrap();

        let possible = match opt[1] {
            "blue" => n <= B_MAX,
            "red" => n <= R_MAX,
            "green" => n <= G_MAX,
            _ => panic!(),
        };
        possible
    })
}

fn get_id(v: &str) -> i32 {
    let split: Vec<&str> = v.split(' ').collect();

    let n: i32 = split[1].parse().unwrap();
    n
}
