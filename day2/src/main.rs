fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let total: i32 = part_2(&input);

    println!("total: {total}");
}

const R_MAX: i32 = 12;
const G_MAX: i32 = 13;
const B_MAX: i32 = 14;

#[derive(Debug)]
struct Game {
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}
impl Game {
    fn new(subsets: Vec<&str>) -> Self {
        let mut game = Game {
            red: Vec::new(),
            green: Vec::new(),
            blue: Vec::new(),
        };

        for subset in subsets {
            let records: Vec<&str> = subset.split(", ").collect();
            records.iter().for_each(|r| {
                let opt: Vec<&str> = r.split(" ").collect();
                let n: i32 = opt[0].parse().unwrap();

                match opt[1] {
                    "blue" => game.blue.push(n),
                    "red" => game.red.push(n),
                    "green" => game.green.push(n),
                    _ => panic!(),
                };
            });
        }

        game
    }
}

fn part_2(input: &str) -> i32 {
    let mut sum_powers = 0;
    input.lines().for_each(|line| {
        let v: Vec<&str> = line.split(": ").collect();

        // let game_id: i32 = get_id(v[0]);
        let subsets: Vec<&str> = v[1].split("; ").collect();

        let game = Game::new(subsets);
        let min_red = game.red.iter().max().unwrap();
        let min_blue = game.blue.iter().max().unwrap();
        let min_green = game.green.iter().max().unwrap();
        sum_powers += (min_red * min_blue * min_green);
    });
    sum_powers
}

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
