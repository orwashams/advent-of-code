pub fn solve() {
    use std::fs;

    let mut from_a_to_z = ('a'..='z').collect::<Vec<char>>();
    let mut from_a_to_z_capital = ('A'..='Z').collect::<Vec<char>>();

    from_a_to_z.append(&mut from_a_to_z_capital);

    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;

    for group in input.lines().collect::<Vec<&str>>().chunks(3) {
        let first = group[0];

        for c in first.chars() {
            if group[1].contains(c) && group[2].contains(c) {
                result += from_a_to_z.iter().position(|&x| x == c).unwrap() + 1;
                break;
            }
        }
    }

    println!("{}", result);
}
