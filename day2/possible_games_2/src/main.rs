use std::{collections::HashMap, fs};

fn main() {
    let games: Vec<String> = fs::read_to_string(String::from("../possible_games/input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    // 12 red, 13 green, 14 blue
    let mut colors_map = HashMap::from([
        ("red".to_string(), 0),
        ("green".to_string(), 0),
        ("blue".to_string(), 0),
    ]);
    let mut sum: i32 = 0;
    for game in games {
        let game_sets: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .map(|s| s.split(',').collect::<Vec<&str>>())
            .flatten()
            .collect();
        for set in game_sets {
            let ball_data = set.split_whitespace().collect::<Vec<&str>>();
            let ball_number = ball_data[0].parse::<i32>().unwrap();
            let ball_color = ball_data[1];
            if ball_number > colors_map[ball_color] {
                colors_map.insert(String::from(ball_color), ball_number);
                continue;
            }
        }
        sum += colors_map.values().product::<i32>();
        colors_map = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
    }
    println!("Total sum of multiplications is {}", sum);
}
