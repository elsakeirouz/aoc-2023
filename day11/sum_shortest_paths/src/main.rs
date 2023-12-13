use std::{char, fs};

fn get_final_pos(
    a: usize,
    b: usize,
    horiz: &Vec<usize>,
    vert: &Vec<usize>,
    coef: usize,
) -> (usize, usize) {
    let mut x_add = 0;
    let mut y_add = 0;

    for val in vert {
        if a < (*val as usize) {
            break;
        }
        x_add += 1;
    }

    for val in horiz {
        if b < (*val as usize) {
            break;
        }
        y_add += 1;
    }

    let res = (a + x_add * (coef - 1), b + y_add * (coef - 1));
    res
}

fn parse_input(part1: bool) -> Vec<(usize, usize)> {
    let mut input = fs::read_to_string(String::from("../input.txt"))
        .expect("Could not read file")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut i = 0;
    let mut horizontal_indexes = Vec::new();
    while i < input.len() {
        let vec = &input[i];
        if vec.iter().all(|&c| c == '.') {
            horizontal_indexes.push(i);
        }
        i += 1;
    }

    let mut indexes = Vec::new();
    let mut j = 0;
    i = 0;
    let vec = &input[i];
    while j < vec.len() {
        while i < input.len() && input[i][j] == '.' {
            i += 1;
        }
        if i == input.len() {
            indexes.push(j);
        }
        j += 1;
        i = 0;
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let coef = if part1 { 2 } else { 1000000 };
    for (i, vec) in &mut input.iter_mut().enumerate() {
        for (j, &c) in vec.iter().enumerate() {
            if c == '#' {
                galaxies.push(get_final_pos(
                    j,
                    i,
                    &horizontal_indexes,
                    &indexes,
                    coef,
                ));
            }
        }
    }

    galaxies
}

fn shortest_path(x1: usize, x2: usize, y1: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn sum_paths(galaxies: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for i in 0..galaxies.len() - 1 {
        let a = galaxies[i];
        for b in galaxies.iter().skip(i + 1) {
            let len = shortest_path(a.0, b.0, a.1, b.1);
            sum += len;
        }
    }

    sum
}

fn main() {
    println!("PART 1 -> {}", sum_paths(&parse_input(true)));
    println!("PART 2 -> {}", sum_paths(&parse_input(false)));
}
