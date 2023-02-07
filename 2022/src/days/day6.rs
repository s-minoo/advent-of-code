use std::{collections::HashSet, io::BufRead};

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    while let Some(Ok(line)) = lines.next() {
        let char_vec = line.chars().collect::<Vec<char>>();
        let packet_marker = detect_start_marker(&char_vec, 4);
        let message_marker = detect_start_marker(&char_vec, 14);


        println!("Start-of-packet: {}",packet_marker);
        println!("Start-of-message: {}",message_marker);
    }
}

fn detect_start_marker(char_vec: &Vec<char>, len:usize) -> usize {
    let count = char_vec
        .as_slice()
        .windows(len)
        .take_while(|g| {
            let set: HashSet<&char> = g.iter().collect();
            set.len() != len
        })
        .count() + len;
    count
}
