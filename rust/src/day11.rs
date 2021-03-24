fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day11.txt").lines().collect()
}

fn occupied_adjacents(seat_matrix: &Vec<&str>, (row_idx, seat_idx): (usize, usize)) -> i32 {
    let left_of_seat = row_idx - 1;
}

pub fn get_solution_pt_1() -> i64 {
    let mut seat_matrix = get_input();

    let temp_matrix = seat_matrix.clone();

    for (row_idx, row) in seat_matrix.iter().enumerate() {
        for (seat_idx, mut seat) in row.chars().enumerate() {
            match seat {
                'L' => {
                    // if a seat is empty (L) and there are no occupied (#) seats
                    // adjacent to it, the seat becomes occupied
                    if occupied_adjacents(&seat_matrix, (row_idx, seat_idx)) == 0 {
                        seat = '#';
                    }
                },
                '#' => {
                    if occupied_adjacents(&seat_matrix, (row_idx, seat_idx)) >= 4 {
                        seat = 'L';
                    }
                },
                '.' => {
                    // do nothing
                    continue;
                },
                _ => ()
            }
        }
    }

    0
}