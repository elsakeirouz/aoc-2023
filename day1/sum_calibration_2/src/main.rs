use std::{
    fs,
};


fn main() {
    // Prompting user for filename

    let mut filename = String::from("input.txt");

    let lines = lines_from_file(&mut filename);
    let mut sum : u32 = 0;
    for line in lines{
        let res = line.chars().filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        sum += 10 * res.first().unwrap() + res.last().unwrap();
    }

    println!("{sum}")
}

fn lines_from_file(filename : &String) ->  Vec<String>{
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let replaced = contents.replace("one","o1e")
        .replace("two","t2o")
        .replace("three","t3e")
        .replace("four","4")
        .replace("five","5e")
        .replace("six","6")
        .replace("seven","7n")
        .replace("eight","e8t")
        .replace("nine","n9e");
    replaced.lines().map(|s| s.to_string()).collect()
}
