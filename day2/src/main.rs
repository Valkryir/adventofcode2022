use std::collections::HashMap;
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
    //let player_codes = ["o", "X", "Y", "Z"];
    //    A,B,C
    let rules: [[u32; 4]; 4] = [
        [0,1,2,5],
        [1,3,6,0], //X
        [2,0,3,6], //Y
        [5,6,0,3], //Z
    ];
    //5s replace 0 to avoid early detection of zero in part 2

    let mut win_codes :HashMap<&str, i32> = HashMap::new();
    win_codes.insert("X", 0);
    win_codes.insert("Y", 3);
    win_codes.insert("Z", 6);

    let game_plan= fs::read_to_string("src/input.txt").unwrap_or_default();
    //let game_plan= fs::read_to_string("src/testInput.txt").unwrap_or_default();
    if game_plan.is_empty() {
        println!("INPUTFAILED");
        exit(1);
    }
    let game_plan = game_plan.split("\n");

    let mut my_score: i32 = 0;
    let mut count = 0;

    for round in game_plan {
        let mut plays = round.split(" ");

        let their_move = plays.next().unwrap();
        let result = plays.next().unwrap();

        let result_code = *win_codes.get(result).unwrap();

        let their_code = opponent_codes.iter().position(|&x| x == their_move).unwrap();

        let my_code = rules[their_code].iter().position(|&x| x == result_code as u32).unwrap();

        let my_round_result_score:i32 = result_code as i32;
        let my_round_move_score = my_code as i32;

        my_score = my_score + my_round_result_score + my_round_move_score;

        count = count + 1;
    }

    println!("My final score {}", my_score);
    println!("Count: {}", count);
}
