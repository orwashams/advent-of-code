pub fn solve() {
    use std::fs;

    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;

    for line in input.lines() {
        result += match line.trim() {
            "A X" => Result::Lose as u32 + RPS::Scissors as u32,
            "A Y" => Result::Draw as u32 + RPS::Rock as u32,
            "A Z" => Result::Win as u32 + RPS::Paper as u32,
            "B X" => Result::Lose as u32 + RPS::Rock as u32,
            "B Y" => Result::Draw as u32 + RPS::Paper as u32,
            "B Z" => Result::Win as u32 + RPS::Scissors as u32,
            "C X" => Result::Lose as u32 + RPS::Paper as u32,
            "C Y" => Result::Draw as u32 + RPS::Scissors as u32,
            "C Z" => Result::Win as u32 + RPS::Rock as u32,
            _ => 0,
        };
    }
    println!("{}", result);
}

enum XYZ {
    X = 0, //You Must Lost
    Y = 3, //You Must Draw
    Z = 6, //You Must Win
}

//A = Rock
//B = Paper
//C = Scissors
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
