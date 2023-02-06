use std::{io::BufRead, process::exit};

fn get_victory_score(enemy: u32, player: u32) -> u32 {
    // draw
    if enemy == player {
        return 3;
    }

    // win
    if player > enemy && player - enemy < 2 {
        return 6;
    }

    if player < enemy && enemy - player == 2 {
        return 6;
    }

    //lose
    return 0;
}

fn get_round_score(enemy: u32, player: u32) -> u32 {
    let victory_score = get_victory_score(enemy, player);

    victory_score + (player)
}

fn get_hand(enemy: u32, player: u32) -> u32 {
    //draw
    if player == 2 {
        return enemy;
    }


    let mut hand;
    // lose
    if player == 1 {
        hand = enemy - 1;
        if hand == 0 {
            return 3;
        }
        return hand;
    }


    // win
    hand = (enemy + 1) % 3;
    if hand == 0 {
        return 3;
    }
    return hand;
}

pub fn solve(input: impl BufRead) {
    let mut lines = input.lines();

    let mut points = 0;
    let mut updated_points = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut round = line.split(" ").flat_map(|l| l.chars());
        let enemy = round.next().unwrap() as u32 - 64;
        let player = round.next().unwrap() as u32 - 64 - 23;
        points += get_round_score(enemy, player);
        updated_points += (player -1) * 3 + get_hand(enemy, player);
    }

    println!("Player scored: {}", points);
    println!("Updated Player score: {}", updated_points);
}
