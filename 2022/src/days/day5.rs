use std::{
    collections::VecDeque,
    io::{BufRead, Lines},
    ops::IndexMut,
    process::exit,
};
#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_command(line: String) -> Move {
    let mut splitted =
        line.split(" ").filter_map(|str| str.parse::<usize>().ok());

    Move {
        num: splitted.next().unwrap(),
        from: splitted.next().unwrap() - 1,
        to: splitted.next().unwrap() - 1,
    }
}

fn build_cargo_lanes(lines: &mut Lines<impl BufRead>) -> Vec<VecDeque<char>> {
    let mut cargo_9000: Vec<VecDeque<char>> = Vec::new();
    let mut cargo_9001: Vec<VecDeque<char>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break;
        }

        let lanes = line
            .chars()
            .enumerate()
            .filter(|&(i, _)| ((i.wrapping_sub(1)) as isize) % 4 == 0)
            .map(|(_, c)| c);

        if cargo_9000.len() == 0 {
            lanes.clone().for_each(|_| cargo_9000.push(VecDeque::new()));
        }
        if cargo_9001.len() == 0 {
            lanes.clone().for_each(|_| cargo_9001.push(VecDeque::new()));
        }

        lanes
            .enumerate()
            .filter(|&(_, c)| c.is_alphabetic())
            .for_each(|(i, c)| {
                let lane = &mut cargo_9000[i];
                let lane2 = &mut cargo_9001[i];
                lane.push_back(c);
                lane2.push_back(c);
            })
    }

    while let Some(Ok(line)) = lines.next() {
        let command = parse_command(line);
        if command.to == command.from {
            continue;
        }

        crane_execute(&command, &mut cargo_9000, cargo9000_swap);
        crane_execute(&command, &mut cargo_9001, cargo9001_swap)
    }

    println!("Part1: ");
    cargo_9000.iter().for_each(|v| println!("{:?}", v));
    println!("Part2: ");
    cargo_9001.iter().for_each(|v| println!("{:?}", v));
    cargo_9000
}

fn crane_execute<MF>(command: &Move, cargo: &mut Vec<VecDeque<char>>, crane: MF)
where
    MF: Fn(&Move, &mut VecDeque<char>, &mut VecDeque<char>) -> (),
{
    if command.to < command.from {
        if let [to_vec, .., from_vec] = &mut cargo[command.to..=command.from] {
            crane(&command, to_vec, from_vec);
        }
    } else {
        if let [from_vec, .., to_vec] = &mut cargo[command.from..=command.to] {
            crane(&command, to_vec, from_vec);
        }
    }
}
fn cargo9001_swap(
    command: &Move,
    to_vec: &mut VecDeque<char>,
    from_vec: &mut VecDeque<char>,
) {
    let mut temp = Vec::new();
    for _ in 0..command.num {
        if let Some(item) = from_vec.pop_front() {
            temp.push(item);
        }
    }

    while let Some(item) = temp.pop() {
        to_vec.push_front(item);
    }
}

fn cargo9000_swap(
    command: &Move,
    to_vec: &mut VecDeque<char>,
    from_vec: &mut VecDeque<char>,
) {
    for _ in 0..command.num {
        if let Some(item) = from_vec.pop_front() {
            to_vec.push_front(item);
        }
    }
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    build_cargo_lanes(&mut lines);
}
