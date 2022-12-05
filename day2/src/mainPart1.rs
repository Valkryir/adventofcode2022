use std::fs;
use std::process::exit;

// A > Z
// B > X
// C > Y

// X > C
// Y > A
// Z > B

fn main() {
    let opponent_codes = ["o", "A", "B", "C"];
    let player_codes = ["o", "X", "Y", "Z"];
    //    A,B,C
    let rules: [[u32; 4]; 4] = [
        [0,1,2,3],
        [1,3,0,6], //X
        [2,6,3,0], //Y
        [3,0,6,3], //Z
    ];

    let game_plan= fs::read_to_string("src/input.txt").unwrap_or_default();
    if game_plan.is_empty() {
        println!("INPUTFAILED");
        exit(1);
    }
    let game_plan = game_plan.split("\n");

    let mut my_score = 0;

    for round in game_plan {
        let mut plays = round.split(" ");
        let their_move = plays.next().unwrap();
        let my_move = plays.next().unwrap();

        let my_code = player_codes.iter().position(|&x| x == my_move).unwrap();
        let their_code = opponent_codes.iter().position(|&x| x == their_move).unwrap();

        let my_round_result_score = rules[my_code][their_code];
        let my_round_move_score = rules[0][my_code];

        my_score = my_score + my_round_result_score + my_round_move_score;
    }

    println!("My final score {}", my_score);
}
