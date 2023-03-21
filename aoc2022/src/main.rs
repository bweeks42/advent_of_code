use std::fs;

fn main() {
    aoc1();
    aoc2();
    aoc3();
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

// Day 2
fn aoc2() {
    use std::collections::HashMap;

    #[derive(Clone, Copy)]
    enum Choice {
        Rock = 1,
        Paper = 2,
        Scissors = 3
    }

    #[derive(Clone, Copy)]
    enum Outcome {
        Win = 6,
        Tie = 3,
        Lose = 0
    }
    use Choice::*;
    use Outcome::*;

    // Fastest way is probably to build a table and do a lookup, but this was easier to implement and not that slow
    fn round(a: Choice, b: Choice) -> Outcome {
        match a {
            Rock => match b {
                Rock => Tie,
                Paper => Win,
                Scissors => Lose
            },
            Paper => match b {
                Rock => Lose,
                Paper => Tie,
                Scissors => Win
            },
            Scissors => match b {
                Rock => Win,
                Paper => Lose,
                Scissors => Tie
            }
        }
    }

    fn choice_for_outcome(a: Choice, o: Outcome) -> Choice {
        match a {
            Rock => match o {
                Win => Paper,
                Tie => Rock,
                Lose => Scissors
            },
            Paper => match o {
                Win => Scissors,
                Tie => Paper,
                Lose => Rock
            }
            Scissors => match o {
                Win => Rock,
                Tie => Scissors,
                Lose => Paper
            }
        }
    }

    let first_mapping = HashMap::from([
        ('A', Rock),
        ('B', Paper),
        ('C', Scissors),
        ('X', Rock),
        ('Y', Paper),
        ('Z', Scissors)
    ]);
    
    let outcome_mapping = HashMap::from([
        ('X', Lose),
        ('Y', Tie),
        ('Z', Win)
    ]);
    

    let mut first_score: u32 = 0;
    let mut second_score: u32 = 0;

    let input = get_input("2");
    let lines = input.split("\n");
    for line in lines {
        let a = first_mapping.get(&line.chars().next().unwrap()).unwrap().to_owned();
        let b = first_mapping.get(&line.chars().nth(2).unwrap()).unwrap().to_owned();
        let outcome = outcome_mapping.get(&line.chars().nth(2).unwrap()).unwrap().to_owned();
        first_score += b as u32 + round(a, b) as u32;
        second_score += outcome as u32 + choice_for_outcome(a, outcome) as u32
    }

    println!("2.");
    println!("\tPart 1: {}", first_score);
    println!("\tPart 2: {}", second_score); 
}

// Day 3
fn aoc3() {

    fn char_to_priority(c: char) -> u32 {
        if c.is_uppercase() {
            c as u32 - 38
        } else {
            c as u32 - 96
        }
    }

    let input = get_input("3");
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for line in lines.clone() {
        let len = line.len();
        let compartment_a = &line[0..len/2];
        let compartment_b = &line[len/2..];
        for c in compartment_a.chars() {
            if compartment_b.contains(c) {
                sum += char_to_priority(c);
                break
            }
        }
    }

    let mut sum2 = 0;
    for i in (0..lines.len()).step_by(3) {
        let a = lines[i];
        let b = lines[i+1];
        let c = lines[i+2];
        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                sum2 += char_to_priority(ch);
                break
            }
        }

    }



    println!("3.");
    println!("\tPart 1: {}", sum);
    println!("\tPart 2: {}", sum2);


}