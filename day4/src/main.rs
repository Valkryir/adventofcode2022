use std::fs;
use std::process::exit;

fn main() {

    let assignments= fs::read_to_string("src/input.txt").unwrap_or_default();
    if assignments.is_empty() {
        println!("INPUTFAILED");
        exit(1);
    }

    let assignments = assignments.split("\n");

    let mut overlaps = 0;

    for pair in assignments {
        let mut zones: Vec<i32> = Vec::new();
        let assign = pair.split(",");
        for a in assign {
            let zones_split = a.split("-");
            for zone in zones_split {
                zones.push(zone.parse().unwrap())
            }
        }

        //part one
        if zones[0] <= zones[2] && zones[1] >= zones[3] {
            overlaps = overlaps + 1;
            //first_contains_second(zones);
        } else if zones[2] <= zones[0] && zones[3] >= zones[1] {
            overlaps = overlaps + 1;
            //check_second_contains_first(zones);
        //part two
        } else if zones[1] >= zones[2] &&  zones[0] <= zones[2]{
            overlaps = overlaps + 1;
        } else if zones[3] >= zones[0] && zones[1] >= zones[3] {
            overlaps = overlaps + 1;
        }


    }

    println!("Overlaps: {}", overlaps);

}

// fn first_contains_second(zone: Vec<i32>) {
//     println!("First Assign: {}-{} contains second assign: {}-{}", zone[0], zone[1], zone[2], zone[3]);
// }
//
// fn check_second_contains_first(zone: Vec<i32>) {
//     println!("second Assign: {}-{} contains first assign: {}-{}", zone[2], zone[3], zone[0], zone[1],);
// }
