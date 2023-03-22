use std::{fs, collections::HashMap};

fn main() {
    aoc1();
    aoc2();
    aoc3();
    aoc4();
    aoc5();
    aoc6();
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

// Day 4
fn aoc4() {
    let input = get_input("4");
    let mut contained_pairs = 0;
    let mut overlapping_pairs = 0;
    for line in input.lines() {
        let elves:Vec<&str> = line.split(",").collect();
        let elf1 = elves[0];
        let elf2 = elves[1];

        let elf1_vals:Vec<&str> = elf1.split("-").collect();
        let elf1_low = elf1_vals[0].parse::<u32>().unwrap();
        let elf1_high = elf1_vals[1].parse::<u32>().unwrap();

        let elf2_vals:Vec<&str> = elf2.split("-").collect();
        let elf2_low = elf2_vals[0].parse::<u32>().unwrap();
        let elf2_high = elf2_vals[1].parse::<u32>().unwrap();

        if (elf1_low <= elf2_low && elf1_high >= elf2_high) || (elf1_low >= elf2_low && elf1_high <= elf2_high) {
            contained_pairs += 1;
        }

        if (elf1_low <= elf2_low && elf1_high >= elf2_low) || 
            (elf2_low <= elf1_low && elf2_high >= elf1_low) {
            overlapping_pairs += 1;
        }
    }

    println!("4.");
    println!("\tPart 1: {}", contained_pairs);
    println!("\tPart 2: {}", overlapping_pairs);
}

// Day 5
fn aoc5() {
    let input = get_input("5");
    let lines: Vec<&str> = input.lines().collect();

    // Skip the crate description as we'll just hard code that
    let instructions = &lines[10..];

    let mut crates = HashMap::from([
        ("1", vec!['Z', 'T', 'F', 'R', 'W', 'J', 'G']),
        ("2", vec!['G', 'W', 'M']),
        ("3", vec!['J', 'N', 'H', 'G']),
        ("4", vec!['J', 'R', 'C', 'N', 'W']),
        ("5", vec!['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M']),
        ("6", vec!['S', 'R', 'T', 'D', 'V', 'W', 'C']),
        ("7", vec!['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V']),
        ("8", vec!['S', 'J', 'N', 'M', 'G', 'C']),
        ("9", vec!['G', 'P', 'N', 'W', 'C', 'J', 'D', 'L'])
    ]);

    let mut second_crates = crates.clone(); // Need a copy for the second part of problem

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let count = parts[1].parse::<u32>().unwrap();
        let from = parts[3];
        let to = parts[5];
        
        // Part 1
        for _ in 0..count {
            let from_stack = crates.get_mut(from).unwrap();
            let c = from_stack.pop().unwrap();
            let to_stack = crates.get_mut(to).unwrap();
            to_stack.push(c);
        }

        // Part 2
        let mut to_move: Vec<char> = vec![];
        for _ in 0..count {
            let from_stack = second_crates.get_mut(from).unwrap();
            let c = from_stack.pop().unwrap();
            to_move.push(c);
        }
        to_move.reverse();

        let to_stack = second_crates.get_mut(to).unwrap();
        for ch in to_move {
            to_stack.push(ch);
        }
    }

    fn str_for_crates(h: &HashMap<&str, Vec<char>>) -> String {
        // Need to iterate as hashmap is unordered
        let mut o = String::new();
        for i in 1..=9 {
            let is = i.to_string(); 
            let ch = h.get(is.as_str()).unwrap().last().unwrap();
            o.push(*ch);
        }
        o
    }
    
    println!("5.");
    println!("\tPart 1: {}", str_for_crates(&crates));
    println!("\tPart 2: {}", str_for_crates(&second_crates));
}

fn aoc6() {
    let input = get_input("6");

    fn start_of_n_unique_in_buffer(n: usize, buffer: &String) -> usize {
        for i in n..buffer.len() {
            let s: Vec<char> = buffer[i-n..i].chars().collect();
            let mut seen_chars: Vec<char> = vec![];
            for ch in s {
                if seen_chars.contains(&ch) {
                    break;
                } else {
                    seen_chars.push(ch);
                }
            }
            if seen_chars.len() == n {
                return i;
            }
        }

        panic!("Should've found something.")
    }
    
    // Part 1: Start of Packet
    let packet_start = start_of_n_unique_in_buffer(4, &input);
    let message_start = start_of_n_unique_in_buffer(14, &input);
    println!("6.");
    println!("\tPart 1: {}", packet_start);
    println!("\tPart 2: {}", message_start);
}
