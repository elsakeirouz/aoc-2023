use std::{collections::HashMap, fs};

fn main() {
    let games: Vec<String> = fs::read_to_string(String::from("input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    // 12 red, 13 green, 14 blue
    let mut colors_map = HashMap::new();
    colors_map.insert("red".to_string(), 12);
    colors_map.insert("green".to_string(), 13);
    colors_map.insert("blue".to_string(), 14);
    let mut sum: i32 = 0;
    for game in games {
        let parts: Vec<&str> = game.split(":").collect();
        let game_number = parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let game_sets: Vec<&str> = parts[1]
            .split(";")
            .map(|s| s.split(',').collect::<Vec<&str>>())
            .flatten()
            .collect();
        let mut impossible = false;
        for set in game_sets {
            let ball_data = set.split_whitespace().collect::<Vec<&str>>();
            let ball_number = ball_data[0].parse::<i32>().unwrap();
            if ball_number > colors_map[ball_data[1]] {
                impossible = true;
                continue;
            }
        }
        if !impossible {
            sum += game_number;
        }
    }
    println!("Total sum of game IDs is {}", sum);
}
