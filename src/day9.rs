use std::collections::VecDeque;

fn get_input() -> Vec<i64> {
    let input: Vec<&'static str> = include_str!("./inputs/day9.txt").lines().collect();
    input.iter().map(|num| num.parse::<i64>().unwrap()).collect()
}

fn sum_of_two(preamble: &VecDeque<i64>, target: i64) -> bool {
    let mut preamble_clone = preamble.clone();
    let mut preamble_to_vec = Vec::from(preamble_clone.as_slices().0);

    // order the numbers
    preamble_to_vec.sort();

    let mut i: usize = 0;
    let mut j: usize = preamble_to_vec.len() - 1;
    let mut is_sum = false;

    while !is_sum {
        // if the iterators equal each other, we did not find a sum
        if i == j {
            break;
        }
        // if the sum of i, j is more than the target
        if (preamble_to_vec[i] + preamble_to_vec[j]) > target {
            // move j to the next last number in the list
            j -= 1;
            // if the sum of i, j is less than the target
        } else if (preamble_to_vec[i] + preamble_to_vec[j]) < target {
            // move to the next number in the beginning of the list
            i += 1;
            // if the numbers are equal, we found a combo
        } else {
            is_sum = true;
        }
    }
    is_sum
}

pub fn get_solution_pt_1() -> i64 {
    let input = get_input();
    let preamble_len: usize = 25;
    let mut no_sum_value: i64 = 0;

    let mut preamble: VecDeque<i64> = VecDeque::new();

    for i in 0..preamble_len {
        preamble.push_back(input[i]);
    }

    // loop through everything from the initial preamble on
    for i in preamble_len..input.len() {
        // if there is a sum in the current preamble
        if sum_of_two(&preamble, input[i]) {
            // push back the new value and pop off the front
            preamble.push_back(input[i]);
            let _unused = preamble.pop_front();
            // if the preamble does not contain two numbers that sum to input[i]
        } else {
            no_sum_value = input[i];
            break;
        }
    }
    no_sum_value
}

#[cfg(test)]
#[test]
fn sum_of_two_test() {
    assert_eq!(sum_of_two(&VecDeque::from(vec![1, 3, 4, 6, 7]), 7), true);
    assert_eq!(sum_of_two(&VecDeque::from(vec![1, 3, 4, 6, 7]), 20), false)
}