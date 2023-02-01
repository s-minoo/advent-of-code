use std::io::BufRead;

fn round_score(enemy: u32, player: u32) -> u32 {




    todo!();
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    let mut points = 0; 
    while let Some(Ok(line)) = lines.next() {
        let mut round = line.split(" ").flat_map(|l| l.chars());
        let enemy = round.next().unwrap().to_digit(10).unwrap();
        let player = round.next().unwrap().to_digit(10).unwrap();
        points += round_score(enemy, player);
    }



    println!("Player scored: {}", points); 
}
