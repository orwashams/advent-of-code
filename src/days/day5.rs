use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn solve() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

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
                map.entry(string.to_string()).or_insert(vec![]);
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
                    x.push(cratee.to_string());
                }
                None => {
                    map.insert(crate_stack.clone(), vec![cratee.to_string()]);
                }
            }
        }
    }

    for item in map.values_mut() {
        item.reverse();
    }

    println!("Before change:");
    print_pretty_hashmap(&map);
    for line in file.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let (amount, from, to) = parse_command(line);
        println!("amount: {:?} from: {:?} to:{:?}", amount, from, to);

        let cratee = map.get_mut(from).unwrap();

        println!("cratee: {:?}", cratee);

        let mut moved_crates = cratee
            .drain((cratee.len() - amount)..cratee.len())
            .collect::<Vec<String>>();

        println!("moved_crates: {:?}", moved_crates);
        map.get_mut(to).unwrap().append(&mut moved_crates);
    }
    println!("After change:");
    print_pretty_hashmap(&map);

    let mut temp_vec: Vec<String> = vec![];
    for item in map.iter() {
        let strss = format!("{} {}", item.0, item.1[item.1.len() - 1]);

        temp_vec.push(strss);
    }
    temp_vec.sort();
    println!("temp_vec: {:?}", temp_vec);
}

#[allow(dead_code)]
fn is_str_number(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

#[allow(dead_code)]
fn is_str_white_space(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

#[allow(dead_code)]
fn parse_command(line: &str) -> (usize, &str, &str) {
    let vec = line.split(" ").collect::<Vec<&str>>();
    (vec[1].parse().unwrap(), vec[3], vec[5])
}

#[allow(dead_code)]
fn print_pretty_hashmap<K, V>(hash_map: &HashMap<K, V>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    for (key, value) in hash_map {
        println!("{:?}: {:?}", key, value);
    }
}
