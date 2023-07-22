use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn solve() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut map: HashMap<String, VecDeque<String>> = HashMap::new();

    for line in file.lines() {
        let vec = line
            .split("")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        if line.is_empty() {
            break;
        }

        for string in &vec {
            if is_str_number(string) {
                map.entry(string.to_string())
                    .or_insert(VecDeque::from(vec![]));
            }
        }

        let vec = vec[1..=vec.len() - 2].to_vec();

        let trimmed_vec = &vec.into_iter().step_by(4).collect::<Vec<&str>>();

        for (index, cratee) in trimmed_vec.into_iter().enumerate() {
            if is_str_white_space(cratee) {
                continue;
            }
            if is_str_number(cratee) {
                continue;
            }

            let crate_stack = (index + 1).to_string();

            match map.get_mut(&crate_stack) {
                Some(x) => {
                    x.push_front(cratee.to_string());
                }
                None => {
                    map.insert(
                        crate_stack.clone(),
                        VecDeque::from(vec![cratee.to_string()]),
                    );
                }
            }
        }
    }
    println!("Before change:");
    print_pretty_hashmap(&map);
    for line in file.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let (amount, from, to) = parse_command(line);
        for _ in 0..amount {
            println!("{:?} {:?} {:?}", amount, from, to);
            let cratee = map.get_mut(&from.to_string()).unwrap().pop_back().unwrap();
            println!("{:?}", cratee);

            map.get_mut(&to.to_string()).unwrap().push_back(cratee);
        }

        println!("After change:");
        print_pretty_hashmap(&map)
    }
}

fn is_str_number(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

fn is_str_white_space(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

fn parse_command(line: &str) -> (usize, &str, &str) {
    let vec = line.split(" ").collect::<Vec<&str>>();
    (vec[1].parse().unwrap(), vec[3], vec[5])
}

fn print_pretty_hashmap<K, V>(hash_map: &HashMap<K, V>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    for (key, value) in hash_map {
        println!("{:?}: {:?}", key, value);
    }
}
