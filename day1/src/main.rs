extern crate core;

use std::fs;
use std::path::Component::Prefix;

fn main() {
    let input= fs::read_to_string("src/input.txt").unwrap_or_default();

    let packs = input.split("\n\n");

    let mut top_three_packs :[i32; 3] = [0,0,0];

    for pack in packs {
        let rations = pack.split("\n");
        let mut sub_total = 0;
        for ration in rations {
            if !ration.is_empty() {
                let ration_value :i32 = ration.parse().unwrap();
                sub_total += ration_value;
            }
        }
        println!("Total: {}", sub_total);
        if sub_total > top_three_packs[0] {
            top_three_packs[2] = top_three_packs[1];
            top_three_packs[1] = top_three_packs[0];
            top_three_packs[0] = sub_total;
        } else if sub_total > top_three_packs[1]{
            top_three_packs[2] = top_three_packs[1];
            top_three_packs[1] = sub_total;
        } else if sub_total > top_three_packs[2]{
            top_three_packs[2] = sub_total;
        }
        println!("\n");
    }

    println!("Top Cal pack: {}", top_three_packs[0]);
    println!("Second Cal pack: {}", top_three_packs[1]);
    println!("Third Cal pack: {}", top_three_packs[2]);

    let mut total = 0;
    for pack_total in top_three_packs {
        total = total + pack_total;
    }

    println!("Total top 3: {}", total);
}