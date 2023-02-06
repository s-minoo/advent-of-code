use std::{collections::HashSet, io::BufRead, process::exit};

fn score_item(item: char) -> u32 {
    let delta;
    if item.is_lowercase() {
        delta = 96;
    } else {
        delta = 65 - 27
    }

    return item as u32 - delta;
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    let mut point = 0;
    let mut grouped_point = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut buf = vec![];

        buf.push(line);
        buf.push(lines.next().unwrap().unwrap());
        buf.push(lines.next().unwrap().unwrap());

        let mut set = HashSet::new();

        for l in buf {
            if set.is_empty() {

                set =  l.chars().collect();
            } else {
                set = set.intersection(&l.chars().collect()).copied().collect();
            }

            get_individual_scores(&l, &mut point);
        }

        grouped_point += score_item(set.into_iter().next().unwrap());


    }

    println!("Grouped sum: {}", grouped_point); 
    println!("Sum priorities: {}", point);
}

fn get_individual_scores(line: &str, point: &mut u32) {
    let (left_pack, right_pack) = line.split_at(line.len() / 2);

    let left_unique: HashSet<char> = left_pack.chars().collect();

    for item in right_pack.chars() {
        if left_unique.contains(&item) {
            *point += score_item(item);
            break;
        }
    }
}
