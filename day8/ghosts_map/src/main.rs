use std::{collections::HashMap, fs};

fn parse_input() -> (HashMap<String, Vec<String>>, Vec<i32>, Vec<String>) {
    let mut lines = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let directions = lines
        .remove(0)
        .chars()
        .map(|c| if c == 'R' { 1 } else { 0 })
        .collect::<Vec<i32>>();

    lines.remove(0);

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let mut start_nodes: Vec<String> = Vec::new();

    lines.iter().for_each(|s| {
        let vec = s
            .split(['=', '(', ')', ' ', ','])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if vec[0].ends_with('A') {
            start_nodes.push(vec[0].to_string());
        }
        map.insert(
            vec[0].to_string(),
            vec![vec[1].to_string(), vec[2].to_string()],
        );
    });

    (map, directions, start_nodes)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn main() {
    let (map, directions, mut curr) = parse_input();

    let mut steps = vec![0; curr.len()];
    let mut x: usize = 0;
    for (ind, val) in &mut curr.iter_mut().enumerate() {
        while !val.ends_with('Z') {
            if x == directions.len() {
                x = 0;
            }
            steps[ind] += 1;
            let side = directions[x] as usize;
            *val = map.get(val).unwrap()[side].clone();
            x += 1;
        }
    }

    let res = steps.iter().cloned().fold(1, |a, b| lcm(a, b));
    println!("STEPS TO ZZZ {:?}", res);
}
