use std::{fs, collections::{HashMap, HashSet, LinkedList}, cell::RefCell, rc::Rc};

fn main() {
    aoc1();
    aoc2();
    aoc3();
    aoc4();
    aoc5();
    aoc6();
    aoc7();
    aoc8();
    aoc9();
    aoc10();
    aoc11();
    aoc12();
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

fn aoc7() {
    let input = get_input("7");

    struct Directory {
        size: u32,
        name: String,
        children: Vec<Rc<RefCell<Directory>>>,
        parent: Option<Rc<RefCell<Directory>>>
    }

    let head = Rc::new(RefCell::new(Directory {
        size: 0,
        name: String::from("\\"),
        children: vec![],
        parent: None
    }));
    let mut current = Rc::clone(&head);

    let lines: Vec<&str> = input.lines().collect();
    let feed = &lines[1..];

    for line in feed {
        if line.starts_with("$ cd") {
            let dir_name = line.split_ascii_whitespace().collect::<Vec<&str>>()[2];
            if dir_name.eq("..") {
                let parent = Rc::clone(current.borrow().parent.as_ref().unwrap());
                current = parent;
            } else {
                let tmp = Rc::clone(&current);
                for child in &tmp.borrow().children {
                    if child.borrow().name.eq(dir_name) {
                        current = Rc::clone(&child);
                    }
                }
            } 
        } else if line.starts_with("$ ls") {
            continue; 
        } else if line.starts_with("dir ") {
            let dir_name = line.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            let child = Rc::new(RefCell::new(Directory {
                size: 0,
                name: String::from(dir_name),
                children: vec![],
                parent: Some(Rc::clone(&current))
            }));
            (*current).borrow_mut().children.push(Rc::clone(&child));
        } else {
            let s = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let size = s[0].parse::<u32>().unwrap();
            let name = s[1];
            let new_child = Rc::new(RefCell::new(Directory {
                size: size,
                name: String::from(name),
                children: vec![],
                parent: Some(Rc::clone(&current))
            }));
            (*current).borrow_mut().children.push(Rc::clone(&new_child));
        }
    }

    // Walk tree to build size
    fn walk_build(dir: Rc<RefCell<Directory>>) -> u32 {
        let mut size = (*dir).borrow().size;
        let mut under_max = 0;
        for child in &(*dir).borrow().children {
            under_max += walk_build(Rc::clone(child));
            size += child.borrow().size;
        }
        (*dir).borrow_mut().size = size;
        let k = (*dir).borrow();
        if k.children.len() > 0 && k.size < 100000 {
            under_max += k.size;
        }
        under_max
    }

    fn walk_biggest(dir: &Rc<RefCell<Directory>>, needed_space: u32) -> u32 {
        let mut biggest = 0;
        if dir.borrow().size > needed_space {
            biggest = dir.borrow().size;
        }
        for child in &dir.borrow().children {
            let child_biggest = walk_biggest(child, needed_space);
            if child_biggest > needed_space && child_biggest < biggest {
                biggest = child_biggest;
            }
        }

        biggest
    }

    let sum_under_10k = walk_build(Rc::clone(&head));

    let free_space_left = 70000000 - head.borrow().size;
    let space_needed = 30000000 - free_space_left;

    let biggest_to_delete = walk_biggest(&Rc::clone(&head), space_needed);


    println!("7.");
    println!("\tPart 1: {}", sum_under_10k);
    println!("\tPart 2: {}", biggest_to_delete);
}

fn aoc8() {
    let input = get_input("8");
    let mut trees: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let mut tree_line = vec![];
        for ch in line.chars() {
            tree_line.push(ch.to_digit(10).unwrap());
        }
        trees.push(tree_line);
    }


    let mut visible = HashSet::new();

    // L to R
    for iy in 0..trees.len() {
        let mut highest_l:i32 = -1;
        let mut highest_r:i32 = -1;
        for ix in 0..trees[0].len() {
            let tree_height = trees[iy][ix] as i32;
            if tree_height > highest_l {
                highest_l = tree_height;
                visible.insert((iy, ix));
            }
        }
        for ix in (0..trees[0].len()).rev() {
            let tree_height = trees[iy][ix] as i32;
            if tree_height > highest_r {
                highest_r = tree_height;
                visible.insert((iy, ix));
            }
        }
    }

    // T to B
    for ix in 0..trees[0].len() {
        let mut highest_t:i32 = -1;
        let mut highest_b:i32 = -1;
        for iy in 0..trees.len() {
            let tree_height = trees[iy][ix] as i32;
            if tree_height > highest_t {
                highest_t = tree_height;
                visible.insert((iy, ix));
            }
        }

        for iy in (0..trees.len()).rev() {
            let tree_height = trees[iy][ix] as i32;
            if tree_height > highest_b {
                highest_b = tree_height;
                visible.insert((iy, ix));
            }
        }
    }
    fn scenic_score(y: usize, x: usize, trees: &Vec<Vec<u32>>) -> u32 {
        let view_height = trees[y][x] as i32;
        let mut score = 1;
        let mut seen = 0;
        // right
        for ix in x+1..trees[0].len() {
            let seen_tree = trees[y][ix] as i32;
            seen += 1; 
            if seen_tree >= view_height {
                break;
            }
        }
        score *= seen;
        seen = 0;

        // left
        if x > 0 {
            for ix in (0..=x-1).rev() {
                let seen_tree = trees[y][ix] as i32;
                seen += 1;
                if seen_tree >= view_height {
                    break;
                }
            }
        }
        score *= seen;
        seen = 0;

        // up
        if y > 0 {
            for iy in (0..=y-1).rev() {
                let seen_tree = trees[iy][x] as i32;
                seen += 1;
                if seen_tree >= view_height {
                    break;
                }
            }
        }
        score *= seen;
        seen = 0;
        
        // down
        for iy in y+1..trees.len() {
            let seen_tree = trees[iy][x] as i32;
            seen += 1;
            if seen_tree >= view_height {
                break;
            }
        }
        score *= seen;
        


        score as u32
    }

    let mut max_scenic = 0;
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            let score = scenic_score(y, x, &trees);
            if score > max_scenic {
                max_scenic = score;
            }
        }
    }



    println!("8.");
    println!("\tPart 1: {}", visible.len());
    println!("\tPart 2: {}", max_scenic);
}

fn aoc9() {
    let input = get_input("9");

    fn adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
        (a.0.abs_diff(b.0) <= 1) && (a.1.abs_diff(b.1) <= 1)
    }

    let mut second_visited = HashSet::new();
    second_visited.insert((0,0));
    let mut last_visited = HashSet::new();
    last_visited.insert((0, 0));

    let mut rope = vec![
        (0,0), (0,0), (0,0), (0,0), (0,0),
        (0,0), (0,0), (0,0), (0,0), (0,0)
    ];
    for line in input.lines() {
        let s:Vec<&str> = line.split_ascii_whitespace().collect();
        let direction = s[0];
        let ammount = String::from(s[1]).parse::<u32>().unwrap();
        for _ in 0..ammount {
            for i in 0..rope.len() {

                // Move head
                if i == 0 { 
                    match direction {
                        "D" => {
                            rope[0].1 -= 1; 
                        },
                        "U" => {
                            rope[0].1 += 1;
                        },
                        "R" => {
                            rope[0].0 += 1;
                        },
                        "L" => {
                            rope[0].0 -= 1;
                        }
                        _ => {
                            panic!("Not sure what {} is!", direction);
                        }
                    }
                } else {
                    if !adjacent(rope[i-1], rope[i]) {
                        // Same x
                        if rope[i-1].0 == rope[i].0 {
                            if rope[i-1].1 > rope[i].1 {
                                rope[i].1 +=1;
                            } else {
                                rope[i].1 -=1;
                            }
                        } 
                        // Same y
                        else if rope[i-1].1 == rope[i].1 {
                            if rope[i-1].0 > rope[i].0 {
                                rope[i].0 += 1;
                            } else {
                                rope[i].0 -= 1;
                            }
                        }
                        // Diagonal
                        else {
                            if rope[i-1].0 > rope[i].0 {
                                rope[i].0 += 1;
                            } else {
                                rope[i].0 -= 1;
                            }

                            if rope[i-1].1 > rope[i].1 {
                                rope[i].1 += 1;
                            } else {
                                rope[i].1 -= 1;
                            }
                        }


                        // If second moved, store
                        if i == 1 {
                            second_visited.insert(rope[i]);
                        }

                        // If tail moved, store
                        if i == rope.len() -1 {
                           last_visited.insert(rope[i]); 
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("9.");
    println!("\tPart 1: {}", second_visited.len());
    println!("\tPart 2: {}", last_visited.len());
}

fn aoc10() {
    let input = get_input("10");


    #[derive(Clone, Copy)]
    enum Command {
        NOOP,
        ADDX{cycles: u32, value: i32}
    }
    
    let mut pipeline:Vec<Command> = vec![];

    for line in input.lines() {
        let s:Vec<&str> = line.split_ascii_whitespace().collect();
        let command = s[0];
        match command {
            "noop" => {
                pipeline.push(Command::NOOP);
            },
            "addx" => {
                let v = s[1].parse::<i32>().unwrap();
                pipeline.push(Command::ADDX { cycles: 2, value: v });
            },
            _ => {
                panic!("Not sure what {} is!", command);
            }
        }
    }
    pipeline.reverse();

    // Part 1
    let mut x = 1;
    let mut cycle = 1;
    let mut strengths = 0;

    // Part 2
    let mut crt = vec![vec![0; 40]; 6];

    // Run    
    while pipeline.len() > 0 {
        let command = &mut pipeline.last_mut().unwrap();
        
        // Draw
        let middle = cycle - 1;
        if (x - middle%40 as i32).abs() <= 1 {
            crt[(middle/40) as usize][(middle%40) as usize] = 1;
        }
        

        // Decrement command cycle count
        let should_pop = match command {
            Command::NOOP => {
                true
            },
            Command::ADDX { cycles, .. } => {
                *cycles -= 1;
                *cycles == 0
            }
        };



        // Check cycle count
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            strengths += cycle * x;
        }
        
        // Pop and process command if done waiting
        if should_pop {
            let command = pipeline.pop().unwrap();
            match command {
                Command::NOOP => {},
                Command::ADDX {value, ..} => {
                    x += value;
                }
            }
        }
        cycle += 1;
    }

    println!("10.\n");
    println!("\tPart 1: {}", strengths);
    println!("\tPart 2:");
    for line in crt {
        print!("\t\t");
        for pixel in line {
            if pixel == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn aoc11() {
    let input = get_input("11");
    let lines:Vec<&str> = input.lines().collect();
    let mut i = 0;

    #[derive(Debug, Clone, Copy)]
    enum Op {
        AddN(u32),
        MulN(u32),
        AddOld,
        MulOld
    }

    #[derive(Clone)]
    struct Monkey {
        items: LinkedList<u32>,
        op: Op,
        div: u32,
        t_monkey: usize,
        f_monkey: usize,
        inspected: u32
    }

    let mut monkeys:Vec<Monkey> = vec![];

    while i < lines.len() {
        // Starting items
        let mut items = vec![];
        let s = lines[i+1].split(": ").collect::<Vec<&str>>()[1];
        let item_raw = s.split(", ").collect::<Vec<&str>>();
        for item in item_raw {
            items.push(item.parse::<u32>().unwrap());
        }

        // Operation
        let s = lines[i+2].split("= old ").collect::<Vec<&str>>()[1].split_ascii_whitespace().collect::<Vec<&str>>(); 
        let op = match s[1] {
            "old" => {
                match s[0] {
                    "+" => Op::AddOld,
                    "*" => Op::MulOld,
                    _ => {panic!("I don't know what {} is!", s[0])}
                }
            }
            _ => {
                match s[0] {
                    "+" => Op::AddN(s[1].parse::<u32>().unwrap()),
                    "*" => Op::MulN(s[1].parse::<u32>().unwrap()),
                    _ => {panic!("I don't know what {} is!", s[0])}
                }
            }
        };

        // Test
        let div = lines[i+3].split_ascii_whitespace().collect::<Vec<&str>>()[3].parse::<u32>().unwrap();
        let t_monkey = lines[i+4].split_ascii_whitespace().collect::<Vec<&str>>()[5].parse::<usize>().unwrap();
        let f_monkey = lines[i+5].split_ascii_whitespace().collect::<Vec<&str>>()[5].parse::<usize>().unwrap();

        let mut item_list = LinkedList::new();
        for item in items {
            item_list.push_back(item);
        }
        monkeys.push(Monkey { items: item_list, op: op, div: div, t_monkey: t_monkey, f_monkey: f_monkey, inspected: 0 });


        i += 7;
    }
    let mut lcm:u64 = 1;
    for monkey in &monkeys {
        lcm *= monkey.div as u64;
    }
    
    let mut monkey_copy = monkeys.to_vec();

    fn process_monkey(monkey: &mut Monkey, div: bool, lcm: u64) -> Vec<(usize, u32)> {
        let mut to_throw = vec![];
        while !monkey.items.is_empty() {
            let item = monkey.items.pop_front().unwrap() as u64;
            monkey.inspected += 1;
            let new = match monkey.op {
                Op::AddN(n) => {
                    item + n as u64
                },
                Op::MulN(n) => {
                    item * n as u64
                },
                Op::AddOld => {
                    item + item as u64
                },
                Op::MulOld => {
                    item * item as u64
                }
            };
            let new_m = (new % lcm) as f64;
            let new_rounded = if div {(new_m / 3.0).floor() as u32} else {new_m as u32}; 
            if new_rounded % monkey.div == 0 {
                to_throw.push((monkey.t_monkey, new_rounded));
            } else {
                to_throw.push((monkey.f_monkey, new_rounded));
            }
        }
        to_throw
    }

    fn monkey_business(monkeys: Vec<Monkey>) -> u64 {
        let mut inspected:Vec<u32> = vec![];
        for monkey in monkeys {
            inspected.push(monkey.inspected);
        }
        inspected.sort();
        inspected.reverse();
        inspected[0] as u64 * inspected[1] as u64

    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let to_push = process_monkey(monkey, true, lcm);
            for p in to_push {
                monkeys[p.0].items.push_back(p.1);
            }
        }
    }
    for _ in 0..10000 {
        for i in 0..monkey_copy.len() {
            let monkey = &mut monkey_copy[i];
            let to_push = process_monkey(monkey, false, lcm);
            for p in to_push {
                monkey_copy[p.0].items.push_back(p.1);
            }
        }
    }


    
    println!("11.");
    println!("\tPart 1: {}", monkey_business(monkeys));
    println!("\tPart 2: {}", monkey_business(monkey_copy))
}

fn aoc12() {
    let input = get_input("12");
    let lines:Vec<&str> = input.lines().collect();
    let mut target = (0, 0);
    let mut start = (0, 0);

    let mut terrain:Vec<Vec<u32>> = Vec::with_capacity(lines.len());

    let mut y = 0;
    for line in &lines {
        let mut l:Vec<u32> = Vec::with_capacity(lines[0].len());
        let mut x = 0;
        for c in line.chars() {
            let k = match c {
                'S' => {
                    start = (x, y);
                    0
                },
                'E' => {
                    target = (x, y);
                    'z' as u32 - 'a' as u32
                },
                _ => c as u32 - 'a' as u32
            };
            l.push(k);
            x += 1;
        }
        terrain.push(l);
        y += 1;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
    struct Location {
        coordinate: (usize, usize)
    }

    fn can_move(from: (usize, usize), to: (usize, usize), t: &Vec<Vec<u32>>) -> bool {
        t[to.1][to.0] as i32 - t[from.1][from.0] as i32 <= 1
    }

    fn neighbors(l: &Location, terrain: &Vec<Vec<u32>>) -> Vec<Location> {
        let mut n = vec![];

        // Right
        if l.coordinate.0 < terrain[0].len() - 1 {
            let next = (l.coordinate.0+1, l.coordinate.1);
            if can_move(l.coordinate, next, terrain) {
                n.push(Location { coordinate: next});
            }
        }

        // Up
        if l.coordinate.1 > 0 {
            let next = (l.coordinate.0, l.coordinate.1 - 1);
            if can_move(l.coordinate, next, terrain) {
                n.push(Location {coordinate: next});
            }
        }

        // Down
        if l.coordinate.1 < terrain.len() - 1 {
            let next = (l.coordinate.0, l.coordinate.1 + 1);
            if can_move(l.coordinate, next, terrain) {
                n.push(Location {coordinate: next});
            }
        }
        // Left
        if l.coordinate.0 > 0 {
            let next = (l.coordinate.0-1, l.coordinate.1);
            if can_move(l.coordinate, next, terrain) {
                n.push(Location {coordinate: next});
            }
        }
        n
    }

    fn search(terrain: &Vec<Vec<u32>>, start: (usize, usize), target: (usize, usize)) -> Option<u32> {
        let mut queue:LinkedList<Location> = LinkedList::new();
        let mut came_from:HashMap<Location, Option<Location>> = HashMap::new();

        let s = Location {coordinate: start};
        
        queue.push_front(s.clone());
        came_from.insert(s.clone(), None);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if current.coordinate == target {
                break;
            }

            for neighbor in neighbors(&current, terrain) {
                if !came_from.contains_key(&neighbor) {
                    queue.push_back(neighbor.clone());
                    came_from.insert(neighbor, Some(current.clone()));
                }
            }
        }

        // Print came_from
        unwind(&came_from, &Location { coordinate: target })
    }

    fn unwind(map: &HashMap<Location, Option<Location>>, l: &Location) -> Option<u32> {
        match map.get(&l) {
            None => None,
            Some(location) => {
                match location {
                    None => Some(0),
                    Some(previous) => match unwind(map, previous) {
                        Some(n) => Some(n+1),
                        None => None
                    }
                }
            }
        }
    }

    let mut shortest_distance = u32::MAX;
    let mut to_search:Vec<(usize, usize)> = vec![];

    let mut y = 0;
    for row in &terrain {
        let mut x = 0;
        for sq in row {
            if *sq == 0 {
                to_search.push((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    for s in to_search {
        match search(&terrain, s, target) {
            Some(dist) => {
                if dist < shortest_distance {
                    shortest_distance = dist
                }
            },
            None => {}
        }
    }


    println!("12.");
    println!("\tPart 1: {}", search(&terrain, start, target).unwrap());
    println!("\tPart 2: {}", shortest_distance);
}