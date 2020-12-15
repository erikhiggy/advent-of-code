pub fn solution_pt_1() -> i64 {
    let entries = get_input();
    let sum_of_two = get_sum_of_two_entries(2020, &entries).unwrap();

    return (sum_of_two.0 * sum_of_two.1) as i64;
}

pub fn solution_pt_2() -> i64 {
    let entries = get_input();
    let sum_of_three = get_sum_of_three_entries(2020, &entries).unwrap();

    return (sum_of_three.0 * sum_of_three.1 * sum_of_three.2) as i64;
}

fn get_sum_of_two_entries(sum: i32, entries: &[i32]) -> Result<(i32, i32), Box<String>> {
    for (index0, entry0) in entries.iter().enumerate() {
        for (index1, entry1) in entries.iter().enumerate() {
            if index0 == index1 {
                continue;
            }

            if entry0 + entry1 == sum {
                return Ok((*entry0, *entry1));
            }
        }
    }

    let error = Box::new(format!("Could not find any two values which sum to '{}'", sum));
    return Err(error);
}

fn get_sum_of_three_entries(sum: i32, entries: &[i32]) -> Result<(i32, i32, i32), Box<String>> {
    for (index0, entry0) in entries.iter().enumerate() {
        for (index1, entry1) in entries.iter().enumerate() {
            if index0 == index1 {
                continue;
            }

            for (index2, entry2) in entries.iter().enumerate() {
                if index0 == index2 || index1 == index2 {
                    continue;
                }

                if entry0 + entry1 + entry2 == sum {
                    return Ok((*entry0, *entry1, *entry2));
                }
            }
        }
    }

    let error = Box::new(format!("Could not find any three values which sum to '{}'", sum));
    return Err(error);
}

fn get_input() -> Vec<i32> {
    let file_contents = include_str!("inputs/day1.txt").split_whitespace();
    let entries = file_contents
        .map(|entry| entry.trim().parse().expect("Failed to convert number to integer")).collect();

    return entries;
}
