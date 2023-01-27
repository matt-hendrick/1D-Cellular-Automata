use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};

fn main() {
    let mut current_vec = generate_starting_vec(50);
    let rule_set = get_rule_set();
    println!("{:?}", current_vec);
    while true {
        let duration = time::Duration::from_millis(100);

        thread::sleep(duration);
        current_vec = calc_new_vec(current_vec, &rule_set);
        println!("{:?}", current_vec);
    }
}

fn generate_starting_vec(size: usize) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for i in 1..size + 1 {
        let randomish = rand::thread_rng().gen_range(0..2);
        new_vec.push(randomish);
    }

    return new_vec;
}

fn get_rule_set() -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();

    hashmap.insert("000".to_string(), 1);
    hashmap.insert("100".to_string(), 0);
    hashmap.insert("010".to_string(), 1);
    hashmap.insert("001".to_string(), 0);
    hashmap.insert("110".to_string(), 1);
    hashmap.insert("011".to_string(), 0);
    hashmap.insert("101".to_string(), 1);
    hashmap.insert("111".to_string(), 1);

    return hashmap;
}

fn calc_new_vec(old_vec: Vec<i32>, rule_set: &HashMap<String, i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

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
        new_vec.push(*new_val);
    }

    return new_vec;
}
