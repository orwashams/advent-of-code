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
    let mut cwd = "/";
    let mut listed_dirs: Vec<&str> = vec![];

    for line in input.lines() {
        let line = line.trim();

        if !line.starts_with("$") {
            println!("Invalid line: {line}");
            continue;
        }

        line.split("$").skip(1).for_each(|line| {
            let line = line.trim();
            let (command, arg) = parse_command(line);
            match command {
                Command::Cd => match arg {
                    "/" => {
                        cwd = "/";
                        dir_stack.clear();
                        dir_stack.push("/");
                        println!("called cd /, before pop, dir_stack = {:?}", dir_stack);
                    }
                    ".." => {
                        println!("called .., before pop, dir_stack = {:?}", dir_stack);
                        dir_stack.pop();
                        println!("called .., after pop, dir_stack = {:?}", dir_stack);
                    }
                    _ => {
                        println!(
                            "called cd on {arg}, before push, dir_stack = {:?}",
                            dir_stack
                        );
                        dir_stack.push(arg);
                        println!(
                            "called cd on {arg}, after push, dir_stack = {:?}",
                            dir_stack
                        );

                        println!("current_dir before change = {:?}", cwd);
                        cwd = dir_stack.last().unwrap();
                        println!("current_dir after change = {:?}", cwd);
                    }
                },
                Command::Ls => {
                    println!(
                        "bla bla: {:?}",
                        input.lines().take(4).collect::<Vec<&str>>()
                    );
                    println!("lised ls, arg = {:?}", listed_dirs);
                    println!("called ls,current_dir = {:?}", cwd);
                    listed_dirs.clear();
                }
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
