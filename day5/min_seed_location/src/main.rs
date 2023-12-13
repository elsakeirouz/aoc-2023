use std::fs;

fn parse_input() -> Vec<String> {
    fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn get_numbers(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let lines = parse_input();

    let mut seeds: Vec<u64> =
        get_numbers(lines[0].split(':').collect::<Vec<&str>>()[1]);

    let mut ind = 2;

    let mut next_seeds: Vec<u64> = Vec::new();
    while ind < lines.len() {
        let curr_line = lines[ind].clone();
        if !curr_line.chars().any(|c| c.is_numeric()) {
            ind += 1;
            seeds.extend_from_slice(&next_seeds);
            next_seeds.clear();
            continue;
        }
        let vals = get_numbers(&curr_line);
        let dst = vals[0];
        let src = vals[1];
        let range = vals[2];
        let mut i = 0;
        while i < seeds.len() {
            let seed = seeds[i];
            if seed >= src && seed < src + range {
                next_seeds.push(dst + seed - src);
                seeds.remove(i);
            } else {
                i += 1;
            }
        }
        ind += 1;
    }
    seeds.extend_from_slice(&next_seeds);

    println!("{}", seeds.iter().min().unwrap());
}
