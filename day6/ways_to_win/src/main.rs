use std::fs;

fn extract_ints(line: &str) -> Vec<i64> {
    line.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn extract_int_2(line: &str) -> i64 {
    line.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap()
}

fn parse_input(part1: bool) -> (Vec<i64>, Vec<i64>) {
    let lines: Vec<String> = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    if part1 {
        (extract_ints(&lines[0]), extract_ints(&lines[1]))
    } else {
        (
            vec![extract_int_2(&lines[0])],
            vec![extract_int_2(&lines[1])],
        )
    }
}

fn multiply_n_options(part1: bool) {
    let (times, dists) = parse_input(part1);

    let len = times.len();
    let mut options = vec![0; len];
    let mut i = 0;

    while i < len {
        let dur = times[i];
        let mut hold = 1;
        while hold < dur {
            if hold * (dur - hold) > dists[i] {
                options[i] += 1;
            }
            hold += 1;
        }
        i += 1;
    }

    println!(
        "RESULT FOR PART {} = {}",
        if part1 { "1" } else { "2" },
        options.iter().fold(1, |a, b| a * b)
    )
}

fn main() {
    multiply_n_options(true);
    multiply_n_options(false);
}
