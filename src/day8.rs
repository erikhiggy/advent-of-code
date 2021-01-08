use std::borrow::Borrow;

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

fn compute_program(program: Vec<&str>) -> i32 {
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

fn compute_program_pt_2(mut program: Vec<&str>) -> i32 {
    let mut visited_array: Vec<bool> = vec![false; program.len()];
    let mut acc_value: i32 = 0;
    let mut should_end: bool = false;
    let mut pc: usize = 0;

    let mut cloned_prog = program.clone();

    for (idx, instr) in program.iter().enumerate() {
        // for each jmp or nop, run the program swapping that jmp to a nop
        // or vice versa until the program successfully terminates
        let mut op: &str = tokenize(instr).0;
        let arg: &str = tokenize(instr).1;

        visited_array = vec![false; program.len()];
        acc_value = 0;
        should_end = false;
        pc = 0;

        // successful terminate flag
        let mut terminated_successfully = false;

        println!("instruction: {:?}", op);

        match op {
            "nop" => cloned_prog[idx] = format!("{}{}", op, arg),
            // we want to ignore acc ops
            "acc" => op = "acc",
            "jmp" => cloned_prog[idx] = format!("nop {}", arg).parse().unwrap(),
            _ => println!("Op {:?} is not valid!", op)
        }

        // println!("Converted instructions: {:?}", op);
        // println!("PC: {:?}", pc);
        // println!("Visited Array: {:?}", visited_array);

        while !should_end {
            let mut op: &str = tokenize(cloned_prog[pc]).0;
            let arg: &str = tokenize(cloned_prog[pc]).1;
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
                _ => {
                    println!("Program ended successfully!");
                    terminated_successfully = true;
                }
            }

            if terminated_successfully == true {
                break;
            }

            if visited_array[pc] == true {
                // if we visit a previously visited instruction
                should_end = true;
            }
        }
    }
    acc_value
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
    compute_program(program)
}

pub fn get_solution_pt_2() -> i32 {
    let program: Vec<&str> = get_input();
    compute_program_pt_2(program)
}