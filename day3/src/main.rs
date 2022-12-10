use std::collections::HashMap;
use std::fs;

fn main() {
    let values = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    //let input = fs::read_to_string("src/testInput.txt").unwrap_or_default();
    let input = fs::read_to_string("src/input.txt").unwrap_or_default();
    let rucksacks = input.split("\n");

    let mut priority_sum = 0;

    let mut team_count = 0;
    let mut teams_count = 0;

    let mut teams = HashMap::new();
    let mut team = ["", "", ""];

    for rucksack in rucksacks {
        team[team_count] = rucksack;
        team_count = team_count + 1;

        if team_count == 3 {
            teams.insert(teams_count, team);
            team = ["", "", ""];
            team_count = 0;
            teams_count = teams_count + 1;
        }
    }

    //println!("{}", teams_count);

    for team in teams {
        println!("{:#?}", team);
        let mut duplicate: char = '0';

        for char1 in team.1[0].chars() {
            for char2 in team.1[1].chars() {
                if char1 == char2 {
                    for char3 in team.1[2].chars() {
                        if char1 == char3 {
                            duplicate = char1;
                        }
                    }
                }
            }
        }
        let item_priority = values.iter().position(|x| *x == duplicate).unwrap();

        println!("dupe: {}", duplicate);
        println!("priority: {}", item_priority);
        priority_sum = priority_sum + item_priority + 1;
    }

    println!("total priority: {}", priority_sum);
}