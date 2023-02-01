use std::{collections::BinaryHeap, io::BufRead};

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    let mut current_elf: u32 = 0;
    let mut binary_heap = BinaryHeap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            binary_heap.push(current_elf);
            current_elf = 0;
        } else {
            let val: u32 = line.parse().unwrap();
            current_elf += val;
        }
    }

    println!("Max cal is: {}", binary_heap.peek().unwrap());

    let mut buf = vec![0];
    buf.push(binary_heap.pop().unwrap());
    buf.push(binary_heap.pop().unwrap());
    buf.push(binary_heap.pop().unwrap());

    println!("Top 3 total: {}", buf.into_iter().sum::<u32>());
}
