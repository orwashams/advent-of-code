use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    println!("{:?}", grid[0]);

    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();
    println!("width: {}, height: {}", width, height);
}
