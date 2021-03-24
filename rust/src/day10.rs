fn get_input() -> Vec<i64> {
    let input: Vec<&'static str> = include_str!("./inputs/day10.txt").lines().collect();
    input.iter().map(|num| num.parse::<i64>().unwrap()).collect()
}

fn joltage_differences() -> i64 {
    let mut joltage_ratings = get_input();
    let mut outlet: i64 = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 1; // because of the highest rated adapter rule
    joltage_ratings.sort();

    for rating in joltage_ratings.iter() {
        let rating_value = *rating;
        if outlet + 1 == rating_value {
            diff_1 += 1;
        } else if outlet + 3 == rating_value {
            diff_3 += 1;
        }

        outlet = rating_value;
    }

    diff_1 * diff_3
}

pub fn get_solution_pt_1() -> i64 {
    joltage_differences()
}