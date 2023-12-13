use std::{cmp::Ordering, collections::HashMap, fs};

fn parse_input() -> HashMap<String, i64> {
    let lines = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let vec = lines
        .iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut hands = HashMap::new();

    for line in &vec {
        hands.insert(line[0].to_string(), line[1].parse::<i64>().unwrap());
    }

    hands
}

fn count_chars(hand: &str) -> Vec<i64> {
    let mut char_count: HashMap<char, i64> = HashMap::new();

    for c in hand.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    char_count.values().cloned().collect::<Vec<i64>>()
}

fn count_chars_2(hand: &str) -> Vec<i64> {
    let mut char_count: HashMap<char, i64> = HashMap::new();

    let mut max_key: char = ' ';
    let mut max_val = std::i64::MIN;
    for c in hand.chars() {
        *char_count.entry(c).or_insert(0) += 1;
        if c != 'J' && &max_val < char_count.get(&c).unwrap() {
            max_val = *char_count.get(&c).unwrap();
            max_key = c;
        }
    }

    if char_count.contains_key(&'J') {
        *char_count.entry(max_key).or_insert(0) += char_count[&'J'];
        char_count.remove(&'J');
    }

    char_count.values().cloned().collect::<Vec<i64>>()
}

fn type_of_hand(char_count: Vec<i64>) -> usize {
    if char_count.contains(&5) {
        6
    } else if char_count.contains(&4) {
        5
    } else if char_count.contains(&3) {
        if char_count.contains(&2) {
            4
        } else {
            3
        }
    } else if char_count.iter().filter(|&&x| x == 2).count() == 2 {
        2
    } else if char_count.contains(&2) {
        1
    } else {
        0
    }
}

// returns true if a is winning hand
fn winning_hand(a: &str, b: &str, part1: bool) -> Ordering {
    // for part 1
    let mut cards = String::from("23456789TJQKA");
    if !part1 {
        // for part 2
        cards = String::from("J23456789TQKA");
    }

    let mut i = 0;
    let a_ch = a.chars().collect::<Vec<char>>();
    let b_ch = b.chars().collect::<Vec<char>>();

    while i < a_ch.len() && a_ch[i] == b_ch[i] {
        i += 1;
    }
    let c1 = a_ch[i];
    let c2 = b_ch[i];
    cards.find(c1).unwrap().cmp(&cards.find(c2).unwrap())
}

fn sort_hands(hands: &HashMap<String, i64>, part1: bool) -> Vec<Vec<String>> {
    let mut sorted_hands: Vec<Vec<String>> = Vec::with_capacity(7);
    for _ in 0..7 {
        sorted_hands.push(Vec::new());
    }

    let mut char_count = Vec::new();
    for hand in hands.keys() {
        if part1 {
            char_count = count_chars(&hand);
        } else {
            char_count = count_chars_2(&hand);
        }
        let hand_type = type_of_hand(char_count);
        sorted_hands[hand_type].push(hand.to_string());
    }

    for vec in &mut sorted_hands {
        vec.sort_by(|a, b| winning_hand(a, b, part1));
    }

    sorted_hands
}

fn find_res(
    hands: &HashMap<String, i64>,
    sorted_hands: &Vec<Vec<String>>,
) -> i64 {
    let ordered_winnings = sorted_hands.into_iter().flatten().map(|x| hands[x]);

    let mut i = 0;
    ordered_winnings
        .map(|x| {
            i += 1;
            x * i
        })
        .sum()
}

fn run(hands: &HashMap<String, i64>, part1: bool) {
    let sorted_hands = sort_hands(&hands, part1);

    let res = find_res(&hands, &sorted_hands);
    println!("PART {} {res}", if part1 { "1" } else { "2" });
}

fn main() {
    let hands = parse_input();

    // Part 1 -> true
    run(&hands, true);

    // Part 2 -> false
    run(&hands, false);
}
