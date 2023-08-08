use std::{
    borrow::Borrow,
    cell::RefCell,
    fs,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    name: String,
    size: RefCell<u64>,
    children: RefCell<Vec<Rc<Node>>>,
    parent: Option<Weak<Node>>,
}
impl Node {
    fn new(name: String, size: RefCell<u64>, parent: Option<Weak<Node>>) -> Self {
        Self {
            name,
            size,
            children: RefCell::new(Vec::new()),
            parent,
        }
    }
}

#[derive(Debug)]
struct Tree {
    root: Rc<Node>,
}
impl Tree {
    fn new() -> Self {
        Self {
            root: Rc::new(Node {
                name: String::from("/"),
                size: RefCell::new(0),
                children: RefCell::new(Vec::new()),
                parent: None,
            }),
        }
    }
    fn find_node(&self, name: &str) -> Option<Rc<Node>> {
        self._find_node(&self.root, name)
    }

    fn _find_node(&self, node: &Rc<Node>, name: &str) -> Option<Rc<Node>> {
        if node.name == name {
            return Some(Rc::clone(node));
        }

        let children = node.children.borrow();
        for child in children.iter() {
            if let Some(found) = self._find_node(child, name) {
                return Some(found);
            }
        }

        None
    }
    fn get_all_sizes(&self) -> Vec<u64> {
        self._get_all_sizes(&self.root)
    }
    fn _get_all_sizes(&self, node: &Rc<Node>) -> Vec<u64> {
        if node.children.borrow().is_empty() {
            return vec![*node.size.borrow()];
        }
        let children = node.children.borrow();
        let mut sizes = Vec::new();
        for child in children.iter() {
            sizes.append(&mut self._get_all_sizes(child));
        }
        sizes
    }
}

pub fn solve() {
    let input = fs::read_to_string("input.txt").unwrap();
    let tree = Tree::new();
    let mut current_dir = Rc::clone(&tree.root);

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.split("$ cd").nth(1).unwrap();
            let dir = dir.trim();
            if dir == ".." {
                current_dir = Rc::clone(&current_dir.parent.clone().unwrap().upgrade().unwrap());
                continue;
            }
            if dir == "/" {
                current_dir = Rc::clone(&tree.root);
                continue;
            }
            if dir != "/" {
                let child = Rc::new(Node::new(
                    dir.to_string(),
                    RefCell::new(0),
                    Some(Rc::downgrade(&current_dir)),
                ));
                current_dir.children.borrow_mut().push(Rc::clone(&child));
                current_dir = Rc::clone(&child);
                continue;
            }
        }
        if line.starts_with("$ ls") {
            continue;
        }
        let first_word = line.split(" ").nth(0).unwrap().trim();
        if first_word == "dir" {
            continue;
        }
        let size = first_word.parse::<u64>().unwrap();
        *current_dir.size.borrow_mut() += size;
    }
    let all_sizes_vec = tree.get_all_sizes();
    println!("vec sum{:?}", all_sizes_vec.iter().sum::<u64>());
    println!("root size{:?}", tree.root.size.borrow());

    println!(
        "{:?}",
        all_sizes_vec.iter().filter(|x| *x <= &100000).sum::<u64>()
    );
}
