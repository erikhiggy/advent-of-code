use std::collections::HashMap;

fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day6.txt").split("\r\n\r\n").collect()
}

fn parse_input(input: Vec<&'static str>) -> Vec<String> {
    let mut parsed_input: Vec<String> = Vec::with_capacity(input.len());
    for line in input.iter() {
        parsed_input.push(line.replace("\r\n", ""))
    }

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

pub fn get_solution_pt_1() -> usize {
    let parsed_input = parse_input(get_input());
    let mut yes_count: usize = 0;
    for group in parsed_input.iter() {
        yes_count += answered_yes(group)
    }
    yes_count
}