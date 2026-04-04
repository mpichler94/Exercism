use std::ops::RangeInclusive;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|y| {
            (0..garden[y].len())
                .map(|x| replace(garden, y, x))
                .collect()
        })
        .collect()
}

fn replace(garden: &[&str], y: usize, x: usize) -> char {
    if garden[y].as_bytes()[x] == b'*' {
        return '*';
    }

    let flowers = neighbor_rows(garden, y as i32)
    .flat_map(|new_y| neighbor_cols(garden[y], x as i32).map(move |new_x| (new_y, new_x)))
    .filter(|&(y, x)| garden[y as usize].as_bytes()[x as usize] == b'*')
    .count();

    if flowers == 0 {
        return ' ';
    }

    char::from_digit(flowers as u32, 10).unwrap()
}

fn neighbor_rows(garden: &[&str], row: i32) -> RangeInclusive<i32> {
    (row - 1).max(0) ..= (row + 1).min(garden.len() as i32 - 1)
}

fn neighbor_cols(garden_row: &str, col: i32) -> RangeInclusive<i32> {
    (col - 1).max(0) ..= (col + 1).min(garden_row.len() as i32 - 1)
}
