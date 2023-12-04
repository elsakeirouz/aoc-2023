use std::fs;

fn main() {
    let cards: Vec<String> = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut wins = vec![1; cards.len()];
    let mut total = 0;

    for card in cards {
        let parts: Vec<&str> = card.split([':', '|']).collect();
        let game_number = parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap() as usize;
        let winning: Vec<u32> = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let elfs: Vec<u32> = parts[2]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let intersection = winning
            .iter()
            .filter(|x| elfs.contains(x))
            .cloned()
            .collect::<Vec<u32>>()
            .len() as usize;
        let mut i: usize = 0;
        let mut index = 0;
        while index < wins[game_number - 1] {
            while i < intersection {
                wins[game_number + i] += 1;
                i += 1;
            }
            i = 0;
            index += 1;
        }
    }
    total = wins.iter().sum();
    println!("TOTAL SUM {total}");
}
