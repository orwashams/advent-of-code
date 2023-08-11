use std::fs::File;
use std::io::{self, BufRead};

pub fn solve() {
    let file = File::open("input.txt").expect("Failed to open input file");
    let reader = io::BufReader::new(file);
    let mut assignment_pairs = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let ranges: Vec<i32> = line
                .split(',')
                .flat_map(|range| range.split('-').map(|n| n.parse::<i32>().unwrap()))
                .collect();

            if ranges.len() == 4 {
                assignment_pairs.push((ranges[0], ranges[1], ranges[2], ranges[3]));
            } else {
                println!("Invalid input format: {}", line);
            }
        }
    }

    let fully_contained_pairs = count_fully_contained_pairs(assignment_pairs);
    println!(
        "Number of assignment pairs with one range fully containing the other: {}",
        fully_contained_pairs
    );
}

fn is_fully_contained(range1: (i32, i32), range2: (i32, i32)) -> bool {
    // Check if range1 is fully contained within range2
    range1.0 >= range2.0 && range1.1 <= range2.1
}

fn is_overlapping(range1: (i32, i32), range2: (i32, i32)) -> bool {
    // Check if th two ranges overlap
    range1.0 <= range2.1 && range1.1 >= range2.0
}

fn count_fully_contained_pairs(assignment_pairs: Vec<(i32, i32, i32, i32)>) -> usize {
    let mut count = 0;
    for pair in assignment_pairs {
        let (r1_start, r1_end, r2_start, r2_end) = pair;

        if is_overlapping((r1_start, r1_end), (r2_start, r2_end))
            || is_overlapping((r2_start, r2_end), (r1_start, r1_end))
        {
            count += 1;
        }
    }
    count
}
