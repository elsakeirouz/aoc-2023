use std::fs;

fn main() {
    let cards: Vec<String> = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut worth = 0;

    for card in cards {
        let parts: Vec<&str> = card.split([':', '|']).collect();
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
            .len() as u32;
        worth += if intersection > 0 {
            i32::pow(2, intersection - 1)
        } else {
            0
        };
    }
    println!("CARDS WORTH {worth}");
}
