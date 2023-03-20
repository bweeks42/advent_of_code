use std::fs;

fn main() {
    aoc1();
}

fn get_input(num: &str) -> String {
    let mut path = String::from("./input/day");
    path.push_str(num);
    path.push_str(".txt");
    
    fs::read_to_string(path).unwrap()
}

// Day 1
fn aoc1() {

    struct Elf {
        items: Vec<u32>,
        total_calories: u32
    }

    let mut elves: Vec<Elf> = vec![];
    let input = get_input("1");
    let groups = input.split("\n\n");

    for group in groups {
        let lines = group.split("\n");
        let mut elf = Elf{items: vec![], total_calories: 0};
        for line in lines {
            let val = line.parse::<u32>().unwrap();
            elf.items.push(val);
            elf.total_calories += val
        }
        elves.push(elf);
    }

    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    println!("1.");
    println!("\tPart 1: {}", elves[0].total_calories);
    println!("\tPart 2: {}", elves[0].total_calories + elves[1].total_calories + elves[2].total_calories);
}
