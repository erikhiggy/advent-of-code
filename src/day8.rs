fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day8.txt").lines().collect()
}

fn tokenize(instruction: &str) -> (String, String) {
    let op: String = instruction[..3].parse().unwrap();
    let arg: String = instruction[4..].parse().unwrap();

    (op, arg)
}

pub fn get_solution_pt_1() -> usize {
    let input = get_input();
    let mut acc_value = 0;
    for instr in input.iter() {
        println!("Op: {:?}, Arg: {:?}", tokenize(instr).0, tokenize(instr).1);
    }
    acc_value
}