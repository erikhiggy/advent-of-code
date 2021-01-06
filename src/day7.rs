use std::collections::{HashSet, HashMap};

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
    let mut bags_map: HashMap<String, usize> = HashMap::new();
    // push the initial bags into the map
    bags_map.insert(String::from("shiny gold"), 1);

    // while there are till bags to go through
    while !bags_map.is_empty() {
        // save a copy of the bags vec for iteration purposes
        let temp_map = bags_map.clone();
        // clear the bags vec to prepare for the new bags
        bags_map.clear();
        for (bag, _number) in temp_map.iter() {
            for line in input.iter() {
                // if we find the bag at the start, do some work then move to next bag
                if line.starts_with(bag) {
                    let child_bags: String = line[line.find("contain").unwrap() + 8..line.find('.').unwrap()].parse().unwrap();

                    // if there are no other bags, dont push the bag in and move on to next bag
                    // we are at a leaf
                    if child_bags == "no other bags" {
                        // println!("Leaves: {:?}", temp_map);
                        break;
                    }

                    for child_bag in child_bags.split(",") {
                        bags_map.insert(child_bag[child_bag.find(child_bag.split_whitespace().next().unwrap()).unwrap() + 2..].parse().unwrap(), child_bag.split_whitespace().next().unwrap().parse().unwrap());
                    }
                    println!("Bags Map: {:?}", bags_map);
                    break;
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