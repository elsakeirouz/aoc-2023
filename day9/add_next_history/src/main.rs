use std::fs;

fn parse_input() -> Vec<Vec<i64>> {
    fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn next_vector(history: &Vec<i64>) -> Vec<i64> {
    let new = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect::<Vec<i64>>();
    new
}

fn next_value(history: &Vec<i64>) -> i64 {
    if history.iter().all(|x| x == &0) {
        0
    } else {
        *history.last().unwrap() + next_value(&next_vector(history))
    }
}

fn prev_value(history: &Vec<i64>) -> i64 {
    if history.iter().all(|x| x == &0) {
        0
    } else {
        *history.first().unwrap() - prev_value(&next_vector(history))
    }
}

fn main() {
    let histories = parse_input();
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    histories.iter().for_each(|history| {
        sum_1 += next_value(history);
        sum_2 += prev_value(history);
    });

    println!("PART 1 {sum_1}\nPART 2 {sum_2}");
}
