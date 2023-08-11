use std::{
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
    fn get_all_sizes(&self, sizes: &mut Vec<u64>) {
        sizes.push(*self.size.borrow());

        for child in &*self.children.borrow() {
            child.get_all_sizes(sizes);
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
    fn get_all_sizes(&self) -> Vec<u64> {
        let mut sizes = Vec::new();
        self.root.get_all_sizes(&mut sizes);
        sizes
    }

    fn sync_sizes(&self) {
        self._sync_sizes(&self.root);
    }
    fn _sync_sizes(&self, node: &Rc<Node>) {
        if node.children.borrow().is_empty() {
            return;
        }
        let children = node.children.borrow();
        for child in children.iter() {
            self._sync_sizes(child);
            *node.size.borrow_mut() += child.size.borrow().clone();
        }
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
            let child = Rc::new(Node::new(
                dir.to_string(),
                RefCell::new(0),
                Some(Rc::downgrade(&current_dir)),
            ));
            current_dir.children.borrow_mut().push(Rc::clone(&child));
            current_dir = Rc::clone(&child);
            continue;
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
    tree.sync_sizes();

    let all_sizes_vec = tree.get_all_sizes();

    println!(
        "\n\nYour answer is: {:?}\n\n",
        all_sizes_vec.iter().filter(|x| *x <= &100000).sum::<u64>()
    );
}
