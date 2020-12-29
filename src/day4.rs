pub fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day4.txt").lines().collect()
}

pub fn parse_input(input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut parsed_input: Vec<Vec<&str>> = input
        .iter()
        .map(|item| item.split_whitespace().collect()).collect();

    parsed_input.iter().filter_map(|s: &Vec<&str>| Some(!s.is_empty())).collect()
}