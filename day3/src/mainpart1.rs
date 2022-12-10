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

    for rucksack in rucksacks {
        let (comp_one, comp_two) = rucksack.split_at(rucksack.len()/2);
        println!("{}", comp_one);
        println!("{}", comp_two);

        let mut duplicate :char = '0';

        for char1 in comp_one.chars() {
            for char2 in comp_two.chars() {
                if char1 == char2 {
                    duplicate = char1;
                }
            }
        }
        let item_priority = values.iter().position(|x| *x == duplicate).unwrap();

        println!("dupe: {}", duplicate);
        println!("priority: {}", item_priority);

        priority_sum = priority_sum + item_priority + 1;
    }
    println!("total: {}", priority_sum);
}
