use std::fs;

fn main() {
    // Prompting user for filename

    let mut filename = String::from("input.txt");
    println!("FILENAME {filename}");

    let lines = lines_from_file(&mut filename);
    let mut sum: i32 = 0;
    for line in lines {
        let digits: Vec<u32> =
            line.chars().filter_map(|a| a.to_digit(10)).collect();
        let mut num = String::new();
        if digits.len() < 2 {
            num = digits[0].to_string();
            num += &digits[0].to_string();
        } else {
            num = digits[0].to_string();
            num += &digits[digits.len() - 1].to_string();
        }
        sum += num.parse::<i32>().unwrap();
    }
    println!("The sum of all calibration values is {sum}")
}

fn lines_from_file(filename: &String) -> Vec<String> {
    let contents =
        fs::read_to_string(filename).expect("Could not read the file");
    contents.lines().map(|s| s.to_string()).collect()
}
