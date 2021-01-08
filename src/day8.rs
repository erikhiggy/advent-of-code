fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day8.txt").lines().collect()
}

fn tokenize(instruction: &str) -> (&str, &str) {
    (&instruction[..3], &instruction[4..])
}

fn accumulate(arg: &str) -> i32 {
    return if arg.chars().next() == Some('-') {
        -arg[1..].parse::<i32>().unwrap()
    } else {
        arg[1..].parse::<i32>().unwrap()
    }
}

#[cfg(test)]
#[test]
fn accumulate_positive_test() {
    assert_eq!(accumulate("+33"), 33)
}
#[test]
fn accumulate_negative_test() {
    assert_eq!(accumulate("-7"), -7)
}
#[test]
fn tokenize_positive_test() {
    assert_eq!(tokenize("acc +3"), ("acc", "+3"))
}
#[test]
fn tokenize_negative_test() {
    assert_eq!(tokenize("acc -3"), ("acc", "-3"))
}

pub fn get_solution_pt_1() -> i32 {
    let program: Vec<&str> = get_input();
    let mut visited_array: Vec<bool> = vec![false; program.len()];
    let mut acc_value: i32 = 0;
    let mut should_end: bool = false;
    let mut pc: usize = 0;

    while !should_end {
        let op: &str = tokenize(program[pc]).0;
        let arg: &str = tokenize(program[pc]).1;

        match op {
            "nop" => {
                // noop, do nothing and increment the pc
                visited_array[pc] = true;
                pc += 1;
            },
            "acc" => {
                // add value to the acc and increment the pc
                visited_array[pc] = true;
                acc_value = acc_value.wrapping_add(accumulate(arg));
                pc += 1;
            },
            "jmp" => {
                // jump to the instruction arg away from this one
                visited_array[pc] = true;
                pc = pc.wrapping_add(accumulate(arg) as usize);
            },
            _ => ()
        }

        if visited_array[pc] == true {
            should_end = true;
        }
    }
    acc_value
}