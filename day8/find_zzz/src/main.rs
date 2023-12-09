use std::{collections::HashMap, fs};

fn parse_input() -> (HashMap<String, Vec<String>>, Vec<i32>) {
    let mut lines = fs::read_to_string(String::from("../medium_input.txt"))
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

    lines.iter().for_each(|s| {
        let vec = s
            .split(['=', '(', ')', ' ', ','])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        map.insert(
            vec[0].to_string(),
            vec![vec[1].to_string(), vec[2].to_string()],
        );
    });

    (map, directions)
}

fn main() {
    let (map, directions) = parse_input();

    let mut val = &String::from("AAA");
    let mut i = 0;
    let mut x: usize = 0;
    while val != "ZZZ" {
        if x == directions.len() {
            x = 0;
        }
        let side = directions[x] as usize;
        val = &map.get(val).unwrap()[side];
        println!("{val}");
        x += 1;
        i += 1;
    }

    println!("STEPS TO ZZZ {i}");
}
