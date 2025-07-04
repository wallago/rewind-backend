pub fn get_next_available_position(mut used_positions: Vec<i32>) -> i32 {
    used_positions.sort_unstable();
    used_positions.dedup();
    let mut next_position = 0;

    for &pos in &used_positions {
        if pos == next_position {
            next_position += 1;
        } else if pos > next_position {
            break;
        }
    }

    next_position
}
