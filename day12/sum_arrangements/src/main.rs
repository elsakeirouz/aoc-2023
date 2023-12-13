use std::fs;

fn process_p1(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut records: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|v| {
            records.push(v[0].chars().collect::<Vec<char>>());
            groups.push(
                v[1].split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        });

    (records, groups)
}

fn process_p2(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut records: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|v| {
            let string = v[0].chars().collect::<Vec<char>>();
            let nums = v[1]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut char_vec = Vec::new();
            let mut num_vec = Vec::new();
            for i in 0..5 {
                num_vec.extend_from_slice(&nums[..]);
                char_vec.extend_from_slice(&string[..]);
                if i != 4 {
                    char_vec.push('?');
                }
            }
            records.push(char_vec);
            groups.push(num_vec);
        });

    (records, groups)
}

fn parse_input() -> String {
    fs::read_to_string(String::from("../mini_input.txt")).expect("Could not read file")
}

fn arrange(rec: &Vec<char>, groups: &Vec<usize>) -> usize {
    let rec_empty = rec.is_empty();
    let groups_empty = groups.is_empty();
    let contains = rec.contains(&'#');
    if groups_empty && (rec_empty || !contains) {
        return 1;
    }
    if rec_empty || (groups_empty && contains) {
        return 0;
    }
    let first = rec[0];
    if first == '.' {
        arrange(&rec[1..].to_vec(), groups)
    } else if first == '?' {
        let mut copy = rec.clone();
        copy[0] = '.';
        let if_dot = arrange(&copy, groups);
        copy[0] = '#';
        return arrange(&copy, groups) + if_dot;
    } else {
        let count = groups[0];
        let mut i = 0;
        if count > rec.len() {
            return arrange(&rec[1..].to_vec(), groups);
        }
        while i < count && rec[i] != '.' {
            i += 1;
        }
        if i < rec.len() {
            if i < count || rec[i] == '#' {
                return 0;
            }
            return arrange(&rec[i + 1..].to_vec(), &groups[1..].to_vec());
        }
        arrange(&rec[i..].to_vec(), &groups[1..].to_vec())
    }
}

fn calculate(part1: bool, input: &str) {
    let mut sum = 0;
    let mut i = 0;
    let (records, groups) = if part1 {
        process_p1(input)
    } else {
        process_p2(input)
    };
    records.iter().for_each(|rec| {
        let arr = arrange(rec, &groups[i]);
        sum += arr;
        i += 1;
    });
    println!("PART {} : {sum}", if part1 { "1" } else { "2" });
}

fn main() {
    let input = parse_input();

    calculate(true, &input);
    calculate(false, &input);
}
