mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day1_solution_pt_1 = day1::solution_pt_1();
    let day1_solution_pt_2 = day1::solution_pt_2();

    println!("Day 1 Solution Part 1: {}", day1_solution_pt_1);
    println!("Day 1 Solution Part 2: {}", day1_solution_pt_2);

    let day2_solution_pt_1 = day2::get_solution_part1();
    let day2_solution_pt_2 = day2::get_solution_part2();

    println!("Day 2 Solution Part 1: {}", day2_solution_pt_1);
    println!("Day 2 Solution Part 2: {}", day2_solution_pt_2);

    let day3_solution_pt_1 = day3::get_solution_part1();
    let day3_solution_pt_2 = day3::get_solution_part2();

    println!("Day 3 Solution Part 1: {}", day3_solution_pt_1);
    println!("Day 3 Solution Part 2: {}", day3_solution_pt_2);

    let day4_solution_pt_1 = day4::get_solution_pt_1();

    println!("Day 4 Solution Part 1: {}", day4_solution_pt_1);
}
