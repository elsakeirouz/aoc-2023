use std::fs;

use regex::Regex;

fn main() {
    let lines: Vec<String> = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    // Look at where the integers are, look at the right and left chars,
    // look at all the chars
    // below in the same range of indexes.
    let re = Regex::new(r"\d+").unwrap();

    let mut indexes = Vec::new();

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        for num in re.find_iter(line) {
            let start = num.start();
            let end = num.end();
            let num = line.get(start..end).unwrap().parse::<i32>().unwrap();
            indexes.push((i, start, end, line.len(), num));
        }
    }

    for (line_num, start, end, len, num) in &indexes {
        let line = &lines[*line_num].chars().collect::<Vec<char>>();

        if start > &0 && is_special_char(line[*start - 1]) {
            sum += num;
            continue;
        }

        if end < &(len - 1) && is_special_char(line[*end]) {
            sum += num;
            continue;
        }

        if line_num > &0 {
            let upper = &lines[*line_num - 1];
            if *end < upper.len() && start < end {
                let begin = if start == &0 { 0 } else { *start - 1 };
                let finish = if *end > upper.len() {
                    upper.len() - 1
                } else {
                    *end + 1
                };
                let substr = upper.get(begin..finish).unwrap();
                if substr.chars().any(|c| is_special_char(c)) {
                    sum += num;
                    continue;
                }
            }
        }

        if line_num < &(lines.len() - 1) {
            let lower = &lines[*line_num + 1];
            let len = lower.len();
            if *end >= len && start >= end {
                continue;
            }
            let begin = if start == &0 { 0 } else { *start - 1 };
            let finish = if *end + 1 > len { len - 1 } else { *end + 1 };
            let substr = lower.get(begin..finish).unwrap();
            if substr.chars().any(|c| is_special_char(c)) {
                sum += num;
                continue;
            }
        }
    }

    println!("SUM IS {sum}");
}

fn is_special_char(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}
