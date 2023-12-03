use std::fs;

use regex::Regex;

fn main() {
    let lines: Vec<String> = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let re = Regex::new(r"\*").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();

    let mut star_indexes = Vec::new();
    let mut num_indexes = Vec::new();

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        for star in re.find_iter(line) {
            let start = star.start();
            star_indexes.push((i as i32, start as i32));
        }
        for num in re_num.find_iter(line) {
            let start = num.start();
            let end = num.end();
            let num = line.get(start..end).unwrap().parse::<i32>().unwrap();
            num_indexes.push((i as i32, start as i32, end as i32, num));
        }
    }

    for (line, index) in &star_indexes {
        let mut gear_ratio = Vec::new();
        for (num_line, start, end, num) in &num_indexes {
            if num_line <= &(line + 1) && num_line >= &(line - 1) {
                if index >= &(start - 1) && index <= &end {
                    gear_ratio.push(num);
                }
            }
        }
        if gear_ratio.len() == 2 {
            sum += gear_ratio.iter().fold(1, |num, &x| num * x);
        }
    }

    println!("SUM IS {sum}");
}
