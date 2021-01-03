use std::collections::HashMap;

fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day6.txt").split("\n\n").collect()
}

fn parse_input_pt_1(input: Vec<&'static str>) -> Vec<String> {
    let mut parsed_input: Vec<String> = Vec::with_capacity(input.len());
    for line in input.iter() {
        parsed_input.push(line.replace("\n", ""))
    }

    parsed_input
}

fn parse_input_pt_2(input: Vec<&'static str>) -> Vec<Vec<&str>> {
    let parsed_input = input
        .iter()
        .map(|group| group.split("\n").collect()).collect();

    parsed_input
}

fn answered_yes(group: &String) -> usize {
    let mut letter_map: HashMap<char, bool> = HashMap::with_capacity(6);
    let mut yes_count: usize = 0;
    for letter in group.chars() {
        if letter_map.get(&letter).is_none() {
            yes_count += 1;
            letter_map.insert(letter, true);
        }
    }

    yes_count
}

fn everyone_yes(group: Vec<&str>) -> usize {
    let mut letter_map: HashMap<char, usize> = HashMap::new();
    let mut yes_count: usize = 0;
    for p in group.iter() {
        for letter in p.chars() {
            // if the letter is not already in the map, add it with a value of 1
            if letter_map.get(&letter).is_none() {
                letter_map.insert(letter, 1);
            } else {
                // otherwise increment the count of the letter
                let letter_count = letter_map.get(&letter).unwrap() + 1;
                letter_map.insert(letter, letter_count);
            }
        }
    }

    for (_key, value) in letter_map.iter() {
        if value == &group.len() {
            yes_count += 1;
        }
    }
    yes_count
}

pub fn get_solution_pt_1() -> usize {
    let parsed_input = parse_input_pt_1(get_input());
    let mut yes_count: usize = 0;
    for group in parsed_input.iter() {
        yes_count += answered_yes(group)
    }
    yes_count
}

pub fn get_solution_pt_2() -> usize {
    let parsed_input: Vec<Vec<&str>> = parse_input_pt_2(get_input());
    let mut yes_count: usize = 0;
    for group in parsed_input {
        yes_count += everyone_yes(group)
    }
    yes_count
}