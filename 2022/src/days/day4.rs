use std::io::BufRead;

fn get_start_end(section: &str) -> (u32, u32) {
    let splitted: Vec<&str> = section.split("-").collect();
    let (start, end) = (splitted[0].parse::<u32>(), splitted[1].parse::<u32>());

    (start.unwrap(), end.unwrap())
}
fn is_overlapped(left: &str, right: &str) -> bool {
    let (left_start, left_end) = get_start_end(left);
    let (right_start, right_end) = get_start_end(right);

    (left_start <= right_start && right_start <= left_end)
        || (right_start <= left_start && left_start <= right_end)
}

fn is_contained(left: &str, right: &str) -> bool {
    let (left_start, left_end) = get_start_end(left);
    let (right_start, right_end) = get_start_end(right);

    (left_start <= right_start && left_end >= right_end)
        || (right_start <= left_start && right_end >= left_end)
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();
    let mut count = 0;

    let mut overlap = 0;
    while let Some(Ok(line)) = lines.next() {
        let splitted: Vec<&str> = line.split(",").collect();

        let (first_section, second_section) = (splitted[0], splitted[1]);

        if is_contained(first_section, second_section) {
            overlap += 1;
            count += 1;
        } else if is_overlapped(first_section, second_section) {
            overlap += 1;
        }
    }

    println!("Contained count: {}", count);
    println!("Overlapping count: {}", overlap);
}
