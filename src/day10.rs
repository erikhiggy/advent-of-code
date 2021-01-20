fn get_input() -> Vec<i64> {
    let input: Vec<&'static str> = include_str!("./inputs/day10.txt").lines().collect();
    input.iter().map(|num| num.parse::<i64>().unwrap()).collect()
}

fn joltage_differences() {
    let joltage_ratings = get_input();
}

pub fn get_solution_pt_1() -> Vec<i64> {
    let mut input = get_input();
    input.sort();
    input

}