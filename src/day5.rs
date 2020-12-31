use core::num::flt2dec::decode;

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

fn get_sid(seat_number: (Vec<i32>, Vec<i32>)) -> i32 {
    seat_number.0[0] * 8 as i32 + seat_number.1[0]
}

pub fn get_solution_pt_1() -> i32 {
    let mut max = 0;
    for line in get_input().iter() {
        let sid = get_sid(decode_seat_string(line));
        if sid > max {
            max = sid;
        }
    }
    max
}

pub fn get_solution_part_2() -> i32 {
    let mut sid_vec: Vec<i32> = Vec::new();
    for line in get_input().iter() {
        let sid = get_sid(decode_seat_string(line));
        sid_vec.push(sid);
    }

    sid_vec.sort();

    for sid in sid_vec.iter() {

    }
    10
}