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

pub fn get_solution_pt_1() -> usize {
    let input = get_input();
    find_shiny_gold(input)
}