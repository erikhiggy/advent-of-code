use std::collections::HashSet;

fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day7.txt").lines().collect()
}

fn find_shiny_gold(input: Vec<&str>) -> usize {
    let mut shiny_gold_count: usize = 0;
    let mut outer_bags: HashSet<String> = HashSet::new();
    for line in input.iter() {
        // this will give us the minimum number of
        // bags that can contain the shiny gold one
        if line.contains("shiny gold") && !line.starts_with("shiny gold") {
            outer_bags.insert(line[..line.find("bag").unwrap() - 1].parse().unwrap());
            shiny_gold_count += 1;
        }
    }

    // TODO: fix this condition and speed up
    let mut i = 0;
    while i < 6 {

        for bag in outer_bags.clone().iter() {
            for line in input.iter() {
                let sliced_line: String = line[..line.find("bag").unwrap() - 1].parse().unwrap();
                if line.contains(bag)
                    && !line.starts_with(bag)
                    && outer_bags.get(&sliced_line).is_none() {
                    outer_bags.insert(sliced_line);
                    shiny_gold_count += 1;
                }
            }
        }
        i += 1;
    }

    shiny_gold_count
}

fn shiny_gold_pt_2(input: Vec<&str>) -> usize {
    let mut shiny_gold_bags: usize = 0;
    let mut bags_vec: Vec<String> = Vec::new();
    // push the initial bags into the list
    bags_vec.push(String::from("shiny gold"));

    // while there are till bags to go through
    while !bags_vec.is_empty() {
        // save a copy of the bags vec for iteration purposes
        let temp_vec = bags_vec.clone();
        println!("Bags Vec: {:?}", bags_vec);
        // clear the bags vec to prepare for the new bags
        bags_vec.clear();
        for bag in temp_vec.iter() {
            for line in input.iter() {
                if line.starts_with(bag) {
                    let new_bags: String = line[line.chars().find(|x| x.is_digit(10))..line.find("bag").unwrap() - 1].parse().unwrap();
                    let mut new_bags_vec: Vec<String> = new_bags.chars().split(',');
                    bags_vec = new_bags_vec;
                }
            }
        }
    }

    shiny_gold_bags
}

pub fn get_solution_pt_1() -> usize {
    let input = get_input();
    find_shiny_gold(input)
}

pub fn get_solution_pt_2() -> usize {
    let input = get_input();
    shiny_gold_pt_2(input)
}