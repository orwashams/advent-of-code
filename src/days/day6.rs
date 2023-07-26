use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn solve() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let window_size = 14;

    for (index, window) in input_text
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        let set = window.iter().collect::<HashSet<_>>();
        if set.len() == window_size {
            println!("Part 1: {}", index + window_size);
            break;
        }
    }
}
