use std::{
    cell::RefCell,
    collections::VecDeque,
    io::BufRead,
    rc::{Rc, Weak},
};

type MutDirectory = RefCell<Directory>;

#[derive(Default, Debug)]
struct Directory {
    size: u64,
    name: String,
    children: VecDeque<Rc<MutDirectory>>,
    parent: Option<Weak<MutDirectory>>,
}

impl Directory {
    pub fn to_mut_rc(self) -> Rc<RefCell<Directory>> {
        Rc::new(RefCell::new(self))
    }
}

enum Command {
    Cd(String),
    Ls,
}

fn execute_cd(dir: String, dir_stack: &mut VecDeque<Rc<MutDirectory>>) {
    println!("{}", dir);
    if dir == ".." {
        dir_stack.pop_back();
        return;
    }

    if dir == "/" && dir_stack.len() >= 1 {
        dir_stack.truncate(1);
        return;
    }

    let parent_opt = dir_stack.pop_back();
    let dir_struct = Directory {
        name: dir,
        size: 0,
        parent: None,
        children: VecDeque::new(),
    };

    if let Some(parent) = parent_opt {
        let dir_struct = Directory {
            parent: Some(Rc::downgrade(&parent)),
            ..dir_struct
        };
        let rc_dir_struct = dir_struct.to_mut_rc();
        parent
            .borrow_mut()
            .children
            .push_back(rc_dir_struct.clone());
        dir_stack.push_back(parent);
        dir_stack.push_back(rc_dir_struct.clone());
    } else {
        let rc_dir_struct = dir_struct.to_mut_rc();
        dir_stack.push_back(rc_dir_struct.clone());
    }
}

fn update_dir_size(size: u64, dir_stack: &mut VecDeque<Rc<MutDirectory>>) {
    let popped_dir = dir_stack.pop_back().unwrap();
    let mut dir = popped_dir.borrow_mut();
    dir.size += size;
    if let Some(parent) = &dir.parent {
        let mut queue = Vec::new();
        queue.push(parent.upgrade().unwrap().clone());
        while let Some(current) = queue.pop() {
            let mut mut_curr = current.borrow_mut();
            mut_curr.size += size;

            let parent_opt = &mut_curr.parent;
            if let Some(parent) = parent_opt {
                queue.push(parent.upgrade().unwrap().clone());
            }
        }
    }
    dir_stack.push_back(popped_dir.clone());
}

fn sum_less_than_100k(
    dir_stack: &mut VecDeque<Rc<MutDirectory>>,
    limit: u64,
    disk_size: u64,
) -> (u64, u64) {
    let root = dir_stack.pop_front().unwrap();

    let mut candidate_size = root.borrow_mut().size;
    let required_space = 30 * 10_u64.pow(6) - (disk_size - candidate_size);
    let mut score: u64 = 0;

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(current) = queue.pop_back() {
        let mut current = current.borrow_mut();
        if current.size <= limit {
            score += current.size;
        }
        if current.size <= candidate_size && current.size >= required_space {
            candidate_size = current.size;
        }

        queue.append(&mut current.children);
    }

    return (score, candidate_size);
}

fn parse_cmd(line: &str) -> Command {
    let mut splitted = line.split(" ");
    // remove $
    splitted.next();

    match splitted.next() {
        Some("cd") => Command::Cd(splitted.next().unwrap().to_owned()),
        Some("ls") => Command::Ls,
        Some(_) => panic!("Waah!"),
        None => panic!("Waaah!"),
    }
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();
    let mut dir_stack = VecDeque::new();
    let mut count = 0;

    while let Some(Ok(line)) = lines.next() {
        if let Some('$') = line.chars().next() {
            let cmd = parse_cmd(&line);

            match cmd {
                Command::Cd(dir) => execute_cd(dir, &mut dir_stack),
                Command::Ls => continue,
            }
        } else {
            let mut splitted = line.split(" ");
            match splitted.next() {
                Some(number) if number.parse::<u64>().is_ok() => {
                    count += 1;
                    update_dir_size(number.parse().unwrap(), &mut dir_stack)
                }
                Some(_) => continue,
                None => panic!("Waah!"),
            }
        }
    }
    println!("{}", count);

    println!(
        "{:?}",
        sum_less_than_100k(&mut dir_stack, 100000, 70 * 10_u64.pow(6))
    );
}
