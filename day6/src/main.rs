fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let value: u64 = _part_2(&input);

    println!("total: {}", value);
}

fn _part_2(_input: &str) -> u64 {
    let lines: Vec<_> = _input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            iter.next();
            let time: String = iter.collect();
            let time_u: u64 = time.parse().unwrap();
            time_u
        })
        .take(2)
        .collect();
    let race = Race {
        time: lines[0],
        record: lines[1],
    };
    println!("{:?}", race);

    let w = race.winning_times();
    println!("{:?}", w);
    let w1: u64 = w.0.floor() as u64;
    let w2 = w.1.ceil() as u64;
    println!("{} {}", w1, w2);
    let result = (w1 - w2) + 1;

    println!("{:?}", result);

    result
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}
impl Race {
    fn winning_times(&self) -> (f64, f64) {
        // th^2 - TT*th + record = 0
        // x1 = -TT + sqrt(TT^2 - 4*record) / 2
        // x2 = -TT - sqrt(TT^2 - 4*record) / 2
        let tt = self.time as f64 * -1.;
        let record = self.record as f64 + 1.;
        let x1 = (-tt + (tt * tt - 4. * record).sqrt()) / 2.;
        let x2 = (-tt - (tt * tt - 4. * record).sqrt()) / 2.;
        (x1, x2)
    }
}
fn _part_1(_input: &str) -> u64 {
    let lines: Vec<_> = _input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            iter.next();
            let row_numbers: Vec<u64> = iter.map(|a| a.parse().unwrap()).collect();
            // println!("{row_numbers:?}");
            row_numbers
        })
        .take(2)
        .collect();
    let races: Vec<Race> = lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(a, b)| Race {
            time: *a,
            record: *b,
        })
        .collect();
    println!("{:?}", races);

    let result: u64 = races
        .iter()
        .map(|race| {
            let w = race.winning_times();
            println!("{:?}", w);
            let w1: u64 = w.0.floor() as u64;
            let w2 = w.1.ceil() as u64;
            println!("{} {}", w1, w2);
            (w1 - w2) + 1
        })
        .product();

    println!("{:?}", result);

    result
}
