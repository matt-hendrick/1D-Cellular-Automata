use rand::Rng;
use std::collections::HashMap;
use std::env::args;
use std::{thread, time};

fn main() {
    let mut current_row: Vec<u32> = generate_starting_row();
    let chosen_rule_set: u32 = parse_rule_from_args();
    println!(
        "Rule set = {}. Binary = {chosen_rule_set:08b}",
        chosen_rule_set
    );
    let rule_set: HashMap<String, u32> = get_rule_set(chosen_rule_set);
    println!("Hashset = {:?}", rule_set);
    print_row(&current_row);
    loop {
        let duration = time::Duration::from_millis(100);

        thread::sleep(duration);
        current_row = calc_new_row(current_row, &rule_set);
        print_row(&current_row);
    }
}

fn print_row(current_row: &Vec<u32>) {
    for num in current_row {
        if *num == 0 {
            print!(".")
        } else {
            print!("X");
        }
    }

    println!();
}

fn parse_rule_from_args() -> u32 {
    if args().len() < 2 {
        println!("No rule set specified. A random rule set will be chosen.");
        rand::thread_rng().gen_range(0..256)
    } else {
        let parsed: Result<u32, std::num::ParseIntError> = args().nth(1).unwrap().parse::<u32>();
        if parsed.is_ok() {
            let result = parsed.unwrap();
            if result <= 256 {
                result
            } else {
                rand::thread_rng().gen_range(0..256)
            }
        } else {
            rand::thread_rng().gen_range(0..256)
        }
    }
}

fn parse_row_size_from_args() -> usize {
    if args().len() < 3 {
        println!("No vec size specified. Vec will be set to to a size of 155.");
        155
    } else {
        let parsed: Result<usize, std::num::ParseIntError> =
            args().nth(2).unwrap().parse::<usize>();
        if parsed.is_ok() {
            parsed.unwrap()
        } else {
            155
        }
    }
}

fn parse_randomize_starting_row_from_args() -> bool {
    if args().len() < 4 {
        println!("No parameter passed in to randomize the starting vec. Starting vec will have a single filled in value at the midpoint.");
        false
    } else {
        println!(
            "Parameter passed in to randomize the starting vec. Starting vec will be pseudo-random"
        );
        true
    }
}

fn generate_starting_row() -> Vec<u32> {
    let size: usize = parse_row_size_from_args();
    let random_start: bool = parse_randomize_starting_row_from_args();

    let mut starting_row: Vec<u32>;

    if random_start {
        starting_row = Vec::new();

        for _ in 1..size + 1 {
            let randomish = rand::thread_rng().gen_range(0..2);
            starting_row.push(randomish);
        }
    } else {
        starting_row = vec![0; size];
        let midpoint = ((starting_row.len() / 2) as f32).floor();
        starting_row[midpoint as usize] = 1;
    }
    return starting_row;
}

fn get_u32_at_binary_string_index(binary_string: &String, index: usize) -> u32 {
    binary_string
        .chars()
        .nth(index)
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn get_rule_set(rule_set: u32) -> HashMap<String, u32> {
    let rule_set_binary: String = format!("{rule_set:08b}");
    let mut hashmap = HashMap::new();

    hashmap.insert(
        "111".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 0),
    );
    hashmap.insert(
        "110".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 1),
    );
    hashmap.insert(
        "101".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 2),
    );
    hashmap.insert(
        "100".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 3),
    );
    hashmap.insert(
        "011".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 4),
    );
    hashmap.insert(
        "010".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 5),
    );
    hashmap.insert(
        "001".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 6),
    );
    hashmap.insert(
        "000".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 7),
    );

    return hashmap;
}

fn calc_new_row(old_row: Vec<u32>, rule_set: &HashMap<String, u32>) -> Vec<u32> {
    let mut new_row: Vec<u32> = Vec::new();

    for (i, num) in old_row.iter().enumerate() {
        let mut curr_key: String = "".to_string();
        if i == 0 {
            curr_key += &old_row[old_row.len() - 1].to_string();
        } else {
            curr_key += &old_row[i - 1].to_string();
        }

        curr_key += &num.to_string();

        if i == old_row.len() - 1 {
            curr_key += &old_row[0].to_string();
        } else {
            curr_key += &old_row[i + 1].to_string();
        }

        let new_val: &u32 = rule_set.get(&curr_key).unwrap();

        new_row.push(*new_val);
    }

    return new_row;
}
