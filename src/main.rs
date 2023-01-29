use rand::Rng;
use std::collections::HashMap;
use std::env::args;
use std::{thread, time};

fn main() {
    let mut current_vec = generate_starting_vec(155);
    let chosen_rule_set = parse_rule_from_args();
    println!(
        "Rule set = {}. Binary = {chosen_rule_set:08b}",
        chosen_rule_set
    );
    let rule_set = get_rule_set(chosen_rule_set);
    println!("Hashset = {:?}", rule_set);
    print_row(&current_vec);
    loop {
        let duration = time::Duration::from_millis(100);

        thread::sleep(duration);
        current_vec = calc_new_vec(current_vec, &rule_set);
        print_row(&current_vec);
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
        let parsed = args().nth(1).unwrap().parse::<u32>();
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

fn generate_starting_vec(size: usize) -> Vec<u32> {
    // let mut new_vec: Vec<u32> = Vec::new();

    // for _ in 1..size + 1 {
    //     let randomish = rand::thread_rng().gen_range(0..2);
    //     new_vec.push(randomish);
    // }

    let mut new_vec: Vec<u32> = vec![0; size];
    let midpoint = ((new_vec.len() / 2) as f32).floor();
    new_vec[midpoint as usize] = 1;

    return new_vec;
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

fn calc_new_vec(old_vec: Vec<u32>, rule_set: &HashMap<String, u32>) -> Vec<u32> {
    let mut new_vec: Vec<u32> = Vec::new();

    for (i, num) in old_vec.iter().enumerate() {
        let mut curr_key: String = "".to_string();
        if i == 0 {
            curr_key += &old_vec[old_vec.len() - 1].to_string();
        } else {
            curr_key += &old_vec[i - 1].to_string();
        }

        curr_key += &num.to_string();

        if i == old_vec.len() - 1 {
            curr_key += &old_vec[0].to_string();
        } else {
            curr_key += &old_vec[i + 1].to_string();
        }

        let new_val = rule_set.get(&curr_key).unwrap();

        // println!("{}, {}, {}, {}", i, num, curr_key, new_val);

        new_vec.push(*new_val);
    }

    return new_vec;
}
