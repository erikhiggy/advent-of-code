fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day5.txt").lines().collect()
}

fn decode_seat_string(seat_string: &str) -> (Vec<i32>, Vec<i32>) {
    let mut rows: Vec<i32> = Vec::new();
    let mut cols: Vec<i32> = Vec::new();
    for i in 0..=127 {
        rows.push(i);
        if i <= 7 {
            cols.push(i);
        }
    }

    for letter in seat_string.chars() {
        // 1. read letter from seat_string
        // 2. if the letter is F || R, remove letter from string
        //    and rerun code with upper half and new string
        // 3. if  the letter is B || L, remove letter from string
        //    and rerun code with lower half
        match letter {
            'B' => {
                rows = rows[(rows.len() / 2)..rows.len()].to_vec();
            },
            'F' => {
                rows = rows[0..=(rows.len() / 2)].to_vec();
            },
            'R' => {
                cols = cols[(cols.len() / 2)..cols.len()].to_vec();
            },
            'L' => {
                cols = cols[0..=(cols.len() / 2)].to_vec();
            },
            _ => ()
        }
    }
    (rows, cols)
}

pub fn get_solution_pt_1() -> i32 {
    let mut max = 0;
    for plane in get_input().iter() {
        let answer_tuple = decode_seat_string(plane);
        if answer_tuple.0[0] * 8 as i32 + answer_tuple.1[0] > max {
            max = answer_tuple.0[0] * 8 as i32 + answer_tuple.1[0];
        }
    }
    max
}