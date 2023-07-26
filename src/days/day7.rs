use std::collections::{HashMap, HashSet};
use std::fs;

struct Dir<'a> {
    name: &'a str,
    size: u32,
    nodes: Vec<Node<'a>>,
}

struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum Command {
    Cd,
    Ls,
}

enum Node<'a> {
    Dir(Dir<'a>),
    File(File),
}

pub fn solve() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut dir_stack = vec!["/"];
    let mut current_dir = Node::Dir(Dir {
        name: "/",
        size: 0,
        nodes: vec![],
    });

    for line in input.lines() {
        let line = line.trim();

        if !line.starts_with("$") {
            continue;
        }

        line.split("$").skip(1).for_each(|line| {
            let line = line.trim();
            let (command, arg) = parse_command(line);
            match command {
                Command::Cd => match arg {
                    "/" => {
                        current_dir = Node::Dir(Dir {
                            name: "/",
                            size: 0,
                            nodes: vec![],
                        })
                    }
                    ".." => {
                        dir_stack.pop();
                    }
                    _ => {
                        dir_stack.push(arg);
                    }
                },
                Command::Ls => {}
            }
        });
    }
}

fn parse_command(cmd: &str) -> (Command, &str) {
    let mut parts = cmd.split(" ");
    let cmd = parts.next().unwrap();
    let arg = parts.next().unwrap_or_else(|| "");

    match cmd {
        "cd" => (Command::Cd, arg.into()),
        "ls" => (Command::Ls, "".into()),
        _ => panic!("Unknown command"),
    }
}
